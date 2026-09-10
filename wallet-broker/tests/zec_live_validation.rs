use bitbook_wallet_broker::zec::test_support::validate_live_batch_for_test;
use zcash_client_backend::proto::compact_formats::{CompactBlock, CompactTx};
use zcash_primitives::block::{BlockHash, BlockHeader, BlockHeaderData};

const HEIGHT: u32 = 1_842_421;

#[test]
fn parsed_header_hashes_must_agree_with_explicit_compact_block_fields() {
    let header = BlockHeaderData {
        version: 4,
        prev_block: BlockHash([0x11; 32]),
        merkle_root: [0x22; 32],
        final_sapling_root: [0x33; 32],
        time: 1_700_000_000,
        bits: 0x1f07_ffff,
        nonce: [0x44; 32],
        solution: Vec::new(),
    }
    .freeze()
    .unwrap();
    let mut encoded_header = Vec::new();
    header.write(&mut encoded_header).unwrap();

    let parsed = BlockHeader::read(encoded_header.as_slice()).unwrap();
    let parsed_hash = parsed.hash();
    let parsed_previous = parsed.prev_block;
    let mut block = compact_block();
    block.header = encoded_header;
    block.hash = parsed_hash.0.to_vec();
    block.prev_hash = parsed_previous.0.to_vec();
    assert_eq!(block.header().unwrap().hash(), parsed_hash);
    assert_eq!(block.hash(), parsed_hash);
    assert_eq!(block.prev_hash(), parsed_previous);

    let mut hash_mismatch = block.clone();
    hash_mismatch.hash[0] ^= 1;
    assert_ne!(hash_mismatch.hash.as_slice(), parsed_hash.0.as_slice());
    assert_eq!(
        hash_mismatch.prev_hash.as_slice(),
        parsed_previous.0.as_slice()
    );
    let hash_error = validate_live_batch_for_test(&[hash_mismatch], HEIGHT, HEIGHT).unwrap_err();
    assert_eq!(hash_error.code(), "PROTOCOL_INCOMPATIBLE");

    let mut previous_mismatch = block.clone();
    previous_mismatch.prev_hash[0] ^= 1;
    assert_eq!(previous_mismatch.hash.as_slice(), parsed_hash.0.as_slice());
    assert_ne!(
        previous_mismatch.prev_hash.as_slice(),
        parsed_previous.0.as_slice()
    );
    let previous_error =
        validate_live_batch_for_test(&[previous_mismatch], HEIGHT, HEIGHT).unwrap_err();
    assert_eq!(previous_error.code(), "PROTOCOL_INCOMPATIBLE");

    validate_live_batch_for_test(&[block], HEIGHT, HEIGHT).unwrap();
}

#[test]
fn compact_transaction_ids_must_be_exactly_thirty_two_bytes_before_scanning() {
    for malformed_length in [31, 33] {
        let mut block = compact_block();
        block.vtx.push(compact_tx(malformed_length));
        let error = validate_live_batch_for_test(&[block], HEIGHT, HEIGHT).unwrap_err();
        assert_eq!(
            error.code(),
            "PROTOCOL_INCOMPATIBLE",
            "accepted {malformed_length}-byte CompactTx txid"
        );
    }

    let mut control = compact_block();
    control.vtx.push(compact_tx(32));
    validate_live_batch_for_test(&[control], HEIGHT, HEIGHT).unwrap();
}

fn compact_block() -> CompactBlock {
    CompactBlock {
        height: u64::from(HEIGHT),
        hash: vec![0xaa; 32],
        prev_hash: vec![0xbb; 32],
        time: 1_700_000_075,
        header: Vec::new(),
        vtx: Vec::new(),
        chain_metadata: None,
    }
}

fn compact_tx(txid_length: usize) -> CompactTx {
    CompactTx {
        index: 0,
        txid: vec![0xcc; txid_length],
        ..Default::default()
    }
}
