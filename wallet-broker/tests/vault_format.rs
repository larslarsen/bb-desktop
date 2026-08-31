use bitbook_wallet_broker::vault::{
    MAX_ENVELOPE_BYTES, MAX_PASSPHRASE_BYTES, MAX_PLAINTEXT_BYTES, Asset, Network,
    SecretBytes, VaultError, VaultInputs, VaultMetadata, VaultWorkObserver, WipeEvent,
    WipeObserver, open_vault_bytes, parse_vault,
};

const FIXTURE: &[u8] = include_bytes!("fixtures/vault-v1.json");

#[derive(Default)]
struct WorkLog {
    allocations: Vec<usize>,
    kdf_calls: usize,
}

impl VaultWorkObserver for WorkLog {
    fn before_allocation(&mut self, bytes: usize) -> Result<(), VaultError> {
        self.allocations.push(bytes);
        Ok(())
    }

    fn before_kdf(&mut self) {
        self.kdf_calls += 1;
    }
}

#[derive(Default)]
struct Wipes(Vec<WipeEvent>);

impl WipeObserver for Wipes {
    fn observe(&mut self, event: WipeEvent) {
        self.0.push(event);
    }
}

fn parse(bytes: &[u8]) -> Result<bitbook_wallet_broker::vault::VaultEnvelope, VaultError> {
    parse_vault(bytes, &mut WorkLog::default())
}

fn replace_once(source: &[u8], from: &str, to: &str) -> Vec<u8> {
    let text = core::str::from_utf8(source).unwrap();
    assert!(text.contains(from), "mutation target missing: {from}");
    text.replacen(from, to, 1).into_bytes()
}

#[test]
fn golden_vault_fixture_is_exact_canonical_bytes() {
    let expected = concat!(
        "{\"format\":\"bitbook-wallet-vault\",\"version\":1,",
        "\"account_id\":\"00112233445566778899aabbccddeeff\",",
        "\"asset\":\"ZEC\",\"network\":\"zec-testnet\",\"epoch\":\"7\",",
        "\"kdf\":{\"algorithm\":\"argon2id\",\"version\":19,\"m_cost_kib\":65536,",
        "\"t_cost\":3,\"p_cost\":1,\"salt_b64\":\"AAAAAAAAAAAAAAAAAAAAAA\"},",
        "\"aead\":{\"algorithm\":\"xchacha20poly1305\",",
        "\"nonce_b64\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\",",
        "\"ciphertext_b64\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"}}\n"
    );
    assert_eq!(FIXTURE, expected.as_bytes());
    assert_eq!(FIXTURE.last(), Some(&b'\n'));
}

#[test]
fn canonical_fixture_round_trips_byte_for_byte() {
    let envelope = parse(FIXTURE).unwrap();
    assert_eq!(envelope.to_bytes(), FIXTURE);
    assert_eq!(envelope.metadata().account_id_hex(), "00112233445566778899aabbccddeeff");
    assert_eq!(envelope.metadata().asset(), Asset::Zec);
    assert_eq!(envelope.metadata().network(), Network::ZecTestnet);
    assert_eq!(envelope.metadata().epoch(), 7);
}

