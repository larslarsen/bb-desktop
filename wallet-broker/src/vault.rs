use core::fmt;

use argon2::{Algorithm, Argon2, Params, Version};
use base64ct::{Base64Unpadded, Encoding};
use chacha20poly1305::aead::{AeadInPlace, KeyInit};
use chacha20poly1305::{Tag, XChaCha20Poly1305, XNonce};
use hkdf::Hkdf;
use secrecy::{ExposeSecret, ExposeSecretMut, SecretSlice};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use zeroize::Zeroize;

pub const VAULT_FORMAT: &str = "bitbook-wallet-vault";
pub const VAULT_VERSION: u8 = 1;
pub const ARGON2_VERSION: u32 = 19;
pub const ARGON2_M_COST_KIB: u32 = 65_536;
pub const ARGON2_T_COST: u32 = 3;
pub const ARGON2_P_COST: u32 = 1;
pub const AEAD_ALGORITHM: &str = "xchacha20poly1305";
pub const MAX_PASSPHRASE_BYTES: usize = 1_024;
pub const MAX_PLAINTEXT_BYTES: usize = 65_536;
pub const MAX_ENVELOPE_BYTES: usize = 128 * 1_024;

const KDF_ALGORITHM: &str = "argon2id";
const INFO_DOMAIN: &[u8] = b"BitBook wallet vault key v1";
const KEY_BYTES: usize = 32;
const SALT_BYTES: usize = 16;
const NONCE_BYTES: usize = 24;
const TAG_BYTES: usize = 16;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Asset {
    Zec,
    Xmr,
}

impl Asset {
    fn text(self) -> &'static str {
        match self {
            Self::Zec => "ZEC",
            Self::Xmr => "XMR",
        }
    }

    fn parse(value: &str) -> Result<Self, VaultError> {
        match value {
            "ZEC" => Ok(Self::Zec),
            "XMR" => Ok(Self::Xmr),
            _ => Err(VaultError::schema()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Network {
    ZecTestnet,
    ZecRegtest,
    XmrStagenet,
}

impl Network {
    fn text(self) -> &'static str {
        match self {
            Self::ZecTestnet => "zec-testnet",
            Self::ZecRegtest => "zec-regtest",
            Self::XmrStagenet => "xmr-stagenet",
        }
    }

    fn parse(value: &str) -> Result<Self, VaultError> {
        match value {
            "zec-testnet" => Ok(Self::ZecTestnet),
            "zec-regtest" => Ok(Self::ZecRegtest),
            "xmr-stagenet" => Ok(Self::XmrStagenet),
            _ => Err(VaultError::schema()),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct VaultError {
    code: &'static str,
    message: &'static str,
}

impl VaultError {
    fn new(code: &'static str, message: &'static str) -> Self {
        Self { code, message }
    }

    pub fn entropy() -> Self {
        Self::new("ENTROPY", "Wallet unavailable")
    }

    pub fn locked() -> Self {
        Self::new("LOCKED", "Wallet locked")
    }

    pub(crate) fn schema() -> Self {
        Self::new("SCHEMA", "Wallet data is invalid")
    }

    pub(crate) fn limit() -> Self {
        Self::new("LIMIT", "Wallet data exceeds its limit")
    }

    fn wrong_network() -> Self {
        Self::new("WRONG_NETWORK", "Wallet network mismatch")
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn public_message(&self) -> &'static str {
        self.message
    }
}

impl fmt::Debug for VaultError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VaultError")
            .field("code", &self.code)
            .finish()
    }
}

impl fmt::Display for VaultError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message)
    }
}

impl std::error::Error for VaultError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WipeEvent {
    pub label: &'static str,
    pub length: usize,
    pub all_zero: bool,
}

impl WipeEvent {
    pub fn field_names() -> [&'static str; 3] {
        ["label", "length", "all_zero"]
    }
}

