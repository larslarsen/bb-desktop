use crate::vault::{SecretBytes, WipeObserver};
use zcash_keys::address::Address;
use zcash_keys::keys::{UnifiedAddressRequest, UnifiedFullViewingKey, UnifiedSpendingKey};
use zcash_protocol::consensus::Parameters;

use super::{Network, ZecError};

pub(crate) fn derive_ufvk(
    network: Network,
    seed: &mut SecretBytes,
    observer: &mut dyn WipeObserver,
) -> Result<String, ZecError> {
    let guard = SeedWipeGuard::new(seed, observer);
    match network {
        Network::Testnet => {
            derive_ufvk_for(&zcash_protocol::consensus::Network::TestNetwork, guard)
        }
        Network::Local(local) => derive_ufvk_for(&local.upstream(), guard),
    }
}

fn derive_ufvk_for<P: Parameters>(
    params: &P,
    guard: SeedWipeGuard<'_>,
) -> Result<String, ZecError> {
    guard.seed().expose(|bytes| {
        if bytes.len() != 32 {
            return Err(ZecError::schema());
        }
        let spending = UnifiedSpendingKey::from_seed(params, bytes, Default::default())
            .map_err(|_| ZecError::internal())?;
        let viewing = spending.to_unified_full_viewing_key();
        Ok(viewing.encode(params))
    })
}

pub(crate) fn derive_orchard_receiver(
    network: Network,
    encoded_ufvk: &str,
    index: u64,
) -> Result<String, ZecError> {
    match network {
        Network::Testnet => derive_receiver_for(
            &zcash_protocol::consensus::Network::TestNetwork,
            encoded_ufvk,
            index,
        ),
        Network::Local(local) => derive_receiver_for(&local.upstream(), encoded_ufvk, index),
    }
}

fn derive_receiver_for<P: Parameters>(
    params: &P,
    encoded_ufvk: &str,
    index: u64,
) -> Result<String, ZecError> {
    let viewing = UnifiedFullViewingKey::decode(params, encoded_ufvk)
        .map_err(|_| ZecError::state_corrupt())?;
    let (address, actual_index) = viewing
        .find_address(index.into(), UnifiedAddressRequest::ORCHARD)
        .map_err(|_| ZecError::protocol_incompatible())?;
    let actual_index = u64::try_from(actual_index).map_err(|_| ZecError::state_corrupt())?;
    if actual_index != index {
        return Err(ZecError::state_corrupt());
    }
    Ok(address.encode(params))
}

pub(crate) fn validate_composition(composition: &str) -> Result<(), ZecError> {
    match composition {
        "orchard" => Ok(()),
        "p2pkh" | "p2sh" | "orchard+p2pkh" => Err(ZecError::transparent_downgrade()),
        _ => Err(ZecError::protocol_incompatible()),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DecodedAddress {
    pub network: Network,
    pub receivers: Vec<DecodedReceiver>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DecodedReceiver {
    Orchard,
    P2pkh,
    P2sh,
    Sapling,
    Tex,
    Unknown,
}

pub(crate) fn decode_unified_address(encoded: &str) -> Result<DecodedAddress, ZecError> {
    if let Some(decoded) = decode_for(
        &zcash_protocol::consensus::Network::TestNetwork,
        Network::Testnet,
        encoded,
    ) {
        return Ok(decoded);
    }
    let local = super::LocalNetwork::new(1, 1, 1)?;
    decode_for(&local.upstream(), Network::Local(local), encoded).ok_or_else(ZecError::schema)
}

fn decode_for<P: Parameters>(
    params: &P,
    network: Network,
    encoded: &str,
) -> Option<DecodedAddress> {
    match Address::decode(params, encoded)? {
        Address::Unified(address) => {
            let mut receivers = Vec::new();
            if address.has_orchard() {
                receivers.push(DecodedReceiver::Orchard);
            }
            if address.has_sapling() {
                receivers.push(DecodedReceiver::Sapling);
            }
            if address.has_transparent() {
                receivers.push(DecodedReceiver::Unknown);
            }
            receivers.extend(address.unknown().iter().map(|_| DecodedReceiver::Unknown));
            Some(DecodedAddress { network, receivers })
        }
        _ => None,
    }
}

#[derive(Clone, Copy)]
pub(crate) enum SeedExit {
    Success,
    Error,
    Cancellation,
    Replacement,
    Unwind,
    Drop,
}

pub(crate) fn exercise_seed_exit(
    network: Network,
    seed: &mut SecretBytes,
    observer: &mut dyn WipeObserver,
    exit: SeedExit,
) -> Result<(), ZecError> {
    if matches!(exit, SeedExit::Replacement) {
        seed.replace(Vec::new(), "zec-seed", observer)
            .map_err(|_| ZecError::internal())?;
        return Err(ZecError::internal());
    }

    let guard = SeedWipeGuard::new(seed, observer);
    match exit {
        SeedExit::Success => {
            derive_only(network, &guard)?;
            Ok(())
        }
        SeedExit::Error | SeedExit::Cancellation => Err(ZecError::internal()),
        SeedExit::Unwind => panic!("controlled seed-owner unwind"),
        SeedExit::Drop => Ok(()),
        SeedExit::Replacement => unreachable!(),
    }
}

fn derive_only(network: Network, guard: &SeedWipeGuard<'_>) -> Result<(), ZecError> {
    match network {
        Network::Testnet => {
            derive_only_for(&zcash_protocol::consensus::Network::TestNetwork, guard)
        }
        Network::Local(local) => derive_only_for(&local.upstream(), guard),
    }
}

fn derive_only_for<P: Parameters>(params: &P, guard: &SeedWipeGuard<'_>) -> Result<(), ZecError> {
    guard.seed().expose(|bytes| {
        if bytes.len() != 32 {
            return Err(ZecError::schema());
        }
        let spending = UnifiedSpendingKey::from_seed(params, bytes, Default::default())
            .map_err(|_| ZecError::internal())?;
        let _viewing = spending.to_unified_full_viewing_key();
        Ok(())
    })
}

struct SeedWipeGuard<'a> {
    seed: &'a mut SecretBytes,
    observer: &'a mut dyn WipeObserver,
}

impl<'a> SeedWipeGuard<'a> {
    fn new(seed: &'a mut SecretBytes, observer: &'a mut dyn WipeObserver) -> Self {
        Self { seed, observer }
    }

    fn seed(&self) -> &SecretBytes {
        self.seed
    }
}

impl Drop for SeedWipeGuard<'_> {
    fn drop(&mut self) {
        self.seed.wipe_with("zec-seed", self.observer);
    }
}
