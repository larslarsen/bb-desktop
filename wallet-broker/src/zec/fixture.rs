use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};

use serde::Deserialize;

use super::{MAX_FIXTURE_MANIFEST_BYTES, ZecError};

const FIXTURE_FORMAT: &str = "bitbook-zec-compact-fixture";
const FIXTURE_VERSION: u32 = 1;

pub(crate) struct FrozenFixture {
    manifest: FixtureManifest,
}

impl FrozenFixture {
    pub(crate) fn open(relative: &str) -> Result<Self, ZecError> {
        let relative = validate_relative_path(relative)?;
        let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let fixture_root = crate_root.join(relative);
        validate_path_chain(crate_root, relative)?;
        let metadata = fs::symlink_metadata(&fixture_root).map_err(|_| ZecError::schema())?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(ZecError::schema());
        }

        let manifest_path = fixture_root.join("manifest.json");
        let manifest_metadata =
            fs::symlink_metadata(&manifest_path).map_err(|_| ZecError::schema())?;
        if manifest_metadata.file_type().is_symlink()
            || !manifest_metadata.file_type().is_file()
            || manifest_metadata.len() == 0
        {
            return Err(ZecError::schema());
        }
        let length = usize::try_from(manifest_metadata.len()).map_err(|_| ZecError::limit())?;
        validate_manifest_length(length)?;
        let bytes = read_manifest_exact(&manifest_path, length)?;
        let manifest: FixtureManifest =
            serde_json::from_slice(&bytes).map_err(|_| ZecError::schema())?;
        manifest.validate()?;
        Ok(Self { manifest })
    }

    pub(crate) fn orchard_only_receiver(&self) -> &str {
        &self.manifest.expected.orchard_only_receiver
    }
}

fn read_manifest_exact(path: &Path, length: usize) -> Result<Vec<u8>, ZecError> {
    validate_manifest_length(length)?;
    let mut bytes = vec![0; length];
    let mut file = fs::File::open(path).map_err(|_| ZecError::schema())?;
    file.read_exact(&mut bytes)
        .map_err(|_| ZecError::schema())?;
    let mut extra = [0; 1];
    if file.read(&mut extra).map_err(|_| ZecError::schema())? != 0 {
        return Err(ZecError::schema());
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

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FixtureManifest {
    format: String,
    version: u32,
    generator: Generator,
    network: FixtureNetwork,
    expected: Expected,
    files: Vec<FileEntry>,
    scenarios: Scenarios,
}

impl FixtureManifest {
    fn validate(&self) -> Result<(), ZecError> {
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
            || self.network.overwinter == 0
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
            || self.scenarios.canonical.is_empty()
            || self.scenarios.replay.is_empty()
        {
            return Err(ZecError::schema());
        }

        let mut names = BTreeSet::new();
        for file in &self.files {
            if validate_relative_path(&file.name).is_err()
                || file.byte_length == 0
                || !is_lower_hex(&file.sha256, 64)
                || file.scenario_labels.is_empty()
                || file.scenario_labels.iter().any(|label| label.is_empty())
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

        let mut scenario_files = self
            .scenarios
            .canonical
            .iter()
            .chain(self.scenarios.replay.iter())
            .chain([
                &self.scenarios.discontinuity,
                &self.scenarios.height_gap,
                &self.scenarios.one_block_reorg,
                &self.scenarios.truncation,
                &self.scenarios.malformed,
                &self.scenarios.corruption,
                &self.scenarios.impossible_tree_state,
            ]);
        if scenario_files
            .any(|name| validate_relative_path(name).is_err() || !names.contains(name.as_str()))
        {
            return Err(ZecError::schema());
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

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Generator {
    zcash_client_backend: String,
    zcash_client_sqlite: String,
    pczt: String,
    zcash_primitives: String,
    zcash_protocol: String,
    zcash_keys: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FixtureNetwork {
    discriminator: String,
    birthday_height: u32,
    checkpoint_height: u32,
    overwinter: u32,
    sapling: u32,
    blossom: u32,
    heartwood: u32,
    canopy: u32,
    nu5: u32,
    nu6: u32,
    nu6_1: u32,
    nu6_2: u32,
    nu6_3: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Expected {
    orchard_only_receiver: String,
    orchard_migration_required_zat: u64,
    ironwood_spendable_zat: u64,
    reorg_victim_ironwood_pending_zat: u64,
    reorg_replacement_ironwood_pending_zat: u64,
    confirmation_height: u32,
    nu6_3_branch_id_hex: String,
    prepared_transaction_version: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FileEntry {
    name: String,
    byte_length: u64,
    sha256: String,
    block_height: Option<u32>,
    block_hash: Option<String>,
    previous_hash: Option<String>,
    scenario_labels: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Scenarios {
    canonical: Vec<String>,
    replay: Vec<String>,
    discontinuity: String,
    height_gap: String,
    one_block_reorg: String,
    truncation: String,
    malformed: String,
    corruption: String,
    impossible_tree_state: String,
}
