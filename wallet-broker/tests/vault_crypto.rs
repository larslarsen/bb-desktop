//! Independent primitive vectors: RFC 5869 test case 1, RFC 9106 §5.3,
//! and draft-irtf-cfrg-xchacha-03 Appendix A.3.1.

use argon2::{Algorithm, Argon2, AssociatedData, ParamsBuilder, Version};
use bitbook_wallet_broker::vault::{
    AEAD_ALGORITHM, ARGON2_M_COST_KIB, ARGON2_P_COST, ARGON2_T_COST, ARGON2_VERSION,
    Asset, EntropyPort, Network, SecretBytes, VAULT_FORMAT, VAULT_VERSION, VaultError,
    VaultMetadata, VaultWorkObserver, WipeEvent, WipeObserver, open_vault_bytes, seal_vault,
};
use chacha20poly1305::aead::{AeadInPlace, KeyInit};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use hkdf::Hkdf;
use sha2::Sha256;

fn hex(value: &str) -> Vec<u8> {
    assert_eq!(value.len() % 2, 0);
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let text = core::str::from_utf8(pair).unwrap();
            u8::from_str_radix(text, 16).unwrap()
        })
        .collect()
}

#[derive(Default)]
struct Wipes(Vec<WipeEvent>);

impl WipeObserver for Wipes {
    fn observe(&mut self, event: WipeEvent) {
        self.0.push(event);
    }
}

#[derive(Default)]
struct Work {
    kdf_calls: usize,
}

impl VaultWorkObserver for Work {
    fn before_allocation(&mut self, _bytes: usize) -> Result<(), VaultError> {
        Ok(())
    }

    fn before_kdf(&mut self) {
        self.kdf_calls += 1;
    }
}