#[test]
fn every_unknown_duplicate_missing_and_reordered_field_is_rejected() {
    let kdf = concat!(
        "\"kdf\":{\"algorithm\":\"argon2id\",\"version\":19,\"m_cost_kib\":65536,",
        "\"t_cost\":3,\"p_cost\":1,\"salt_b64\":\"AAAAAAAAAAAAAAAAAAAAAA\"}"
    );
    let aead = concat!(
        "\"aead\":{\"algorithm\":\"xchacha20poly1305\",",
        "\"nonce_b64\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\",",
        "\"ciphertext_b64\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"}"
    );
    let mut rows = vec![
        replace_once(FIXTURE, "{\"format\"", "{\"unknown\":true,\"format\""),
        replace_once(
            FIXTURE,
            "\"asset\":\"ZEC\",\"network\":\"zec-testnet\"",
            "\"network\":\"zec-testnet\",\"asset\":\"ZEC\"",
        ),
        replace_once(FIXTURE, "\"salt_b64\"", "\"extra\":0,\"salt_b64\""),
        replace_once(FIXTURE, "\"nonce_b64\"", "\"extra\":0,\"nonce_b64\""),
        replace_once(
            FIXTURE,
            "\"version\":19,\"m_cost_kib\":65536",
            "\"m_cost_kib\":65536,\"version\":19",
        ),
        replace_once(
            FIXTURE,
            "\"nonce_b64\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\",\"ciphertext_b64\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"",
            "\"ciphertext_b64\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\",\"nonce_b64\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"",
        ),
        replace_once(FIXTURE, &format!("{kdf},"), ""),
        replace_once(FIXTURE, &format!(",{aead}"), ""),
        replace_once(FIXTURE, kdf, &format!("{kdf},{kdf}")),
        replace_once(FIXTURE, aead, &format!("{aead},{aead}")),
    ];

    for field in [
        "\"format\":\"bitbook-wallet-vault\"",
        "\"version\":1",
        "\"account_id\":\"00112233445566778899aabbccddeeff\"",
        "\"asset\":\"ZEC\"",
        "\"network\":\"zec-testnet\"",
        "\"epoch\":\"7\"",
        "\"algorithm\":\"argon2id\"",
        "\"version\":19",
        "\"m_cost_kib\":65536",
        "\"t_cost\":3",
        "\"p_cost\":1",
        "\"salt_b64\":\"AAAAAAAAAAAAAAAAAAAAAA\"",
        "\"algorithm\":\"xchacha20poly1305\"",
        "\"nonce_b64\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"",
        "\"ciphertext_b64\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"",
    ] {
        rows.push(replace_once(FIXTURE, field, &format!("{field},{field}")));
    }

    for (field, replacement) in [
        ("\"format\":\"bitbook-wallet-vault\",", ""),
        ("\"version\":1,", ""),
        ("\"account_id\":\"00112233445566778899aabbccddeeff\",", ""),
        ("\"asset\":\"ZEC\",", ""),
        ("\"network\":\"zec-testnet\",", ""),
        ("\"epoch\":\"7\",", ""),
        ("\"algorithm\":\"argon2id\",", ""),
        ("\"version\":19,", ""),
        ("\"m_cost_kib\":65536,", ""),
        ("\"t_cost\":3,", ""),
        ("\"p_cost\":1,", ""),
        (",\"salt_b64\":\"AAAAAAAAAAAAAAAAAAAAAA\"", ""),
        ("\"algorithm\":\"xchacha20poly1305\",", ""),
        ("\"nonce_b64\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\",", ""),
        (",\"ciphertext_b64\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"", ""),
    ] {
        rows.push(replace_once(FIXTURE, field, replacement));
    }
    for bytes in rows {
        assert_eq!(parse(&bytes).unwrap_err().code(), "SCHEMA");
    }
}

