use core::fmt;
use std::path::Path;
use std::sync::Mutex;

#[cfg(target_os = "linux")]
use std::fs;

#[cfg(target_os = "linux")]
use std::os::unix::fs::{MetadataExt, PermissionsExt};

use serde_json::Value;
use zeroize::Zeroizing;

#[cfg(target_os = "linux")]
use crate::xmr::account::current_uid;
use crate::xmr::account::{AccountCapabilities, AccountKind};
use crate::xmr::model::{DeviceState, NodeState, WalletState, XmrError, XmrNetwork};
use crate::xmr::rpc::{HardForkInfoResult, RpcMethod, TypedResult, parse_typed_result};
use crate::xmr::store::{
    AccountStore, DIRECTORY_MODE, PathSqliteSurface, ReceiverPersistenceProof, StoredIdentity,
    StoredReceiver,
};

pub use crate::xmr::store::ReceiverSchemaView;

pub const MAX_SUBADDRESS_INDEX: u32 = u32::MAX;
pub const MAX_ISSUANCE_SEQUENCE: i64 = i64::MAX;

const ASSET: &str = "XMR";
const PRIVACY: &str = "private";
const VIEW_FIELD_NAMES: [&str; 13] = [
    "account_id",
    "asset",
    "network",
    "kind",
    "privacy",
    "node_state",
    "wallet_state",
    "device_state",
    "node_height",
    "wallet_height",
    "balance_atomic",
    "unlocked_balance_atomic",
    "capabilities",
];
const CAPABILITY_FIELD_NAMES: [&str; 21] = [
    "can_view",
    "can_derive_fresh_receiver",
    "can_receive_private",
    "can_receive_orchard",
    "can_receive_ironwood",
    "can_prepare_tx",
    "can_sign_spend",
    "can_sign_orchard",
    "can_sign_ironwood",
    "can_tx_v6",
    "can_migrate_orchard_to_ironwood",
    "can_sign_transparent",
    "can_display_amount_on_device",
    "can_display_recipient_on_device",
    "can_display_network_on_device",
    "can_verify_pczt_on_device",
    "can_export_viewing_material",
    "can_broadcast",
    "consensus_branch",
    "pczt_version",
    "tx_version_max",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewCapabilities {
    pub can_view: bool,
    pub can_derive_fresh_receiver: bool,
    pub can_receive_private: bool,
    pub can_receive_orchard: bool,
    pub can_receive_ironwood: bool,
    pub can_prepare_tx: bool,
    pub can_sign_spend: bool,
    pub can_sign_orchard: bool,
    pub can_sign_ironwood: bool,
    pub can_tx_v6: bool,
    pub can_migrate_orchard_to_ironwood: bool,
    pub can_sign_transparent: bool,
    pub can_display_amount_on_device: bool,
    pub can_display_recipient_on_device: bool,
    pub can_display_network_on_device: bool,
    pub can_verify_pczt_on_device: bool,
    pub can_export_viewing_material: bool,
    pub can_broadcast: bool,
    pub consensus_branch: Option<String>,
    pub pczt_version: Option<String>,
    pub tx_version_max: Option<String>,
}

impl ViewCapabilities {
    pub fn xmr_viewing(consensus_branch: Option<String>) -> Self {
        let flags = AccountCapabilities::viewing_only();
        Self {
            can_view: flags.can_view,
            can_derive_fresh_receiver: flags.can_derive_fresh_receiver,
            can_receive_private: flags.can_receive_private,
            can_receive_orchard: flags.can_receive_orchard,
            can_receive_ironwood: flags.can_receive_ironwood,
            can_prepare_tx: flags.can_prepare_tx,
            can_sign_spend: flags.can_sign_spend,
            can_sign_orchard: flags.can_sign_orchard,
            can_sign_ironwood: flags.can_sign_ironwood,
            can_tx_v6: flags.can_tx_v6,
            can_migrate_orchard_to_ironwood: flags.can_migrate_orchard_to_ironwood,
            can_sign_transparent: flags.can_sign_transparent,
            can_display_amount_on_device: flags.can_display_amount_on_device,
            can_display_recipient_on_device: flags.can_display_recipient_on_device,
            can_display_network_on_device: flags.can_display_network_on_device,
            can_verify_pczt_on_device: flags.can_verify_pczt_on_device,
            can_export_viewing_material: flags.can_export_viewing_material,
            can_broadcast: flags.can_broadcast,
            consensus_branch,
            pczt_version: None,
            tx_version_max: None,
        }
    }

    pub fn field_names(&self) -> [&'static str; 21] {
        CAPABILITY_FIELD_NAMES
    }

    fn json(&self) -> serde_json::Value {
        serde_json::json!({
            "can_view": self.can_view,
            "can_derive_fresh_receiver": self.can_derive_fresh_receiver,
            "can_receive_private": self.can_receive_private,
            "can_receive_orchard": self.can_receive_orchard,
            "can_receive_ironwood": self.can_receive_ironwood,
            "can_prepare_tx": self.can_prepare_tx,
            "can_sign_spend": self.can_sign_spend,
            "can_sign_orchard": self.can_sign_orchard,
            "can_sign_ironwood": self.can_sign_ironwood,
            "can_tx_v6": self.can_tx_v6,
            "can_migrate_orchard_to_ironwood": self.can_migrate_orchard_to_ironwood,
            "can_sign_transparent": self.can_sign_transparent,
            "can_display_amount_on_device": self.can_display_amount_on_device,
            "can_display_recipient_on_device": self.can_display_recipient_on_device,
            "can_display_network_on_device": self.can_display_network_on_device,
            "can_verify_pczt_on_device": self.can_verify_pczt_on_device,
            "can_export_viewing_material": self.can_export_viewing_material,
            "can_broadcast": self.can_broadcast,
            "consensus_branch": self.consensus_branch,
            "pczt_version": self.pczt_version,
            "tx_version_max": self.tx_version_max,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SanitizedAccountView {
    pub account_id: String,
    pub asset: &'static str,
    pub network: String,
    pub kind: String,
    pub privacy: &'static str,
    pub node_state: &'static str,
    pub wallet_state: &'static str,
    pub device_state: &'static str,
    pub node_height: Option<String>,
    pub wallet_height: Option<String>,
    pub balance_atomic: String,
    pub unlocked_balance_atomic: String,
    pub capabilities: ViewCapabilities,
}

impl SanitizedAccountView {
    pub fn field_names(&self) -> [&'static str; 13] {
        VIEW_FIELD_NAMES
    }

    pub fn sanitized_json(&self) -> String {
        serde_json::json!({
            "account_id": self.account_id,
            "asset": self.asset,
            "network": self.network,
            "kind": self.kind,
            "privacy": self.privacy,
            "node_state": self.node_state,
            "wallet_state": self.wallet_state,
            "device_state": self.device_state,
            "node_height": self.node_height,
            "wallet_height": self.wallet_height,
            "balance_atomic": self.balance_atomic,
            "unlocked_balance_atomic": self.unlocked_balance_atomic,
            "capabilities": self.capabilities.json(),
        })
        .to_string()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct FreshReceiver {
    pub account_id: String,
    pub network: String,
    pub request_id: String,
    pub receiver: String,
    pub account_index: u32,
    pub subaddress_index: u32,
    pub issued_at_sequence: i64,
}

impl fmt::Debug for FreshReceiver {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FreshReceiver")
            .field("account_id", &self.account_id)
            .field("network", &self.network)
            .field("request_id", &"[REDACTED]")
            .field("receiver", &"[REDACTED]")
            .field("account_index", &self.account_index)
            .field("subaddress_index", &self.subaddress_index)
            .field("issued_at_sequence", &self.issued_at_sequence)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ViewSnapshotInput<'a> {
    pub account_id: &'a str,
    pub network: XmrNetwork,
    pub kind: AccountKind,
    pub node_state: NodeState,
    pub wallet_state: WalletState,
    pub node_height: Option<u64>,
    pub wallet_height: Option<u64>,
    pub total_atomic: u64,
    pub unlocked_atomic: u64,
    pub consensus_branch: Option<&'a str>,
}

pub fn validate_account_id(value: &str) -> Result<(), XmrError> {
    validate_hex_id(value)
}

pub fn validate_request_id(value: &str) -> Result<(), XmrError> {
    validate_hex_id(value)
}

pub fn validate_network(value: &str) -> Result<XmrNetwork, XmrError> {
    XmrNetwork::parse(value)
}

pub fn format_atomic(value: u64) -> String {
    value.to_string()
}

pub fn parse_canonical_atomic(value: &str) -> Result<u64, XmrError> {
    if value.is_empty()
        || !value.bytes().all(|byte| byte.is_ascii_digit())
        || (value.starts_with('0') && value != "0")
    {
        return Err(XmrError::protocol_incompatible());
    }
    value
        .parse::<u64>()
        .map_err(|_| XmrError::protocol_incompatible())
}

pub fn parse_balance_result(value: &Value) -> Result<(u64, u64), XmrError> {
    match parse_typed_result(value, RpcMethod::GetBalance)? {
        TypedResult::Balance { total, unlocked } => {
            if unlocked > total {
                Err(XmrError::protocol_incompatible())
            } else {
                Ok((total, unlocked))
            }
        }
        _ => Err(XmrError::protocol_incompatible()),
    }
}

pub(crate) fn parse_hard_fork_result(value: &Value) -> Result<HardForkInfoResult, XmrError> {
    match parse_typed_result(value, RpcMethod::HardForkInfo)? {
        TypedResult::HardForkInfo(info) => Ok(info),
        _ => Err(XmrError::protocol_incompatible()),
    }
}

pub(crate) fn validate_view_hard_fork(info: &HardForkInfoResult) -> Result<u8, XmrError> {
    if info.status != "OK" || !info.enabled || info.untrusted || info.version == 0 {
        Err(XmrError::protocol_incompatible())
    } else {
        Ok(info.version)
    }
}

pub fn sanitize_view(input: ViewSnapshotInput<'_>) -> Result<SanitizedAccountView, XmrError> {
    validate_account_id(input.account_id)?;
    if input.unlocked_atomic > input.total_atomic {
        return Err(XmrError::protocol_incompatible());
    }
    Ok(SanitizedAccountView {
        account_id: input.account_id.to_owned(),
        asset: ASSET,
        network: input.network.name().to_owned(),
        kind: input.kind.as_str().to_owned(),
        privacy: PRIVACY,
        node_state: input.node_state.as_str(),
        wallet_state: input.wallet_state.as_str(),
        device_state: DeviceState::NotApplicable.as_str(),
        node_height: input.node_height.map(format_atomic),
        wallet_height: input.wallet_height.map(format_atomic),
        balance_atomic: format_atomic(input.total_atomic),
        unlocked_balance_atomic: format_atomic(input.unlocked_atomic),
        capabilities: ViewCapabilities::xmr_viewing(input.consensus_branch.map(str::to_owned)),
    })
}

pub(crate) struct ProductionViewInput<'a> {
    pub account_id: &'a str,
    pub network: XmrNetwork,
    pub kind: AccountKind,
    pub node_state: NodeState,
    pub node_height: Option<u64>,
    pub wallet_available: bool,
    pub wallet_locked: bool,
    pub wallet_height: Option<u64>,
    pub total_atomic: Option<u64>,
    pub unlocked_atomic: Option<u64>,
    pub hard_fork: Option<&'a HardForkInfoResult>,
}

pub(crate) fn build_production_view(
    input: ProductionViewInput<'_>,
) -> Result<SanitizedAccountView, XmrError> {
    validate_account_id(input.account_id)?;
    let wallet_state = if input.wallet_locked {
        WalletState::Locked
    } else if !input.wallet_available {
        WalletState::Unavailable
    } else {
        let wallet_height = input
            .wallet_height
            .ok_or_else(XmrError::protocol_incompatible)?;
        match input.node_height {
            Some(node_height) if wallet_height < node_height => WalletState::Refreshing,
            Some(node_height) if wallet_height == node_height => WalletState::Ready,
            Some(_) => return Err(XmrError::protocol_incompatible()),
            None => WalletState::Ready,
        }
    };
    let (total_atomic, unlocked_atomic) = match wallet_state {
        WalletState::Ready | WalletState::Refreshing => (
            input
                .total_atomic
                .ok_or_else(XmrError::protocol_incompatible)?,
            input
                .unlocked_atomic
                .ok_or_else(XmrError::protocol_incompatible)?,
        ),
        WalletState::Unavailable | WalletState::Locked => (0, 0),
    };
    if unlocked_atomic > total_atomic {
        return Err(XmrError::protocol_incompatible());
    }
    let consensus_branch = input
        .hard_fork
        .map(validate_view_hard_fork)
        .transpose()?
        .map(|version| version.to_string());
    sanitize_view(ViewSnapshotInput {
        account_id: input.account_id,
        network: input.network,
        kind: input.kind,
        node_state: input.node_state,
        wallet_state,
        node_height: input.node_height,
        wallet_height: input.wallet_height,
        total_atomic,
        unlocked_atomic,
        consensus_branch: consensus_branch.as_deref(),
    })
}

pub(crate) fn expected_nettype(network: XmrNetwork) -> &'static str {
    match network {
        XmrNetwork::Stagenet => "stagenet",
        XmrNetwork::Testnet => "testnet",
    }
}

pub(crate) struct CreatedSubaddress {
    pub address: Zeroizing<String>,
    pub account_index: u32,
    pub subaddress_index: u32,
}

pub(crate) struct AddressClassification {
    pub valid: bool,
    pub integrated: bool,
    pub subaddress: bool,
    pub nettype: String,
}

pub(crate) trait ReceiverPort {
    fn rpc_calls(&self) -> Vec<String>;
    fn clear_rpc_calls(&mut self);
    fn lookup_binding(&mut self, request_id: &str) -> Result<Option<FreshReceiver>, XmrError>;
    fn lookup_all(&mut self) -> Result<Vec<FreshReceiver>, XmrError>;
    fn load_identity(&mut self) -> Result<StoredIdentity, XmrError>;
    fn create_address(&mut self) -> Result<CreatedSubaddress, XmrError>;
    fn validate_subaddress(
        &mut self,
        address: &str,
        network: XmrNetwork,
    ) -> Result<AddressClassification, XmrError>;
    fn get_indexed_address(
        &mut self,
        account_index: u32,
        address_index: u32,
    ) -> Result<Zeroizing<String>, XmrError>;
    fn persist_binding(
        &mut self,
        binding: &FreshReceiver,
    ) -> Result<ReceiverPersistenceProof, XmrError>;
    fn set_issuance_watermarks(&mut self, index: u64, sequence: u64) -> Result<(), XmrError>;
    fn greatest_sequence(&mut self) -> Result<i64, XmrError>;
    fn last_subaddress_index(&mut self) -> Result<u32, XmrError>;
    fn inspect_schema(&mut self) -> Result<ReceiverSchemaView, XmrError>;
    fn reopen(&mut self) -> Result<(), XmrError>;
    fn begin_create_address(&mut self);
    fn end_create_address(&mut self);
    fn max_in_flight_create_address(&self) -> usize;
    fn wallet_state(&self) -> WalletState;
    fn node_state(&self) -> NodeState;
    fn watch_only_initialization_failed(&self) -> bool;
    fn prove_owned_identity(&mut self) -> Result<(), XmrError>;
    fn prepare_receiver(&mut self) -> Result<(), XmrError>;
    fn latch_unavailable(&mut self);
    fn authority_unavailable(&self) -> bool;
}

pub(crate) struct ReceiverObserver {
    pub last_lookup_was_durable: bool,
    pub validate_reported_subaddress: bool,
    pub get_address_equal: bool,
    pub binding_committed_before_return: bool,
    pub binding_file_synced_before_return: bool,
    pub used_primary_or_stale_fallback: bool,
    pub reconstructed_from_wallet_output: bool,
    pub returned: Option<FreshReceiver>,
    pub returned_addresses: Vec<String>,
}

impl ReceiverObserver {
    pub(crate) fn new() -> Self {
        Self {
            last_lookup_was_durable: false,
            validate_reported_subaddress: false,
            get_address_equal: false,
            binding_committed_before_return: false,
            binding_file_synced_before_return: false,
            used_primary_or_stale_fallback: false,
            reconstructed_from_wallet_output: false,
            returned: None,
            returned_addresses: Vec::new(),
        }
    }

    pub fn address_was_reused(&self, address: &str) -> bool {
        self.returned_addresses
            .iter()
            .filter(|item| *item == address)
            .count()
            > 1
    }
}

pub(crate) struct ReceiverManager<P: ReceiverPort> {
    account_id: String,
    network: XmrNetwork,
    state: Mutex<ReceiverSession<P>>,
}

struct ReceiverSession<P: ReceiverPort> {
    port: P,
    observer: ReceiverObserver,
}

impl<P: ReceiverPort> ReceiverManager<P> {
    pub fn new(account_id: &str, network: XmrNetwork, port: P) -> Result<Self, XmrError> {
        validate_account_id(account_id)?;
        Ok(Self {
            account_id: account_id.to_owned(),
            network,
            state: Mutex::new(ReceiverSession {
                port,
                observer: ReceiverObserver::new(),
            }),
        })
    }

    pub fn fresh(
        &self,
        account_id: &str,
        network: &str,
        request_id: &str,
    ) -> Result<FreshReceiver, XmrError> {
        let mut session = self.state.lock().map_err(|_| XmrError::internal())?;
        session.observer.returned = None;
        session.observer.last_lookup_was_durable = false;
        session.observer.validate_reported_subaddress = false;
        session.observer.get_address_equal = false;
        session.observer.binding_committed_before_return = false;
        session.observer.binding_file_synced_before_return = false;
        session.observer.used_primary_or_stale_fallback = false;
        let ReceiverSession { port, observer } = &mut *session;
        issue_fresh(
            &self.account_id,
            self.network,
            account_id,
            network,
            request_id,
            port,
            observer,
        )
    }

    pub fn set_receiver_state(&self, index: u64, sequence: u64) -> Result<(), XmrError> {
        let mut session = self.state.lock().map_err(|_| XmrError::internal())?;
        session.port.set_issuance_watermarks(index, sequence)
    }

    pub fn inspect_schema(&self) -> Result<ReceiverSchemaView, XmrError> {
        let mut session = self.state.lock().map_err(|_| XmrError::internal())?;
        session.port.inspect_schema()
    }

    pub fn reopen(&self) -> Result<(), XmrError> {
        let mut session = self.state.lock().map_err(|_| XmrError::internal())?;
        session.observer.returned = None;
        session.observer.reconstructed_from_wallet_output = false;
        if let Err(error) = session.port.reopen() {
            if error.code() == "STATE_CORRUPT" {
                session.port.latch_unavailable();
            }
            return Err(error);
        }
        let identity = match session.port.load_identity() {
            Ok(identity) => identity,
            Err(error) => {
                if error.code() == "STATE_CORRUPT" {
                    session.port.latch_unavailable();
                }
                return Err(error);
            }
        };
        if identity.account_id() != self.account_id || identity.network() != self.network.name() {
            session.port.latch_unavailable();
            Err(XmrError::state_corrupt())
        } else {
            Ok(())
        }
    }

    pub fn observer(&self) -> Result<ReceiverObserver, XmrError> {
        let session = self.state.lock().map_err(|_| XmrError::internal())?;
        Ok(ReceiverObserver {
            last_lookup_was_durable: session.observer.last_lookup_was_durable,
            validate_reported_subaddress: session.observer.validate_reported_subaddress,
            get_address_equal: session.observer.get_address_equal,
            binding_committed_before_return: session.observer.binding_committed_before_return,
            binding_file_synced_before_return: session.observer.binding_file_synced_before_return,
            used_primary_or_stale_fallback: session.observer.used_primary_or_stale_fallback,
            reconstructed_from_wallet_output: session.observer.reconstructed_from_wallet_output,
            returned: session.observer.returned.clone(),
            returned_addresses: session.observer.returned_addresses.clone(),
        })
    }

    pub fn rpc_calls(&self) -> Result<Vec<String>, XmrError> {
        let session = self.state.lock().map_err(|_| XmrError::internal())?;
        Ok(session.port.rpc_calls())
    }

    pub fn with_port_mut<R>(&self, operation: impl FnOnce(&mut P) -> R) -> Result<R, XmrError> {
        let mut session = self.state.lock().map_err(|_| XmrError::internal())?;
        Ok(operation(&mut session.port))
    }

    pub fn clear_rpc_calls(&self) -> Result<(), XmrError> {
        let mut session = self.state.lock().map_err(|_| XmrError::internal())?;
        session.port.clear_rpc_calls();
        Ok(())
    }

    pub fn greatest_issuance_sequence(&self) -> Result<i64, XmrError> {
        let mut session = self.state.lock().map_err(|_| XmrError::internal())?;
        session.port.greatest_sequence()
    }

    pub fn max_in_flight_create_address(&self) -> Result<usize, XmrError> {
        let session = self.state.lock().map_err(|_| XmrError::internal())?;
        Ok(session.port.max_in_flight_create_address())
    }

    pub fn persisted_bindings(&self) -> Result<Vec<FreshReceiver>, XmrError> {
        let mut session = self.state.lock().map_err(|_| XmrError::internal())?;
        session.port.lookup_all()
    }
}

pub(crate) fn issue_fresh_with_port<P: ReceiverPort>(
    owned_account_id: &str,
    owned_network: XmrNetwork,
    account_id: &str,
    network: &str,
    request_id: &str,
    port: &mut P,
) -> Result<FreshReceiver, XmrError> {
    let mut observer = ReceiverObserver::new();
    issue_fresh(
        owned_account_id,
        owned_network,
        account_id,
        network,
        request_id,
        port,
        &mut observer,
    )
}

fn issue_fresh<P: ReceiverPort>(
    owned_account_id: &str,
    owned_network: XmrNetwork,
    account_id: &str,
    network: &str,
    request_id: &str,
    port: &mut P,
    observer: &mut ReceiverObserver,
) -> Result<FreshReceiver, XmrError> {
    validate_account_id(account_id)?;
    validate_request_id(request_id)?;
    let network = validate_network(network)?;
    if account_id != owned_account_id {
        return Err(XmrError::request_schema());
    }
    if network != owned_network {
        return Err(XmrError::wrong_network());
    }
    if port.authority_unavailable() {
        return Err(XmrError::state_corrupt());
    }
    let identity = match port.load_identity() {
        Ok(identity) => identity,
        Err(error) => {
            if error.code() == "STATE_CORRUPT" {
                port.latch_unavailable();
            }
            return Err(error);
        }
    };
    if identity.account_id() != owned_account_id || identity.network() != owned_network.name() {
        port.latch_unavailable();
        return Err(XmrError::state_corrupt());
    }
    let existing = match port.lookup_binding(request_id) {
        Ok(existing) => existing,
        Err(error) => {
            if error.code() == "STATE_CORRUPT" {
                port.latch_unavailable();
            }
            return Err(error);
        }
    };
    if let Some(existing) = existing {
        observer.last_lookup_was_durable = true;
        observer.returned = Some(existing.clone());
        return Ok(existing);
    }
    port.prove_owned_identity()?;
    port.prepare_receiver()?;
    reject_ineligible(port)?;
    if let Err(error) = reject_exhaustion(port) {
        if error.code() == "STATE_CORRUPT" {
            port.latch_unavailable();
        }
        return Err(error);
    }
    let primary = identity.primary_address()?;
    port.begin_create_address();
    let created = port.create_address();
    port.end_create_address();
    let created = created?;
    if created.account_index != 0
        || created.subaddress_index == 0
        || created.address.as_str() == primary.as_str()
    {
        return Err(XmrError::protocol_incompatible());
    }
    let classification = port.validate_subaddress(created.address.as_str(), owned_network)?;
    if !classification.valid
        || classification.integrated
        || !classification.subaddress
        || classification.nettype != expected_nettype(owned_network)
    {
        return Err(XmrError::protocol_incompatible());
    }
    observer.validate_reported_subaddress = true;
    let confirmed = port.get_indexed_address(created.account_index, created.subaddress_index)?;
    if confirmed.as_str() != created.address.as_str() || confirmed.as_str() == primary.as_str() {
        return Err(XmrError::protocol_incompatible());
    }
    observer.get_address_equal = true;
    let next_sequence = identity
        .greatest_issuance_sequence()
        .checked_add(1)
        .ok_or_else(XmrError::limit)?;
    let issued = FreshReceiver {
        account_id: owned_account_id.to_owned(),
        network: owned_network.name().to_owned(),
        request_id: request_id.to_owned(),
        receiver: created.address.to_string(),
        account_index: created.account_index,
        subaddress_index: created.subaddress_index,
        issued_at_sequence: next_sequence,
    };
    let proof = match port.persist_binding(&issued) {
        Ok(proof) => proof,
        Err(_) => {
            port.latch_unavailable();
            return Err(XmrError::state_corrupt());
        }
    };
    if !proof.durable_and_proved() {
        port.latch_unavailable();
        return Err(XmrError::state_corrupt());
    }
    observer.binding_committed_before_return = proof.committed();
    observer.binding_file_synced_before_return = proof.durable_and_proved();
    observer.returned_addresses.push(issued.receiver.clone());
    observer.returned = Some(issued.clone());
    Ok(issued)
}

fn reject_ineligible<P: ReceiverPort>(port: &P) -> Result<(), XmrError> {
    if port.watch_only_initialization_failed() {
        return Err(XmrError::watch_only());
    }
    match port.wallet_state() {
        WalletState::Locked => return Err(XmrError::locked()),
        WalletState::Refreshing | WalletState::Unavailable => {
            return Err(XmrError::unavailable());
        }
        WalletState::Ready => {}
    }
    match port.node_state() {
        NodeState::Syncing => return Err(XmrError::node_syncing()),
        NodeState::Unavailable => return Err(XmrError::node_unavailable()),
        NodeState::Ready => {}
    }
    Ok(())
}

fn reject_exhaustion<P: ReceiverPort>(port: &mut P) -> Result<(), XmrError> {
    let sequence = port.greatest_sequence()?;
    let index = port.last_subaddress_index()?;
    if sequence == MAX_ISSUANCE_SEQUENCE || index == MAX_SUBADDRESS_INDEX {
        Err(XmrError::limit())
    } else if sequence < 0 {
        Err(XmrError::state_corrupt())
    } else {
        Ok(())
    }
}

fn validate_hex_id(value: &str) -> Result<(), XmrError> {
    if value.len() == 32
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        Ok(())
    } else {
        Err(XmrError::request_schema())
    }
}

pub(crate) fn binding_from_stored(
    account_id: &str,
    network: XmrNetwork,
    stored: &StoredReceiver,
) -> Result<FreshReceiver, XmrError> {
    Ok(FreshReceiver {
        account_id: account_id.to_owned(),
        network: network.name().to_owned(),
        request_id: stored.request_id().to_owned(),
        receiver: stored.subaddress_text()?.to_string(),
        account_index: stored.account_index(),
        subaddress_index: stored.subaddress_index(),
        issued_at_sequence: stored.issued_at_sequence(),
    })
}

pub(crate) fn stored_from_binding(binding: &FreshReceiver) -> Result<StoredReceiver, XmrError> {
    StoredReceiver::new(
        binding.request_id.clone(),
        binding.account_index,
        binding.subaddress_index,
        &binding.receiver,
        binding.issued_at_sequence,
    )
}

pub(crate) fn initialize_receiver_store(
    root: &Path,
    identity: &StoredIdentity,
) -> Result<AccountStore<PathSqliteSurface>, XmrError> {
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (root, identity);
        Err(XmrError::unavailable())
    }
    #[cfg(target_os = "linux")]
    {
        let owner = current_uid()?;
        create_owned_private_directory(root, owner)?;
        let file = root.join("state.sqlite");
        let handle = PathSqliteSurface::exclusive_create_file(&file, owner)?;
        let file_identity = PathSqliteSurface::created_file_identity(&handle)?;
        let surface = PathSqliteSurface::bind_created(root, owner, handle, file_identity)?;
        let mut store = AccountStore::new(surface);
        store.initialize()?;
        store.persist_identity(identity)?;
        Ok(store)
    }
}

pub(crate) fn open_receiver_store(
    root: &Path,
) -> Result<AccountStore<PathSqliteSurface>, XmrError> {
    #[cfg(not(target_os = "linux"))]
    {
        let _ = root;
        Err(XmrError::unavailable())
    }
    #[cfg(target_os = "linux")]
    {
        let owner = current_uid()?;
        let surface = PathSqliteSurface::open_existing(root, owner)?;
        AccountStore::attach_existing(surface)
    }
}

pub(crate) fn kind_from_identity(identity: &StoredIdentity) -> Result<AccountKind, XmrError> {
    AccountKind::from_code(identity.kind())
}

#[cfg(target_os = "linux")]
fn create_owned_private_directory(path: &Path, owner: u32) -> Result<(), XmrError> {
    fs::create_dir(path).map_err(|_| XmrError::internal())?;
    fs::set_permissions(path, fs::Permissions::from_mode(DIRECTORY_MODE))
        .map_err(|_| XmrError::internal())?;
    let metadata = fs::symlink_metadata(path).map_err(|_| XmrError::state_corrupt())?;
    if metadata.file_type().is_symlink()
        || !metadata.file_type().is_dir()
        || metadata.uid() != owner
        || metadata.permissions().mode() & 0o777 != DIRECTORY_MODE
    {
        return Err(XmrError::state_corrupt());
    }
    Ok(())
}
