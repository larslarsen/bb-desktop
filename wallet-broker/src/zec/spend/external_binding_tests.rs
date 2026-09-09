use pczt::roles::signer::SpendAuthSignature;

use super::{RetainedSpendBinding, TaggedContribution, validate_external_binding};

const SLOT_ZERO_KEY: [u8; 32] = [0x11; 32];
const SLOT_ONE_KEY: [u8; 32] = [0x12; 32];
const NULLIFIER: [u8; 32] = [0x22; 32];
const NONZERO_SIGNATURE: [u8; 64] = [0x5a; 64];
const ZERO_SIGNATURE: [u8; 64] = [0; 64];
const OTHER_KEY: [u8; 32] = [0xee; 32];

fn binding_for_slot(action_index: usize) -> RetainedSpendBinding {
    RetainedSpendBinding {
        action_index,
        randomized_key: if action_index == 0 {
            SLOT_ZERO_KEY
        } else {
            SLOT_ONE_KEY
        },
        nullifier: NULLIFIER,
        value_zat: 1,
    }
}

fn contribution(
    pool: orchard::ValuePool,
    action_index: usize,
    randomized_key: [u8; 32],
    signature: [u8; 64],
) -> TaggedContribution {
    TaggedContribution {
        signature: SpendAuthSignature::from_parts(pool, action_index, signature),
        randomized_key,
    }
}

fn matching_contribution(binding: &RetainedSpendBinding) -> TaggedContribution {
    contribution(
        orchard::ValuePool::Ironwood,
        binding.action_index,
        binding.randomized_key,
        NONZERO_SIGNATURE,
    )
}

fn assert_signature_invalid(contribution: TaggedContribution, binding: &RetainedSpendBinding) {
    match validate_external_binding(&contribution, binding) {
        Err(error) => assert_eq!(error.code(), "SIGNATURE_INVALID"),
        Ok(()) => panic!("misbound external contribution was accepted"),
    }
}

#[test]
fn accepts_matching_retained_slot_zero_and_one() {
    let slot_zero = binding_for_slot(0);
    assert!(validate_external_binding(&matching_contribution(&slot_zero), &slot_zero).is_ok());
    let slot_one = binding_for_slot(1);
    assert!(validate_external_binding(&matching_contribution(&slot_one), &slot_one).is_ok());
}

#[test]
fn rejects_misbound_external_contribution() {
    let slot_zero = binding_for_slot(0);
    let slot_one = binding_for_slot(1);

    for binding in [&slot_zero, &slot_one] {
        let other_slot = 1 - binding.action_index;
        assert_signature_invalid(
            contribution(
                orchard::ValuePool::Ironwood,
                other_slot,
                binding.randomized_key,
                NONZERO_SIGNATURE,
            ),
            binding,
        );
        assert_signature_invalid(
            contribution(
                orchard::ValuePool::Ironwood,
                2,
                binding.randomized_key,
                NONZERO_SIGNATURE,
            ),
            binding,
        );
        assert_signature_invalid(
            contribution(
                orchard::ValuePool::Orchard,
                binding.action_index,
                binding.randomized_key,
                NONZERO_SIGNATURE,
            ),
            binding,
        );
        assert_signature_invalid(
            contribution(
                orchard::ValuePool::Ironwood,
                binding.action_index,
                OTHER_KEY,
                NONZERO_SIGNATURE,
            ),
            binding,
        );
        assert_signature_invalid(
            contribution(
                orchard::ValuePool::Ironwood,
                binding.action_index,
                binding.randomized_key,
                ZERO_SIGNATURE,
            ),
            binding,
        );
    }
}