#[test]
fn wrong_json_types_and_noncanonical_epoch_are_rejected() {
    for bytes in [
        replace_once(FIXTURE, "\"format\":\"bitbook-wallet-vault\"", "\"format\":null"),
        replace_once(FIXTURE, "\"version\":1", "\"version\":\"1\""),
        replace_once(FIXTURE, "\"account_id\":\"00112233445566778899aabbccddeeff\"", "\"account_id\":1"),
        replace_once(FIXTURE, "\"asset\":\"ZEC\"", "\"asset\":[]"),
        replace_once(FIXTURE, "\"network\":\"zec-testnet\"", "\"network\":false"),
        replace_once(FIXTURE, "\"epoch\":\"7\"", "\"epoch\":7"),
        replace_once(FIXTURE, "\"epoch\":\"7\"", "\"epoch\":\"0\""),
        replace_once(FIXTURE, "\"epoch\":\"7\"", "\"epoch\":\"07\""),
        replace_once(FIXTURE, "\"epoch\":\"7\"", "\"epoch\":\"18446744073709551616\""),
        replace_once(FIXTURE, "\"m_cost_kib\":65536", "\"m_cost_kib\":\"65536\""),
        replace_once(FIXTURE, "\"version\":19", "\"version\":\"19\""),
        replace_once(FIXTURE, "\"t_cost\":3", "\"t_cost\":\"3\""),
        replace_once(FIXTURE, "\"p_cost\":1", "\"p_cost\":null"),
        replace_once(FIXTURE, "\"algorithm\":\"argon2id\"", "\"algorithm\":19"),
        replace_once(
            FIXTURE,
            "\"algorithm\":\"xchacha20poly1305\"",
            "\"algorithm\":false",
        ),
        replace_once(FIXTURE, "\"salt_b64\":\"AAAAAAAAAAAAAAAAAAAAAA\"", "\"salt_b64\":[]"),
        replace_once(
            FIXTURE,
            "\"nonce_b64\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"",
            "\"nonce_b64\":1",
        ),
        replace_once(
            FIXTURE,
            "\"ciphertext_b64\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"",
            "\"ciphertext_b64\":null",
        ),
    ] {
        assert_eq!(parse(&bytes).unwrap_err().code(), "SCHEMA");
    }
}

#[test]
fn noncanonical_json_bom_crlf_trailing_and_invalid_utf8_are_rejected() {
    let mut bom = vec![0xef, 0xbb, 0xbf];
    bom.extend_from_slice(FIXTURE);
    let mut invalid_utf8 = FIXTURE.to_vec();
    invalid_utf8[1] = 0xff;
    for bytes in [
        replace_once(FIXTURE, "{\"format\"", "{ \"format\""),
        replace_once(FIXTURE, ":1,", ": 1,"),
        replace_once(FIXTURE, "\n", "\r\n"),
        [FIXTURE, b"x"].concat(),
        bom,
        invalid_utf8,
    ] {
        assert_eq!(parse(&bytes).unwrap_err().code(), "SCHEMA");
    }
}

