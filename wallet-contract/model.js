'use strict';

function outcome(account, canReceive, canSpend, errorCode) {
  return {
    can_receive: canReceive,
    can_spend: canSpend,
    error_code: errorCode,
    signer_kind: account.kind,
    privacy: account.privacy,
  };
}

function evaluateCapability(account, request) {
  const capabilities = account && account.capabilities ? account.capabilities : {};
  if (!account || !request || !['software', 'hardware_backed', 'watch_only'].includes(account.kind)) {
    return outcome(account || {}, false, false, 'CAPABILITY_MISSING');
  }
  const networks = account.asset === 'ZEC'
    ? ['zec-mainnet', 'zec-testnet', 'zec-regtest']
    : account.asset === 'XMR'
      ? ['xmr-mainnet', 'xmr-stagenet', 'xmr-testnet']
      : [];
  if (
    networks.length === 0 ||
    !networks.includes(account.network) ||
    account.asset !== request.asset ||
    account.network !== request.network
  ) {
    return outcome(account || {}, false, false, 'WRONG_NETWORK');
  }

  const isHardware = account.kind === 'hardware_backed';
  if (isHardware && account.device_present !== true) {
    return outcome(account, false, false, 'DEVICE_DISCONNECTED');
  }
  if (isHardware && (!account.probed_at || !account.probe_source)) {
    return outcome(account, false, false, 'CAPABILITY_MISSING');
  }

  const privateReceive = account.privacy === 'private' &&
    capabilities.can_receive_private === true &&
    capabilities.can_derive_fresh_receiver === true;

  if (account.asset === 'ZEC') {
    const canReceive = privateReceive && capabilities.can_receive_ironwood === true;
    if (capabilities.consensus_branch !== 'nu6.3-test-fixture') {
      return outcome(account, false, false, 'PROTOCOL_INCOMPATIBLE');
    }
    if (account.kind === 'watch_only') return outcome(account, canReceive, false, 'WATCH_ONLY');
    if (!canReceive) return outcome(account, false, false, 'CAPABILITY_MISSING');
    if (capabilities.can_tx_v6 !== true || capabilities.tx_version_max !== '6') {
      return outcome(account, true, false, 'PROTOCOL_INCOMPATIBLE');
    }
    if (capabilities.pczt_version !== 'v6-fixture') {
      return outcome(account, true, false, 'PROTOCOL_INCOMPATIBLE');
    }
    if (account.restored_pool === 'orchard') return outcome(account, true, false, 'MIGRATION_REQUIRED');
    if (
      capabilities.can_prepare_tx !== true ||
      capabilities.can_sign_spend !== true ||
      capabilities.can_sign_ironwood !== true ||
      (isHardware && capabilities.can_verify_pczt_on_device !== true)
    ) {
      return outcome(account, true, false, 'CAPABILITY_MISSING');
    }
    return outcome(account, true, true, null);
  }

  if (account.asset === 'XMR') {
    const canReceive = privateReceive;
    if (capabilities.consensus_branch !== 'xmr-fixture-hf') {
      return outcome(account, false, false, 'PROTOCOL_INCOMPATIBLE');
    }
    if (account.kind === 'watch_only') return outcome(account, canReceive, false, 'WATCH_ONLY');
    if (!canReceive || capabilities.can_prepare_tx !== true || capabilities.can_sign_spend !== true) {
      return outcome(account, canReceive, false, 'CAPABILITY_MISSING');
    }
    return outcome(account, true, true, null);
  }

  return outcome(account, false, false, 'CAPABILITY_MISSING');
}

module.exports = {
  evaluateCapability,
};
