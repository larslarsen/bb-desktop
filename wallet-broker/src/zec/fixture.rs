use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};

use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use sha2::{Digest, Sha256};

use super::{MAX_COMPACT_BLOCK_BYTES, MAX_FIXTURE_MANIFEST_BYTES, ZecError};

const FIXTURE_FORMAT: &str = "bitbook-zec-compact-fixture";
const FIXTURE_VERSION: u32 = 1;
const MAX_FIXTURE_FILES: usize = 64;
const MAX_FIXTURE_BYTES: usize = 64 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ManifestMutation {
    UnknownField,
    DuplicateEntry,
    PathTraversal,
    AbsolutePath,
    WrongLength,
    WrongSha256,
    WrongNetwork,
    UnsupportedVersion,
    DuplicateJsonKey,
}

#[derive(Clone)]
pub(crate) struct FrozenFixture {
    root: PathBuf,
    raw_manifest: Vec<u8>,
    pub(crate) manifest: FixtureManifest,
    mutation: Option<ManifestMutation>,
}

#[derive(Clone)]
pub(crate) struct ValidatedFixture {
    pub(crate) root: PathBuf,
    pub(crate) manifest: FixtureManifest,
}

#[derive(Clone)]
pub(crate) struct ValidatedBlock {
    pub(crate) height_hint: Option<u32>,
    pub(crate) hash_hint: Option<String>,
    pub(crate) previous_hash_hint: Option<String>,
    pub(crate) bytes: Vec<u8>,
}

impl FrozenFixture {
    pub(crate) fn open(relative: &str) -> Result<Self, ZecError> {
        let relative = validate_relative_path(relative)?;
        let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = crate_root.join(relative);
        validate_path_chain(crate_root, relative)?;
        validate_directory(&root)?;
        let manifest_path = root.join("manifest.json");
        validate_regular_file(&manifest_path)?;
        let length = usize::try_from(
            fs::symlink_metadata(&manifest_path)
                .map_err(|_| ZecError::schema())?
                .len(),
        )
        .map_err(|_| ZecError::limit())?;
        validate_manifest_length(length)?;
        if length == 0 {
            return Err(ZecError::schema());
        }
        let raw_manifest = read_exact(&manifest_path, length, MAX_FIXTURE_MANIFEST_BYTES)?;
        reject_duplicate_json_keys(&raw_manifest)?;
        let manifest: FixtureManifest =
            serde_json::from_slice(&raw_manifest).map_err(|_| ZecError::schema())?;
        manifest.validate_structure()?;
        Ok(Self {
            root,
            raw_manifest,
            manifest,
            mutation: None,
        })
    }

    pub(crate) fn orchard_only_receiver(&self) -> &str {
        &self.manifest.expected.orchard_only_receiver
    }

    pub(crate) fn mutated(&self, mutation: ManifestMutation) -> Self {
        let mut fixture = self.clone();
        fixture.mutation = Some(mutation);
        fixture
    }

    pub(crate) fn validate_complete(&self) -> Result<ValidatedFixture, ZecError> {
        reject_duplicate_json_keys(&self.raw_manifest)?;
        let mut manifest = self.manifest.clone();
        if let Some(mutation) = self.mutation {
            match mutation {
                ManifestMutation::UnknownField | ManifestMutation::DuplicateJsonKey => {
                    return Err(ZecError::schema());
                }
                ManifestMutation::DuplicateEntry => {
                    let duplicate = manifest
                        .files
                        .first()
                        .cloned()
                        .ok_or_else(ZecError::schema)?;
                    manifest.files.push(duplicate);
                }
                ManifestMutation::PathTraversal => {
                    manifest.files[0].name = "../escape.compact".to_owned();
                }
                ManifestMutation::AbsolutePath => {
                    manifest.files[0].name = "/escape.compact".to_owned();
                }
                ManifestMutation::WrongLength => {
                    manifest.files[0].byte_length = manifest.files[0].byte_length.saturating_add(1);
                }
                ManifestMutation::WrongSha256 => manifest.files[0].sha256 = "0".repeat(64),
                ManifestMutation::WrongNetwork => {
                    manifest.network.discriminator = "zec-testnet".to_owned();
                }
                ManifestMutation::UnsupportedVersion => {
                    manifest.version = manifest.version.saturating_add(1);
                }
            }
        }
        manifest.validate_structure()?;
        validate_fixture_files(&self.root, &manifest)?;
        Ok(ValidatedFixture {
            root: self.root.clone(),
            manifest,
        })
    }
}