#[test]
fn base64_is_unpadded_unspaced_and_exact_length() {
    for bytes in [
        replace_once(FIXTURE, "AAAAAAAAAAAAAAAAAAAAAA\"", "AAAAAAAAAAAAAAAAAAAAAA=\""),
        replace_once(FIXTURE, "AAAAAAAAAAAAAAAAAAAAAA\"", " AAAAAAAAAAAAAAAAAAAAAA\""),
        replace_once(FIXTURE, "AAAAAAAAAAAAAAAAAAAAAA\"", "AAAAAAAAAAAAAAAAAAAAA\""),
        replace_once(
            FIXTURE,
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"",
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=\"",
        ),
        replace_once(FIXTURE, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA", "A"),
    ] {
        assert_eq!(parse(&bytes).unwrap_err().code(), "SCHEMA");
    }
}

#[test]
fn asset_network_and_account_id_are_closed() {
    for bytes in [
        replace_once(FIXTURE, "\"asset\":\"ZEC\"", "\"asset\":\"BTC\""),
        replace_once(FIXTURE, "zec-testnet", "xmr-stagenet"),
        replace_once(FIXTURE, "00112233445566778899aabbccddeeff", "00112233445566778899AABBCCDDEEFF"),
        replace_once(FIXTURE, "00112233445566778899aabbccddeeff", "00112233"),
    ] {
        assert_eq!(parse(&bytes).unwrap_err().code(), "SCHEMA");
    }
}

#[test]
fn every_kdf_and_aead_parameter_downgrade_is_rejected_before_kdf() {
    let rows = [
        ("\"algorithm\":\"argon2id\"", "\"algorithm\":\"argon2i\""),
        ("\"version\":19", "\"version\":16"),
        ("\"m_cost_kib\":65536", "\"m_cost_kib\":65535"),
        ("\"m_cost_kib\":65536", "\"m_cost_kib\":1048576"),
        ("\"t_cost\":3", "\"t_cost\":2"),
        ("\"t_cost\":3", "\"t_cost\":4"),
        ("\"p_cost\":1", "\"p_cost\":0"),
        ("\"p_cost\":1", "\"p_cost\":8"),
        ("\"algorithm\":\"xchacha20poly1305\"", "\"algorithm\":\"chacha20poly1305\""),
    ];
    for (from, to) in rows {
        let bytes = replace_once(FIXTURE, from, to);
        let mut work = WorkLog::default();
        let mut passphrase = SecretBytes::new(b"synthetic-vault-passphrase".to_vec()).unwrap();
        let mut wipes = Wipes::default();
        assert_eq!(
            open_vault_bytes(&bytes, &mut passphrase, &mut work, &mut wipes)
                .unwrap_err()
                .code(),
            "SCHEMA"
        );
        assert_eq!(work.kdf_calls, 0);
    }
}

#[test]
fn passphrase_and_plaintext_bounds_are_exact_before_entropy_or_kdf() {
    assert_eq!(MAX_PASSPHRASE_BYTES, 1_024);
    assert_eq!(MAX_PLAINTEXT_BYTES, 65_536);
    for (passphrase, plaintext, accepted) in [
        (0, 1, false),
        (1, 1, true),
        (1_024, 65_536, true),
        (1_025, 1, false),
        (1, 0, false),
        (1, 65_537, false),
    ] {
        let passphrase = SecretBytes::new(vec![b'p'; passphrase]).unwrap();
        let plaintext = SecretBytes::new(vec![0x5a; plaintext]).unwrap();
        let result = VaultInputs::new(passphrase, plaintext);
        assert_eq!(result.is_ok(), accepted);
        if !accepted {
            assert_eq!(result.unwrap_err().code(), "LIMIT");
        }
    }

    let invalid_utf8 = SecretBytes::new(vec![0xff]).unwrap();
    let plaintext = SecretBytes::new(vec![0x5a]).unwrap();
    assert!(VaultInputs::new(invalid_utf8, plaintext).is_err());
}

#[test]
fn envelope_limit_is_checked_before_body_allocation() {
    assert_eq!(MAX_ENVELOPE_BYTES, 128 * 1_024);
    let mut exact_work = WorkLog::default();
    let exact = vec![b' '; MAX_ENVELOPE_BYTES];
    assert_eq!(parse_vault(&exact, &mut exact_work).unwrap_err().code(), "SCHEMA");
    assert_eq!(exact_work.allocations, vec![MAX_ENVELOPE_BYTES]);

    let mut over_work = WorkLog::default();
    let over = vec![b' '; MAX_ENVELOPE_BYTES + 1];
    assert_eq!(parse_vault(&over, &mut over_work).unwrap_err().code(), "LIMIT");
    assert!(over_work.allocations.is_empty());
    assert_eq!(over_work.kdf_calls, 0);
}

#[test]
fn metadata_constructor_rejects_zero_epoch_and_crossed_networks() {
    let id = [0x11; 16];
    assert_eq!(
        VaultMetadata::new(id, Asset::Zec, Network::ZecTestnet, 0).unwrap_err().code(),
        "SCHEMA"
    );
    assert_eq!(
        VaultMetadata::new(id, Asset::Zec, Network::XmrStagenet, 1).unwrap_err().code(),
        "WRONG_NETWORK"
    );
    assert_eq!(
        VaultMetadata::new(id, Asset::Xmr, Network::ZecRegtest, 1).unwrap_err().code(),
        "WRONG_NETWORK"
    );
}