struct FixedEntropy {
    salt: [u8; 16],
    nonce: [u8; 24],
    calls: Vec<(&'static str, usize)>,
}

impl EntropyPort for FixedEntropy {
    fn fill(&mut self, label: &'static str, output: &mut [u8]) -> Result<(), VaultError> {
        self.calls.push((label, output.len()));
        match label {
            "vault-salt" if output.len() == self.salt.len() => output.copy_from_slice(&self.salt),
            "vault-nonce" if output.len() == self.nonce.len() => output.copy_from_slice(&self.nonce),
            _ => return Err(VaultError::entropy()),
        }
        Ok(())
    }
}

fn metadata(epoch: u64) -> VaultMetadata {
    VaultMetadata::new(
        [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
            0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
        ],
        Asset::Zec,
        Network::ZecTestnet,
        epoch,
    )
    .unwrap()
}

fn sealed(epoch: u64) -> Vec<u8> {
    let mut entropy = FixedEntropy {
        salt: [0x42; 16],
        nonce: [0x24; 24],
        calls: Vec::new(),
    };
    let mut wipes = Wipes::default();
    let mut passphrase = SecretBytes::new(b"synthetic-vault-passphrase".to_vec()).unwrap();
    let mut plaintext = SecretBytes::new(b"CANARY_WAL004_OPAQUE_SECRET".to_vec()).unwrap();
    let envelope = seal_vault(
        &metadata(epoch),
        &mut passphrase,
        &mut plaintext,
        &mut entropy,
        &mut wipes,
    )
    .unwrap();
    assert_eq!(entropy.calls, vec![("vault-salt", 16), ("vault-nonce", 24)]);
    envelope.into_bytes()
}

#[test]
fn crypto_constants_are_the_closed_v1_profile() {
    assert_eq!(VAULT_FORMAT, "bitbook-wallet-vault");
    assert_eq!(VAULT_VERSION, 1);
    assert_eq!(ARGON2_VERSION, 19);
    assert_eq!(ARGON2_M_COST_KIB, 65_536);
    assert_eq!(ARGON2_T_COST, 3);
    assert_eq!(ARGON2_P_COST, 1);
    assert_eq!(AEAD_ALGORITHM, "xchacha20poly1305");
}

#[test]
fn rfc9106_argon2id_v19_vector_is_independent() {
    let associated = AssociatedData::new(&[0x04; 12]).unwrap();
    let mut builder = ParamsBuilder::new();
    builder
        .m_cost(32)
        .t_cost(3)
        .p_cost(4)
        .output_len(32)
        .data(associated);
    let params = builder.build().unwrap();
    let secret = [0x03; 8];
    let argon2 = Argon2::new_with_secret(
        &secret,
        Algorithm::Argon2id,
        Version::V0x13,
        params,
    )
    .unwrap();
    let mut output = [0u8; 32];
    argon2
        .hash_password_into(&[0x01; 32], &[0x02; 16], &mut output)
        .unwrap();
    assert_eq!(
        output.as_slice(),
        hex("0d640df58d78766c08c037a34a8b53c9d01ef0452d75b65eb52520e96b01e659")
    );
}

#[test]
fn rfc5869_hkdf_sha256_vector_is_independent() {
    let ikm = [0x0b; 22];
    let salt = hex("000102030405060708090a0b0c");
    let info = hex("f0f1f2f3f4f5f6f7f8f9");
    let hkdf = Hkdf::<Sha256>::new(Some(&salt), &ikm);
    let mut output = [0u8; 42];
    hkdf.expand(&info, &mut output).unwrap();
    assert_eq!(
        output.as_slice(),
        hex(concat!(
            "3cb25f25faacd57a90434f64d0362f2a",
            "2d2d0a90cf1a5a4c5db02d56ecc4c5bf",
            "34007208d5b887185865"
        ))
    );
}

#[test]
fn xchacha20poly1305_draft_vector_is_independent() {
    let key = hex("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f");
    let nonce = hex("404142434445464748494a4b4c4d4e4f5051525354555657");
    let aad = hex("50515253c0c1c2c3c4c5c6c7");
    let mut plaintext = hex(concat!(
        "4c616469657320616e642047656e746c656d656e206f662074686520636c617373206f66202739393a20",
        "4966204920636f756c64206f6666657220796f75206f6e6c79206f6e652074697020666f722074686520",
        "6675747572652c2073756e73637265656e20776f756c642062652069742e"
    ));
    let cipher = XChaCha20Poly1305::new_from_slice(&key).unwrap();
    let tag = cipher
        .encrypt_in_place_detached(XNonce::from_slice(&nonce), &aad, &mut plaintext)
        .unwrap();
    assert_eq!(
        plaintext,
        hex(concat!(
            "bd6d179d3e83d43b9576579493c0e939572a1700252bfaccbed2902c21396cbb",
            "731c7f1b0b4aa6440bf3a82f4eda7e39ae64c6708c54c216cb96b72e1213b452",
            "2f8c9ba40db5d945b11b69b982c1bb9e3f3fac2bc369488f76b2383565d3fff9",
            "21f9664c97637da9768812f615c68b13b52e"
        ))
    );
    assert_eq!(tag.as_ref(), hex("c0875924c1c7987947deafd8780acf49"));
}

#[test]
fn deterministic_entropy_produces_one_stable_envelope_and_is_fully_openable() {
    let actual = sealed(7);
    let expected = concat!(
        "{\"format\":\"bitbook-wallet-vault\",\"version\":1,",
        "\"account_id\":\"00112233445566778899aabbccddeeff\",",
        "\"asset\":\"ZEC\",\"network\":\"zec-testnet\",\"epoch\":\"7\",",
        "\"kdf\":{\"algorithm\":\"argon2id\",\"version\":19,\"m_cost_kib\":65536,",
        "\"t_cost\":3,\"p_cost\":1,\"salt_b64\":\"QkJCQkJCQkJCQkJCQkJCQg\"},",
        "\"aead\":{\"algorithm\":\"xchacha20poly1305\",",
        "\"nonce_b64\":\"JCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQk\",",
        "\"ciphertext_b64\":\"u97UrVKi33aeVs/8hWOSv2nFGDxmoitKxvjXfg6Dg99bvKJDu76lymjHUA\"}}\n"
    );
    assert_eq!(actual, expected.as_bytes());
    let mut passphrase = SecretBytes::new(b"synthetic-vault-passphrase".to_vec()).unwrap();
    let mut work = Work::default();
    let mut wipes = Wipes::default();
    let opened = open_vault_bytes(expected.as_bytes(), &mut passphrase, &mut work, &mut wipes).unwrap();
    assert_eq!(work.kdf_calls, 1);
    opened.expose(|bytes| assert_eq!(bytes, b"CANARY_WAL004_OPAQUE_SECRET"));
}

#[test]
fn fresh_entropy_randomizes_both_salt_and_nonce_and_changes_ciphertext() {
    let mut entropy_a = FixedEntropy { salt: [1; 16], nonce: [2; 24], calls: Vec::new() };
    let mut entropy_b = FixedEntropy { salt: [3; 16], nonce: [4; 24], calls: Vec::new() };
    let mut wipes = Wipes::default();
    let mut pass_a = SecretBytes::new(b"synthetic-vault-passphrase".to_vec()).unwrap();
    let mut pass_b = SecretBytes::new(b"synthetic-vault-passphrase".to_vec()).unwrap();
    let mut plain_a = SecretBytes::new(b"CANARY_WAL004_OPAQUE_SECRET".to_vec()).unwrap();
    let mut plain_b = SecretBytes::new(b"CANARY_WAL004_OPAQUE_SECRET".to_vec()).unwrap();
    let a = seal_vault(&metadata(8), &mut pass_a, &mut plain_a, &mut entropy_a, &mut wipes).unwrap();
    let b = seal_vault(&metadata(8), &mut pass_b, &mut plain_b, &mut entropy_b, &mut wipes).unwrap();
    assert_ne!(a.salt(), b.salt());
    assert_ne!(a.nonce(), b.nonce());
    assert_ne!(a.ciphertext(), b.ciphertext());
}

#[test]
fn passphrases_are_exact_utf8_bytes_without_unicode_normalization() {
    let mut entropy_a = FixedEntropy { salt: [5; 16], nonce: [6; 24], calls: Vec::new() };
    let mut entropy_b = FixedEntropy { salt: [5; 16], nonce: [6; 24], calls: Vec::new() };
    let mut wipes = Wipes::default();
    let mut composed = SecretBytes::new("synthetic-\u{00e9}".as_bytes().to_vec()).unwrap();
    let mut decomposed = SecretBytes::new("synthetic-e\u{0301}".as_bytes().to_vec()).unwrap();
    let mut plain_a = SecretBytes::new(b"CANARY_WAL004_OPAQUE_SECRET".to_vec()).unwrap();
    let mut plain_b = SecretBytes::new(b"CANARY_WAL004_OPAQUE_SECRET".to_vec()).unwrap();
    let a = seal_vault(&metadata(8), &mut composed, &mut plain_a, &mut entropy_a, &mut wipes).unwrap();
    let b = seal_vault(&metadata(8), &mut decomposed, &mut plain_b, &mut entropy_b, &mut wipes).unwrap();
    assert_ne!(a.ciphertext(), b.ciphertext());
    let mut wrong_form = SecretBytes::new("synthetic-e\u{0301}".as_bytes().to_vec()).unwrap();
    let mut work = Work::default();
    assert_eq!(
        open_vault_bytes(a.as_bytes(), &mut wrong_form, &mut work, &mut wipes).unwrap_err().code(),
        "LOCKED"
    );
}

#[test]
fn authenticated_domain_mutations_all_fail_locked() {
    let original = sealed(9);
    let replacements = [
        ("00112233445566778899aabbccddeeff", "10112233445566778899aabbccddeeff"),
        ("\"network\":\"zec-testnet\"", "\"network\":\"zec-regtest\""),
        ("\"epoch\":\"9\"", "\"epoch\":\"10\""),
    ];
    for (from, to) in replacements {
        let mutated = String::from_utf8(original.clone()).unwrap().replacen(from, to, 1).into_bytes();
        let mut passphrase = SecretBytes::new(b"synthetic-vault-passphrase".to_vec()).unwrap();
        let mut work = Work::default();
        let mut wipes = Wipes::default();
        let error = open_vault_bytes(&mutated, &mut passphrase, &mut work, &mut wipes).unwrap_err();
        assert_eq!(error.code(), "LOCKED");
    }
}

#[test]
fn salt_nonce_and_ciphertext_mutations_fail_closed() {
    let original = sealed(11);
    for field in ["salt_b64", "nonce_b64", "ciphertext_b64"] {
        let mut text = String::from_utf8(original.clone()).unwrap();
        let value_start = text.find(&format!("\"{field}\":\"")).unwrap() + field.len() + 4;
        let byte = text.as_bytes()[value_start];
        text.replace_range(value_start..value_start + 1, if byte == b'A' { "B" } else { "A" });
        let mut passphrase = SecretBytes::new(b"synthetic-vault-passphrase".to_vec()).unwrap();
        let mut work = Work::default();
        let mut wipes = Wipes::default();
        assert_eq!(
            open_vault_bytes(text.as_bytes(), &mut passphrase, &mut work, &mut wipes).unwrap_err().code(),
            "LOCKED"
        );
    }
}

#[test]
fn zec_xmr_and_social_domain_substitution_cannot_open() {
    let zec = sealed(12);
    let xmr = String::from_utf8(zec.clone()).unwrap()
        .replace("\"asset\":\"ZEC\",\"network\":\"zec-testnet\"", "\"asset\":\"XMR\",\"network\":\"xmr-stagenet\"")
        .into_bytes();
    let mut passphrase = SecretBytes::new(b"synthetic-vault-passphrase".to_vec()).unwrap();
    let mut work = Work::default();
    let mut wipes = Wipes::default();
    assert_eq!(open_vault_bytes(&xmr, &mut passphrase, &mut work, &mut wipes).unwrap_err().code(), "LOCKED");

    let social = String::from_utf8(zec).unwrap()
        .replace("bitbook-wallet-vault", "bitbook-social-identity")
        .into_bytes();
    let mut passphrase = SecretBytes::new(b"synthetic-vault-passphrase".to_vec()).unwrap();
    let mut work = Work::default();
    assert!(open_vault_bytes(&social, &mut passphrase, &mut work, &mut wipes).is_err());
}

#[test]
fn wrong_passphrase_and_corrupt_tag_have_identical_public_failure() {
    let envelope = sealed(13);
    let mut wrong = SecretBytes::new(b"synthetic-wrong-passphrase".to_vec()).unwrap();
    let mut corrupt = String::from_utf8(envelope.clone()).unwrap();
    let field = "\"ciphertext_b64\":\"";
    let index = corrupt.find(field).unwrap() + field.len();
    let byte = corrupt.as_bytes()[index];
    corrupt.replace_range(index..index + 1, if byte == b'A' { "B" } else { "A" });
    let mut correct = SecretBytes::new(b"synthetic-vault-passphrase".to_vec()).unwrap();
    let mut wrong_work = Work::default();
    let mut corrupt_work = Work::default();
    let mut wipes = Wipes::default();
    let wrong_error = open_vault_bytes(&envelope, &mut wrong, &mut wrong_work, &mut wipes).unwrap_err();
    let corrupt_error = open_vault_bytes(
        corrupt.as_bytes(),
        &mut correct,
        &mut corrupt_work,
        &mut wipes,
    )
    .unwrap_err();
    assert_eq!(wrong_error.code(), "LOCKED");
    assert_eq!(wrong_error.code(), corrupt_error.code());
    assert_eq!(wrong_error.public_message(), corrupt_error.public_message());
    assert_eq!(wrong_error.public_message(), "Wallet locked");
}