impl ValidatedFixture {
    pub(crate) fn file(&self, name: &str) -> Result<ValidatedBlock, ZecError> {
        let entry = self
            .manifest
            .files
            .iter()
            .find(|entry| entry.name == name)
            .ok_or_else(ZecError::schema)?;
        let length = usize::try_from(entry.byte_length).map_err(|_| ZecError::limit())?;
        let bytes = read_exact(
            &self.root.join(validate_relative_path(&entry.name)?),
            length,
            MAX_COMPACT_BLOCK_BYTES,
        )?;
        if sha256_hex(&bytes) != entry.sha256 {
            return Err(ZecError::state_corrupt());
        }
        Ok(ValidatedBlock {
            height_hint: entry.block_height,
            hash_hint: entry.block_hash.clone(),
            previous_hash_hint: entry.previous_hash.clone(),
            bytes,
        })
    }

    pub(crate) fn canonical_through(&self, height: u32) -> Result<Vec<ValidatedBlock>, ZecError> {
        let mut blocks = Vec::new();
        for name in &self.manifest.scenarios.canonical {
            let block = self.file(name)?;
            let block_height = block.height_hint.ok_or_else(ZecError::schema)?;
            if block_height <= height {
                blocks.push(block);
            }
        }
        if blocks.is_empty() || blocks.last().and_then(|block| block.height_hint) != Some(height) {
            return Err(ZecError::schema());
        }
        Ok(blocks)
    }

    pub(crate) fn scenario_file(&self, scenario: &str) -> Result<ValidatedBlock, ZecError> {
        let name = match scenario {
            "truncated" => &self.manifest.scenarios.truncation,
            "malformed" => &self.manifest.scenarios.malformed,
            "wrong-previous-hash" => &self.manifest.scenarios.discontinuity,
            "height-gap" => &self.manifest.scenarios.height_gap,
            "one-block-reorg" => &self.manifest.scenarios.one_block_reorg,
            "impossible-tree-state" => &self.manifest.scenarios.impossible_tree_state,
            "corruption" => &self.manifest.scenarios.corruption,
            _ => return Err(ZecError::schema()),
        };
        self.file(name)
    }
}

fn validate_fixture_files(root: &Path, manifest: &FixtureManifest) -> Result<(), ZecError> {
    let mut total = 0usize;
    for entry in &manifest.files {
        let relative = validate_relative_path(&entry.name)?;
        validate_path_chain(root, relative)?;
        let path = root.join(relative);
        validate_regular_file(&path)?;
        if fs::symlink_metadata(&path)
            .map_err(|_| ZecError::state_corrupt())?
            .len()
            != entry.byte_length
        {
            return Err(ZecError::state_corrupt());
        }
        let length = usize::try_from(entry.byte_length).map_err(|_| ZecError::limit())?;
        if length == 0 || length > MAX_COMPACT_BLOCK_BYTES {
            return Err(ZecError::limit());
        }
        total = total.checked_add(length).ok_or_else(ZecError::limit)?;
        if total > MAX_FIXTURE_BYTES {
            return Err(ZecError::limit());
        }
        if sha256_hex(&read_exact(&path, length, MAX_COMPACT_BLOCK_BYTES)?) != entry.sha256 {
            return Err(ZecError::state_corrupt());
        }
    }
    Ok(())
}