pub trait WipeObserver {
    fn observe(&mut self, event: WipeEvent);
}

pub trait EntropyPort {
    fn fill(&mut self, label: &'static str, output: &mut [u8]) -> Result<(), VaultError>;
}

#[derive(Default)]
pub struct OsEntropy;

impl EntropyPort for OsEntropy {
    fn fill(&mut self, _label: &'static str, output: &mut [u8]) -> Result<(), VaultError> {
        getrandom::fill(output).map_err(|_| VaultError::entropy())
    }
}

pub trait VaultWorkObserver {
    fn before_allocation(&mut self, bytes: usize) -> Result<(), VaultError>;
    fn before_kdf(&mut self);
}

pub struct SecretBytes {
    bytes: SecretSlice<u8>,
    drop_observer: Option<(&'static str, Box<dyn WipeObserver>)>,
}

impl SecretBytes {
    pub fn new(bytes: Vec<u8>) -> Result<Self, VaultError> {
        Ok(Self {
            bytes: SecretSlice::new(bytes.into_boxed_slice()),
            drop_observer: None,
        })
    }

    pub fn new_observed(
        bytes: Vec<u8>,
        label: &'static str,
        observer: Box<dyn WipeObserver>,
    ) -> Result<Self, VaultError> {
        Ok(Self {
            bytes: SecretSlice::new(bytes.into_boxed_slice()),
            drop_observer: Some((label, observer)),
        })
    }

    pub fn len(&self) -> usize {
        self.bytes.expose_secret().len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.expose_secret().is_empty()
    }

    pub fn expose<T>(&self, expose: impl FnOnce(&[u8]) -> T) -> T {
        expose(self.bytes.expose_secret())
    }

    pub fn replace(
        &mut self,
        replacement: Vec<u8>,
        label: &'static str,
        observer: &mut dyn WipeObserver,
    ) -> Result<(), VaultError> {
        self.wipe_with(label, observer);
        self.bytes = SecretSlice::new(replacement.into_boxed_slice());
        Ok(())
    }

    pub(crate) fn wipe_with(&mut self, label: &'static str, observer: &mut dyn WipeObserver) {
        let mut secret = core::mem::replace(
            &mut self.bytes,
            SecretSlice::new(Vec::new().into_boxed_slice()),
        );
        let bytes = secret.expose_secret_mut();
        let length = bytes.len();
        bytes.zeroize();
        observer.observe(WipeEvent {
            label,
            length,
            all_zero: bytes.iter().all(|byte| *byte == 0),
        });
    }
}

impl fmt::Debug for SecretBytes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SecretBytes([REDACTED])")
    }
}

impl fmt::Display for SecretBytes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