fn read_exact(path: &Path, length: usize, limit: usize) -> Result<Vec<u8>, ZecError> {
    if length > limit {
        return Err(ZecError::limit());
    }
    let mut bytes = vec![0; length];
    let mut file = fs::File::open(path).map_err(|_| ZecError::state_corrupt())?;
    file.read_exact(&mut bytes)
        .map_err(|_| ZecError::state_corrupt())?;
    let mut extra = [0; 1];
    if file
        .read(&mut extra)
        .map_err(|_| ZecError::state_corrupt())?
        != 0
    {
        return Err(ZecError::state_corrupt());
    }
    Ok(bytes)
}

pub(crate) fn validate_manifest_length(length: usize) -> Result<(), ZecError> {
    if length > MAX_FIXTURE_MANIFEST_BYTES {
        Err(ZecError::limit())
    } else {
        Ok(())
    }
}

pub(crate) fn allocate_manifest_sized(length: usize) -> Result<Vec<u8>, ZecError> {
    validate_manifest_length(length)?;
    Ok(vec![0; length])
}

fn validate_relative_path(value: &str) -> Result<&Path, ZecError> {
    let path = Path::new(value);
    if value.is_empty()
        || path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(ZecError::schema());
    }
    Ok(path)
}

fn validate_path_chain(base: &Path, relative: &Path) -> Result<(), ZecError> {
    let mut current = PathBuf::from(base);
    for component in relative.components() {
        let Component::Normal(component) = component else {
            return Err(ZecError::schema());
        };
        current.push(component);
        let metadata = fs::symlink_metadata(&current).map_err(|_| ZecError::schema())?;
        if metadata.file_type().is_symlink() {
            return Err(ZecError::schema());
        }
    }
    Ok(())
}

fn validate_directory(path: &Path) -> Result<(), ZecError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| ZecError::schema())?;
    if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
        Err(ZecError::schema())
    } else {
        Ok(())
    }
}

fn validate_regular_file(path: &Path) -> Result<(), ZecError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| ZecError::schema())?;
    if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
        Err(ZecError::schema())
    } else {
        Ok(())
    }
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FixtureManifest {
    pub(crate) format: String,
    pub(crate) version: u32,
    pub(crate) generator: Generator,
    pub(crate) network: FixtureNetwork,
    pub(crate) expected: Expected,
    pub(crate) files: Vec<FileEntry>,
    pub(crate) scenarios: Scenarios,
}

impl FixtureManifest {
    fn validate_structure(&self) -> Result<(), ZecError> {
        if self.format != FIXTURE_FORMAT
            || self.version != FIXTURE_VERSION
            || self.expected.orchard_only_receiver.is_empty()
            || self.generator.zcash_client_backend != "0.24.0"
            || self.generator.zcash_client_sqlite != "0.22.0"
            || self.generator.pczt != "0.9.3"
            || self.generator.zcash_primitives != "0.30.1"
            || self.generator.zcash_protocol != "0.10.5"
            || self.generator.zcash_keys != "0.16.1"
            || self.network.discriminator != "zec-local"
            || self.network.birthday_height == 0
            || self.network.checkpoint_height.checked_add(1) != Some(self.network.birthday_height)
            || self.network.overwinter != 1
            || [
                self.network.sapling,
                self.network.blossom,
                self.network.heartwood,
                self.network.canopy,
                self.network.nu5,
                self.network.nu6,
                self.network.nu6_1,
                self.network.nu6_2,
            ]
            .iter()
            .any(|height| *height != self.network.birthday_height)
            || self.network.nu6_3 < self.network.birthday_height
            || self.expected.orchard_migration_required_zat == 0
            || self.expected.ironwood_spendable_zat == 0
            || self.expected.reorg_victim_ironwood_pending_zat == 0
            || self.expected.reorg_replacement_ironwood_pending_zat == 0
            || self.expected.confirmation_height < self.network.nu6_3
            || !is_lower_hex(&self.expected.nu6_3_branch_id_hex, 8)
            || self.expected.prepared_transaction_version == 0
            || self.files.is_empty()
            || self.files.len() > MAX_FIXTURE_FILES
            || self.scenarios.canonical.is_empty()
            || self.scenarios.canonical != self.scenarios.replay
        {
            return Err(ZecError::schema());
        }

        let mut names = BTreeSet::new();
        for file in &self.files {
            if validate_relative_path(&file.name).is_err()
                || file.byte_length == 0
                || file.byte_length > MAX_COMPACT_BLOCK_BYTES as u64
                || !is_lower_hex(&file.sha256, 64)
                || file.scenario_labels.is_empty()
                || file.scenario_labels.iter().any(|label| label.is_empty())
                || file.scenario_labels.iter().collect::<BTreeSet<_>>().len()
                    != file.scenario_labels.len()
                || !names.insert(file.name.as_str())
            {
                return Err(ZecError::schema());
            }
            match (
                file.block_height,
                file.block_hash.as_deref(),
                file.previous_hash.as_deref(),
            ) {
                (Some(height), Some(hash), Some(previous))
                    if height >= self.network.birthday_height
                        && is_lower_hex(hash, 64)
                        && is_lower_hex(previous, 64) => {}
                (None, None, None) => {}
                _ => return Err(ZecError::schema()),
            }
        }

        let mut referenced = BTreeSet::new();
        let mut previous_height: Option<u32> = None;
        let mut previous_hash: Option<&str> = None;
        for name in &self.scenarios.canonical {
            if !referenced.insert(name.as_str()) {
                return Err(ZecError::schema());
            }
            let file = self
                .files
                .iter()
                .find(|file| &file.name == name)
                .ok_or_else(ZecError::schema)?;
            let height = file.block_height.ok_or_else(ZecError::schema)?;
            if previous_height.map_or(height != self.network.birthday_height, |previous| {
                previous.checked_add(1) != Some(height)
            }) {
                return Err(ZecError::schema());
            }
            if previous_hash.is_some_and(|hash| file.previous_hash.as_deref() != Some(hash)) {
                return Err(ZecError::schema());
            }
            previous_height = Some(height);
            previous_hash = file.block_hash.as_deref();
        }
        for name in [
            &self.scenarios.discontinuity,
            &self.scenarios.height_gap,
            &self.scenarios.one_block_reorg,
            &self.scenarios.truncation,
            &self.scenarios.malformed,
            &self.scenarios.corruption,
            &self.scenarios.impossible_tree_state,
        ] {
            if validate_relative_path(name).is_err()
                || !names.contains(name.as_str())
                || !referenced.insert(name.as_str())
            {
                return Err(ZecError::schema());
            }
        }
        if referenced != names {
            return Err(ZecError::schema());
        }
        for (name, label) in [
            (&self.scenarios.discontinuity, "discontinuity"),
            (&self.scenarios.height_gap, "height-gap"),
            (&self.scenarios.one_block_reorg, "one-block-reorg"),
            (&self.scenarios.truncation, "truncation"),
            (&self.scenarios.malformed, "malformed"),
            (&self.scenarios.corruption, "corruption"),
            (
                &self.scenarios.impossible_tree_state,
                "impossible-tree-state",
            ),
        ] {
            let entry = self
                .files
                .iter()
                .find(|file| &file.name == name)
                .ok_or_else(ZecError::schema)?;
            if !entry
                .scenario_labels
                .iter()
                .any(|candidate| candidate == label)
            {
                return Err(ZecError::schema());
            }
        }
        Ok(())
    }
}