impl Drop for SecretBytes {
    fn drop(&mut self) {
        let bytes = self.bytes.expose_secret_mut();
        let length = bytes.len();
        bytes.zeroize();
        let all_zero = bytes.iter().all(|byte| *byte == 0);
        if let Some((label, mut observer)) = self.drop_observer.take() {
            observer.observe(WipeEvent {
                label,
                length,
                all_zero,
            });
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultMetadata {
    account_id: [u8; 16],
    asset: Asset,
    network: Network,
    epoch: u64,
}

impl VaultMetadata {
    pub fn new(
        account_id: [u8; 16],
        asset: Asset,
        network: Network,
        epoch: u64,
    ) -> Result<Self, VaultError> {
        if epoch == 0 {
            return Err(VaultError::schema());
        }
        validate_asset_network(asset, network)?;
        Ok(Self {
            account_id,
            asset,
            network,
            epoch,
        })
    }

    pub fn account_id_hex(&self) -> String {
        hex_encode(&self.account_id)
    }

    pub fn asset(&self) -> Asset {
        self.asset
    }

    pub fn network(&self) -> Network {
        self.network
    }

    pub fn epoch(&self) -> u64 {
        self.epoch
    }
}

#[derive(Debug)]
pub struct VaultInputs {
    pub passphrase: SecretBytes,
    pub plaintext: SecretBytes,
}

impl VaultInputs {
    pub fn new(passphrase: SecretBytes, plaintext: SecretBytes) -> Result<Self, VaultError> {
        let passphrase_valid = !passphrase.is_empty()
            && passphrase.len() <= MAX_PASSPHRASE_BYTES
            && passphrase.expose(|bytes| core::str::from_utf8(bytes).is_ok());
        if !passphrase_valid || plaintext.is_empty() || plaintext.len() > MAX_PLAINTEXT_BYTES {
            return Err(VaultError::limit());
        }
        Ok(Self {
            passphrase,
            plaintext,
        })
    }
}

#[derive(Clone)]
pub struct VaultEnvelope {
    metadata: VaultMetadata,
    salt: [u8; SALT_BYTES],
    nonce: [u8; NONCE_BYTES],
    ciphertext: Vec<u8>,
    bytes: Vec<u8>,
}

impl fmt::Debug for VaultEnvelope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VaultEnvelope")
            .field("metadata", &self.metadata)
            .field("ciphertext_length", &self.ciphertext.len())
            .finish()
    }
}

impl VaultEnvelope {
    pub fn metadata(&self) -> &VaultMetadata {
        &self.metadata
    }

    pub fn salt(&self) -> &[u8] {
        &self.salt
    }

    pub fn nonce(&self) -> &[u8] {
        &self.nonce
    }

    pub fn ciphertext(&self) -> &[u8] {
        &self.ciphertext
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireEnvelope {
    format: String,
    version: u8,
    account_id: String,
    asset: String,
    network: String,
    epoch: String,
    kdf: WireKdf,
    aead: WireAead,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireKdf {
    algorithm: String,
    version: u32,
    m_cost_kib: u32,
    t_cost: u32,
    p_cost: u32,
    salt_b64: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireAead {
    algorithm: String,
    nonce_b64: String,
    ciphertext_b64: String,
}

pub fn parse_vault(
    bytes: &[u8],
    work_observer: &mut dyn VaultWorkObserver,
) -> Result<VaultEnvelope, VaultError> {
    if bytes.len() > MAX_ENVELOPE_BYTES {
        return Err(VaultError::limit());
    }
    work_observer.before_allocation(bytes.len())?;
    let wire: WireEnvelope = serde_json::from_slice(bytes).map_err(|_| VaultError::schema())?;
    validate_wire_profile(&wire)?;

    let account_id = parse_account_id(&wire.account_id)?;
    let asset = Asset::parse(&wire.asset)?;
    let network = Network::parse(&wire.network)?;
    let epoch = parse_epoch(&wire.epoch)?;
    let metadata = VaultMetadata::new(account_id, asset, network, epoch).map_err(|error| {
        if error.code() == "WRONG_NETWORK" {
            VaultError::schema()
        } else {
            error
        }
    })?;
    let salt = fixed_bytes::<SALT_BYTES>(
        &Base64Unpadded::decode_vec(&wire.kdf.salt_b64).map_err(|_| VaultError::schema())?,
    )?;
    let nonce = fixed_bytes::<NONCE_BYTES>(
        &Base64Unpadded::decode_vec(&wire.aead.nonce_b64).map_err(|_| VaultError::schema())?,
    )?;
    let ciphertext =
        Base64Unpadded::decode_vec(&wire.aead.ciphertext_b64).map_err(|_| VaultError::schema())?;
    if !(TAG_BYTES + 1..=MAX_PLAINTEXT_BYTES + TAG_BYTES).contains(&ciphertext.len()) {
        return Err(VaultError::schema());
    }

    let canonical = serialize_wire(&wire)?;
    if canonical != bytes {
        return Err(VaultError::schema());
    }
    Ok(VaultEnvelope {
        metadata,
        salt,
        nonce,
        ciphertext,
        bytes: canonical,
    })
}

pub fn seal_vault(
    metadata: &VaultMetadata,
    passphrase: &mut SecretBytes,
    plaintext: &mut SecretBytes,
    entropy: &mut dyn EntropyPort,
    wipe_observer: &mut dyn WipeObserver,
) -> Result<VaultEnvelope, VaultError> {
    if let Err(error) = validate_secret_bounds(passphrase, plaintext) {
        passphrase.wipe_with("passphrase", wipe_observer);
        plaintext.wipe_with("plaintext", wipe_observer);
        return Err(error);
    }
    let mut salt = [0u8; SALT_BYTES];
    let mut nonce = [0u8; NONCE_BYTES];
    let mut argon_output = [0u8; KEY_BYTES];
    let mut key = [0u8; KEY_BYTES];
    let mut plaintext_scratch = Vec::new();

    let result = (|| {
        entropy.fill("vault-salt", &mut salt)?;
        entropy.fill("vault-nonce", &mut nonce)?;
        derive_argon(passphrase, &salt, &mut argon_output)?;
        derive_key(&metadata_hkdf_info(metadata), &argon_output, &mut key)?;
        let aad = vault_aad(metadata, &salt, &nonce);
        plaintext_scratch = plaintext.expose(|bytes| bytes.to_vec());
        let cipher = XChaCha20Poly1305::new_from_slice(&key).map_err(|_| VaultError::locked())?;
        let cipher_nonce = XNonce::from_slice(nonce.as_slice());
        let tag = cipher
            .encrypt_in_place_detached(cipher_nonce, &aad, plaintext_scratch.as_mut_slice())
            .map_err(|_| VaultError::locked())?;
        let mut ciphertext = core::mem::take(&mut plaintext_scratch);
        ciphertext.extend_from_slice(tag.as_ref());
        envelope_from_parts(metadata.clone(), salt, nonce, ciphertext)
    })();

    passphrase.wipe_with("passphrase", wipe_observer);
    wipe_array(&mut argon_output, "argon2-output", wipe_observer);
    wipe_array(&mut key, "hkdf-output", wipe_observer);
    plaintext.wipe_with("plaintext", wipe_observer);
    plaintext_scratch.zeroize();
    result
}

pub fn open_vault_bytes(
    bytes: &[u8],
    passphrase: &mut SecretBytes,
    work_observer: &mut dyn VaultWorkObserver,
    wipe_observer: &mut dyn WipeObserver,
) -> Result<SecretBytes, VaultError> {
    if passphrase.is_empty()
        || passphrase.len() > MAX_PASSPHRASE_BYTES
        || !passphrase.expose(|value| core::str::from_utf8(value).is_ok())
    {
        passphrase.wipe_with("passphrase", wipe_observer);
        return Err(VaultError::locked());
    }
    let envelope = match parse_vault(bytes, work_observer) {
        Ok(envelope) => envelope,
        Err(error) => {
            passphrase.wipe_with("passphrase", wipe_observer);
            return Err(error);
        }
    };
    work_observer.before_kdf();
    let mut argon_output = [0u8; KEY_BYTES];
    let mut key = [0u8; KEY_BYTES];
    let mut plaintext = envelope.ciphertext.clone();
    let result = (|| {
        derive_argon(passphrase, &envelope.salt, &mut argon_output)?;
        derive_key(
            &metadata_hkdf_info(&envelope.metadata),
            &argon_output,
            &mut key,
        )?;
        let aad = vault_aad(&envelope.metadata, &envelope.salt, &envelope.nonce);
        let tag_at = plaintext
            .len()
            .checked_sub(TAG_BYTES)
            .ok_or_else(VaultError::locked)?;
        let tag_bytes = plaintext.split_off(tag_at);
        let cipher = XChaCha20Poly1305::new_from_slice(&key).map_err(|_| VaultError::locked())?;
        let cipher_nonce = XNonce::from_slice(envelope.nonce.as_slice());
        let tag = Tag::from_slice(tag_bytes.as_slice());
        cipher
            .decrypt_in_place_detached(cipher_nonce, &aad, plaintext.as_mut_slice(), tag)
            .map_err(|_| VaultError::locked())?;
        SecretBytes::new(core::mem::take(&mut plaintext))
    })();

    if result.is_err() {
        plaintext.zeroize();
    }
    passphrase.wipe_with("passphrase", wipe_observer);
    wipe_array(&mut argon_output, "argon2-output", wipe_observer);
    wipe_array(&mut key, "hkdf-output", wipe_observer);
    result.map_err(|_| VaultError::locked())
}

fn validate_secret_bounds(
    passphrase: &SecretBytes,
    plaintext: &SecretBytes,
) -> Result<(), VaultError> {
    if passphrase.is_empty()
        || passphrase.len() > MAX_PASSPHRASE_BYTES
        || !passphrase.expose(|value| core::str::from_utf8(value).is_ok())
        || plaintext.is_empty()
        || plaintext.len() > MAX_PLAINTEXT_BYTES
    {
        return Err(VaultError::limit());
    }
    Ok(())
}

fn derive_argon(
    passphrase: &SecretBytes,
    salt: &[u8; SALT_BYTES],
    output: &mut [u8; KEY_BYTES],
) -> Result<(), VaultError> {
    let params = Params::new(
        ARGON2_M_COST_KIB,
        ARGON2_T_COST,
        ARGON2_P_COST,
        Some(KEY_BYTES),
    )
    .map_err(|_| VaultError::locked())?;
    let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    passphrase
        .expose(|value| argon.hash_password_into(value, salt, output))
        .map_err(|_| VaultError::locked())
}

fn derive_key(info: &[u8], input: &[u8], output: &mut [u8]) -> Result<(), VaultError> {
    Hkdf::<Sha256>::new(None, input)
        .expand(info, output)
        .map_err(|_| VaultError::locked())
}

fn metadata_hkdf_info(metadata: &VaultMetadata) -> Vec<u8> {
    let mut output = Vec::new();
    frame(&mut output, INFO_DOMAIN);
    frame(&mut output, metadata.asset.text().as_bytes());
    frame(&mut output, metadata.network.text().as_bytes());
    frame(&mut output, &metadata.account_id);
    output
}

fn vault_aad(
    metadata: &VaultMetadata,
    salt: &[u8; SALT_BYTES],
    nonce: &[u8; NONCE_BYTES],
) -> Vec<u8> {
    let mut output = Vec::new();
    frame(&mut output, VAULT_FORMAT.as_bytes());
    frame(&mut output, VAULT_VERSION.to_string().as_bytes());
    frame(&mut output, &metadata.account_id);
    frame(&mut output, metadata.asset.text().as_bytes());
    frame(&mut output, metadata.network.text().as_bytes());
    frame(&mut output, metadata.epoch.to_string().as_bytes());
    frame(&mut output, KDF_ALGORITHM.as_bytes());
    frame(&mut output, ARGON2_VERSION.to_string().as_bytes());
    frame(&mut output, ARGON2_M_COST_KIB.to_string().as_bytes());
    frame(&mut output, ARGON2_T_COST.to_string().as_bytes());
    frame(&mut output, ARGON2_P_COST.to_string().as_bytes());
    frame(&mut output, AEAD_ALGORITHM.as_bytes());
    frame(&mut output, salt);
    frame(&mut output, nonce);
    output
}

fn frame(output: &mut Vec<u8>, value: &[u8]) {
    output.extend_from_slice(&(value.len() as u32).to_be_bytes());
    output.extend_from_slice(value);
}

fn envelope_from_parts(
    metadata: VaultMetadata,
    salt: [u8; SALT_BYTES],
    nonce: [u8; NONCE_BYTES],
    ciphertext: Vec<u8>,
) -> Result<VaultEnvelope, VaultError> {
    let wire = WireEnvelope {
        format: VAULT_FORMAT.to_owned(),
        version: VAULT_VERSION,
        account_id: metadata.account_id_hex(),
        asset: metadata.asset.text().to_owned(),
        network: metadata.network.text().to_owned(),
        epoch: metadata.epoch.to_string(),
        kdf: WireKdf {
            algorithm: KDF_ALGORITHM.to_owned(),
            version: ARGON2_VERSION,
            m_cost_kib: ARGON2_M_COST_KIB,
            t_cost: ARGON2_T_COST,
            p_cost: ARGON2_P_COST,
            salt_b64: Base64Unpadded::encode_string(&salt),
        },
        aead: WireAead {
            algorithm: AEAD_ALGORITHM.to_owned(),
            nonce_b64: Base64Unpadded::encode_string(&nonce),
            ciphertext_b64: Base64Unpadded::encode_string(&ciphertext),
        },
    };
    let bytes = serialize_wire(&wire)?;
    if bytes.len() > MAX_ENVELOPE_BYTES {
        return Err(VaultError::limit());
    }
    Ok(VaultEnvelope {
        metadata,
        salt,
        nonce,
        ciphertext,
        bytes,
    })
}

fn serialize_wire(wire: &WireEnvelope) -> Result<Vec<u8>, VaultError> {
    let mut bytes = serde_json::to_vec(wire).map_err(|_| VaultError::schema())?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn validate_wire_profile(wire: &WireEnvelope) -> Result<(), VaultError> {
    if wire.format != VAULT_FORMAT
        || wire.version != VAULT_VERSION
        || wire.kdf.algorithm != KDF_ALGORITHM
        || wire.kdf.version != ARGON2_VERSION
        || wire.kdf.m_cost_kib != ARGON2_M_COST_KIB
        || wire.kdf.t_cost != ARGON2_T_COST
        || wire.kdf.p_cost != ARGON2_P_COST
        || wire.aead.algorithm != AEAD_ALGORITHM
    {
        return Err(VaultError::schema());
    }
    Ok(())
}

fn validate_asset_network(asset: Asset, network: Network) -> Result<(), VaultError> {
    match (asset, network) {
        (Asset::Zec, Network::ZecTestnet | Network::ZecRegtest)
        | (Asset::Xmr, Network::XmrStagenet) => Ok(()),
        _ => Err(VaultError::wrong_network()),
    }
}

fn parse_epoch(value: &str) -> Result<u64, VaultError> {
    let epoch = value.parse::<u64>().map_err(|_| VaultError::schema())?;
    if epoch == 0 || epoch.to_string() != value {
        return Err(VaultError::schema());
    }
    Ok(epoch)
}

fn parse_account_id(value: &str) -> Result<[u8; 16], VaultError> {
    if value.len() != 32
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(VaultError::schema());
    }
    let mut output = [0u8; 16];
    for (index, pair) in value.as_bytes().as_chunks::<2>().0.iter().enumerate() {
        output[index] = (hex_value(pair[0])? << 4) | hex_value(pair[1])?;
    }
    Ok(output)
}

pub(crate) fn valid_account_id(value: &str) -> bool {
    parse_account_id(value).is_ok()
}

fn hex_value(value: u8) -> Result<u8, VaultError> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => Err(VaultError::schema()),
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn fixed_bytes<const N: usize>(bytes: &[u8]) -> Result<[u8; N], VaultError> {
    bytes.try_into().map_err(|_| VaultError::schema())
}

fn wipe_array<const N: usize>(
    bytes: &mut [u8; N],
    label: &'static str,
    observer: &mut dyn WipeObserver,
) {
    bytes.zeroize();
    observer.observe(WipeEvent {
        label,
        length: N,
        all_zero: bytes.iter().all(|byte| *byte == 0),
    });
}