fn is_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let digest = Sha256::digest(bytes);
    let mut result = String::with_capacity(64);
    for byte in digest {
        result.push(char::from(DIGITS[usize::from(byte >> 4)]));
        result.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    result
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Generator {
    pub(crate) zcash_client_backend: String,
    pub(crate) zcash_client_sqlite: String,
    pub(crate) pczt: String,
    pub(crate) zcash_primitives: String,
    pub(crate) zcash_protocol: String,
    pub(crate) zcash_keys: String,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FixtureNetwork {
    pub(crate) discriminator: String,
    pub(crate) birthday_height: u32,
    pub(crate) checkpoint_height: u32,
    pub(crate) overwinter: u32,
    pub(crate) sapling: u32,
    pub(crate) blossom: u32,
    pub(crate) heartwood: u32,
    pub(crate) canopy: u32,
    pub(crate) nu5: u32,
    pub(crate) nu6: u32,
    pub(crate) nu6_1: u32,
    pub(crate) nu6_2: u32,
    pub(crate) nu6_3: u32,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Expected {
    pub(crate) orchard_only_receiver: String,
    pub(crate) orchard_migration_required_zat: u64,
    pub(crate) ironwood_spendable_zat: u64,
    pub(crate) reorg_victim_ironwood_pending_zat: u64,
    pub(crate) reorg_replacement_ironwood_pending_zat: u64,
    pub(crate) confirmation_height: u32,
    pub(crate) nu6_3_branch_id_hex: String,
    pub(crate) prepared_transaction_version: u32,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FileEntry {
    pub(crate) name: String,
    pub(crate) byte_length: u64,
    pub(crate) sha256: String,
    pub(crate) block_height: Option<u32>,
    pub(crate) block_hash: Option<String>,
    pub(crate) previous_hash: Option<String>,
    pub(crate) scenario_labels: Vec<String>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Scenarios {
    pub(crate) canonical: Vec<String>,
    pub(crate) replay: Vec<String>,
    pub(crate) discontinuity: String,
    pub(crate) height_gap: String,
    pub(crate) one_block_reorg: String,
    pub(crate) truncation: String,
    pub(crate) malformed: String,
    pub(crate) corruption: String,
    pub(crate) impossible_tree_state: String,
}

fn reject_duplicate_json_keys(bytes: &[u8]) -> Result<(), ZecError> {
    serde_json::from_slice::<DuplicateChecked>(bytes)
        .map(|_| ())
        .map_err(|_| ZecError::schema())
}

struct DuplicateChecked;

impl<'de> Deserialize<'de> for DuplicateChecked {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(DuplicateVisitor)
    }
}

struct DuplicateVisitor;

impl<'de> Visitor<'de> for DuplicateVisitor {
    type Value = DuplicateChecked;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a JSON value without duplicate object keys")
    }

    fn visit_bool<E: de::Error>(self, _: bool) -> Result<Self::Value, E> {
        Ok(DuplicateChecked)
    }

    fn visit_i64<E: de::Error>(self, _: i64) -> Result<Self::Value, E> {
        Ok(DuplicateChecked)
    }

    fn visit_u64<E: de::Error>(self, _: u64) -> Result<Self::Value, E> {
        Ok(DuplicateChecked)
    }

    fn visit_f64<E: de::Error>(self, _: f64) -> Result<Self::Value, E> {
        Ok(DuplicateChecked)
    }

    fn visit_str<E: de::Error>(self, _: &str) -> Result<Self::Value, E> {
        Ok(DuplicateChecked)
    }

    fn visit_string<E: de::Error>(self, _: String) -> Result<Self::Value, E> {
        Ok(DuplicateChecked)
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(DuplicateChecked)
    }

    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(DuplicateChecked)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        DuplicateChecked::deserialize(deserializer)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Self::Value, A::Error> {
        while sequence.next_element::<DuplicateChecked>()?.is_some() {}
        Ok(DuplicateChecked)
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut keys = BTreeSet::new();
        while let Some(key) = map.next_key::<String>()? {
            if !keys.insert(key) {
                return Err(de::Error::custom("duplicate JSON object key"));
            }
            map.next_value::<DuplicateChecked>()?;
        }
        Ok(DuplicateChecked)
    }
}
