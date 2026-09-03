use core::fmt;

use sha2::{Digest, Sha256};

const FINGERPRINT_DOMAIN: &[u8] = b"bitbook.zec.hardware.fingerprint.v1";
const REQUIRED_BRANCH: &str = "37a5165b";
const REQUIRED_TRANSACTION_VERSION: &str = "6";
const REQUIRED_PCZT_ENCODING_VERSION: &str = "2";

pub static PRODUCTION_REVIEWED_PROFILES: &[ReviewedProfile] = &[];

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeviceVendor {
    Keystone,
    Ledger,
    Trezor,
}

impl DeviceVendor {
    fn fingerprint_bytes(self) -> &'static [u8] {
        match self {
            Self::Keystone => b"keystone",
            Self::Ledger => b"ledger",
            Self::Trezor => b"trezor",
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct FingerprintComponent(String);

impl FingerprintComponent {
    fn parse(value: &str) -> Result<Self, HardwareError> {
        if (1..=64).contains(&value.len())
            && value.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'+' | b'-')
            })
        {
            Ok(Self(value.to_owned()))
        } else {
            Err(HardwareError::schema())
        }
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for FingerprintComponent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FingerprintComponent([REDACTED])")
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct DeviceFingerprint {
    pub(crate) vendor: DeviceVendor,
    model: FingerprintComponent,
    app_name: FingerprintComponent,
    app_version: FingerprintComponent,
}

impl DeviceFingerprint {
    pub(crate) fn new(
        vendor: DeviceVendor,
        model: &str,
        app_name: &str,
        app_version: &str,
    ) -> Result<Self, HardwareError> {
        Ok(Self {
            vendor,
            model: FingerprintComponent::parse(model)?,
            app_name: FingerprintComponent::parse(app_name)?,
            app_version: FingerprintComponent::parse(app_version)?,
        })
    }

    pub fn vendor(&self) -> DeviceVendor {
        self.vendor
    }

    pub fn model(&self) -> &str {
        self.model.as_str()
    }

    pub fn app_name(&self) -> &str {
        self.app_name.as_str()
    }

    pub fn app_version(&self) -> &str {
        self.app_version.as_str()
    }

    pub(crate) fn digest(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(FINGERPRINT_DOMAIN);
        update_length_prefixed(&mut hasher, self.vendor.fingerprint_bytes());
        update_length_prefixed(&mut hasher, self.model.as_str().as_bytes());
        update_length_prefixed(&mut hasher, self.app_name.as_str().as_bytes());
        update_length_prefixed(&mut hasher, self.app_version.as_str().as_bytes());
        lowercase_hex(&hasher.finalize())
    }

    pub(crate) fn replacing_vendor(&self, vendor: DeviceVendor) -> Self {
        let mut replaced = self.clone();
        replaced.vendor = vendor;
        replaced
    }

    pub(crate) fn replacing_component(
        &self,
        field: FingerprintField,
        value: &str,
    ) -> Result<Self, HardwareError> {
        let component = FingerprintComponent::parse(value)?;
        let mut replaced = self.clone();
        match field {
            FingerprintField::Model => replaced.model = component,
            FingerprintField::AppName => replaced.app_name = component,
            FingerprintField::AppVersion => replaced.app_version = component,
        }
        Ok(replaced)
    }
}

impl fmt::Debug for DeviceFingerprint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeviceFingerprint")
            .field("vendor", &self.vendor)
            .field("components", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FingerprintField {
    Model,
    AppName,
    AppVersion,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SigningPool {
    Transparent,
    Orchard,
    Ironwood,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum VerifiedField {
    Amount,
    Recipient,
    Network,
    Fee,
    Memo,
}

impl VerifiedField {
    pub(crate) const ALL: [Self; 5] = [
        Self::Amount,
        Self::Recipient,
        Self::Network,
        Self::Fee,
        Self::Memo,
    ];

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Amount => "amount",
            Self::Recipient => "recipient",
            Self::Network => "network",
            Self::Fee => "fee",
            Self::Memo => "memo",
        }
    }
}

impl SigningPool {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Transparent => "transparent",
            Self::Orchard => "orchard",
            Self::Ironwood => "ironwood",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaimedRoute {
    Software,
    OtherDevice,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityFlag {
    CanView,
    CanDeriveFreshReceiver,
    CanReceivePrivate,
    CanReceiveOrchard,
    CanReceiveIronwood,
    CanPrepareTx,
    CanSignSpend,
    CanSignOrchard,
    CanSignIronwood,
    CanTxV6,
    CanMigrateOrchardToIronwood,
    CanSignTransparent,
    CanDisplayAmountOnDevice,
    CanDisplayRecipientOnDevice,
    CanDisplayNetworkOnDevice,
    CanVerifyPcztOnDevice,
    CanExportViewingMaterial,
    CanBroadcast,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct HardwareCapabilities {
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
    pub transaction_version: String,
    pub consensus_branch: String,
    pub pczt_encoding_version: String,
    pub allowed_signing_pools: Vec<SigningPool>,
}

impl HardwareCapabilities {
    pub fn contains(&self, flag: CapabilityFlag) -> bool {
        match flag {
            CapabilityFlag::CanView => self.can_view,
            CapabilityFlag::CanDeriveFreshReceiver => self.can_derive_fresh_receiver,
            CapabilityFlag::CanReceivePrivate => self.can_receive_private,
            CapabilityFlag::CanReceiveOrchard => self.can_receive_orchard,
            CapabilityFlag::CanReceiveIronwood => self.can_receive_ironwood,
            CapabilityFlag::CanPrepareTx => self.can_prepare_tx,
            CapabilityFlag::CanSignSpend => self.can_sign_spend,
            CapabilityFlag::CanSignOrchard => self.can_sign_orchard,
            CapabilityFlag::CanSignIronwood => self.can_sign_ironwood,
            CapabilityFlag::CanTxV6 => self.can_tx_v6,
            CapabilityFlag::CanMigrateOrchardToIronwood => self.can_migrate_orchard_to_ironwood,
            CapabilityFlag::CanSignTransparent => self.can_sign_transparent,
            CapabilityFlag::CanDisplayAmountOnDevice => self.can_display_amount_on_device,
            CapabilityFlag::CanDisplayRecipientOnDevice => self.can_display_recipient_on_device,
            CapabilityFlag::CanDisplayNetworkOnDevice => self.can_display_network_on_device,
            CapabilityFlag::CanVerifyPcztOnDevice => self.can_verify_pczt_on_device,
            CapabilityFlag::CanExportViewingMaterial => self.can_export_viewing_material,
            CapabilityFlag::CanBroadcast => self.can_broadcast,
        }
    }

    pub(crate) fn set(&mut self, flag: CapabilityFlag, value: bool) {
        match flag {
            CapabilityFlag::CanView => self.can_view = value,
            CapabilityFlag::CanDeriveFreshReceiver => self.can_derive_fresh_receiver = value,
            CapabilityFlag::CanReceivePrivate => self.can_receive_private = value,
            CapabilityFlag::CanReceiveOrchard => self.can_receive_orchard = value,
            CapabilityFlag::CanReceiveIronwood => self.can_receive_ironwood = value,
            CapabilityFlag::CanPrepareTx => self.can_prepare_tx = value,
            CapabilityFlag::CanSignSpend => self.can_sign_spend = value,
            CapabilityFlag::CanSignOrchard => self.can_sign_orchard = value,
            CapabilityFlag::CanSignIronwood => self.can_sign_ironwood = value,
            CapabilityFlag::CanTxV6 => self.can_tx_v6 = value,
            CapabilityFlag::CanMigrateOrchardToIronwood => {
                self.can_migrate_orchard_to_ironwood = value;
            }
            CapabilityFlag::CanSignTransparent => self.can_sign_transparent = value,
            CapabilityFlag::CanDisplayAmountOnDevice => {
                self.can_display_amount_on_device = value;
            }
            CapabilityFlag::CanDisplayRecipientOnDevice => {
                self.can_display_recipient_on_device = value;
            }
            CapabilityFlag::CanDisplayNetworkOnDevice => {
                self.can_display_network_on_device = value;
            }
            CapabilityFlag::CanVerifyPcztOnDevice => self.can_verify_pczt_on_device = value,
            CapabilityFlag::CanExportViewingMaterial => self.can_export_viewing_material = value,
            CapabilityFlag::CanBroadcast => self.can_broadcast = value,
        }
    }

    fn intersection(reviewed: &Self, live: &Self) -> Self {
        let mut narrowed = Self::default();
        for flag in ALL_CAPABILITY_FLAGS {
            narrowed.set(flag, reviewed.contains(flag) && live.contains(flag));
        }
        narrowed.transaction_version = live.transaction_version.clone();
        narrowed.consensus_branch = live.consensus_branch.clone();
        narrowed.pczt_encoding_version = live.pczt_encoding_version.clone();
        narrowed.allowed_signing_pools = reviewed
            .allowed_signing_pools
            .iter()
            .copied()
            .filter(|pool| live.allowed_signing_pools.contains(pool))
            .collect();
        narrowed
    }

    fn clear_private_authority(&mut self) {
        self.can_prepare_tx = false;
        self.can_sign_spend = false;
        self.can_sign_orchard = false;
        self.can_sign_ironwood = false;
        self.can_tx_v6 = false;
        self.can_migrate_orchard_to_ironwood = false;
        self.can_verify_pczt_on_device = false;
        self.allowed_signing_pools.clear();
    }
}

pub(crate) const ALL_CAPABILITY_FLAGS: [CapabilityFlag; 18] = [
    CapabilityFlag::CanView,
    CapabilityFlag::CanDeriveFreshReceiver,
    CapabilityFlag::CanReceivePrivate,
    CapabilityFlag::CanReceiveOrchard,
    CapabilityFlag::CanReceiveIronwood,
    CapabilityFlag::CanPrepareTx,
    CapabilityFlag::CanSignSpend,
    CapabilityFlag::CanSignOrchard,
    CapabilityFlag::CanSignIronwood,
    CapabilityFlag::CanTxV6,
    CapabilityFlag::CanMigrateOrchardToIronwood,
    CapabilityFlag::CanSignTransparent,
    CapabilityFlag::CanDisplayAmountOnDevice,
    CapabilityFlag::CanDisplayRecipientOnDevice,
    CapabilityFlag::CanDisplayNetworkOnDevice,
    CapabilityFlag::CanVerifyPcztOnDevice,
    CapabilityFlag::CanExportViewingMaterial,
    CapabilityFlag::CanBroadcast,
];

#[derive(Clone, Eq, PartialEq)]
pub struct ReviewedProfile {
    pub(crate) fingerprint: DeviceFingerprint,
    pub(crate) table_revision: String,
    pub(crate) capabilities: HardwareCapabilities,
    pub(crate) verified_fields: Vec<VerifiedField>,
    pub(crate) test_only: bool,
}

impl ReviewedProfile {
    pub(crate) fn from_parts(
        fingerprint: DeviceFingerprint,
        table_revision: &str,
        capabilities: HardwareCapabilities,
        verified_fields: Vec<VerifiedField>,
        test_only: bool,
    ) -> Result<Self, HardwareError> {
        if table_revision.is_empty()
            || table_revision.len() > 64
            || !table_revision.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'+' | b'-')
            })
            || !valid_branch_claim(&capabilities.consensus_branch)
            || !valid_version_claim(&capabilities.transaction_version)
            || !valid_version_claim(&capabilities.pczt_encoding_version)
            || has_duplicates(&capabilities.allowed_signing_pools)
            || has_duplicates(&verified_fields)
        {
            return Err(HardwareError::schema());
        }
        Ok(Self {
            fingerprint,
            table_revision: table_revision.to_owned(),
            capabilities,
            verified_fields,
            test_only,
        })
    }

    pub fn fingerprint(&self) -> &DeviceFingerprint {
        &self.fingerprint
    }

    pub fn table_revision(&self) -> &str {
        &self.table_revision
    }

    pub fn is_test_only(&self) -> bool {
        self.test_only
    }
}

impl fmt::Debug for ReviewedProfile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ReviewedProfile")
            .field("fingerprint_digest", &self.fingerprint.digest())
            .field("table_revision", &self.table_revision)
            .field("capabilities", &self.capabilities)
            .field("verified_fields", &self.verified_fields)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct LiveProbe {
    pub(crate) present: bool,
    pub(crate) capabilities: HardwareCapabilities,
    pub(crate) verified_fields: Vec<VerifiedField>,
    pub(crate) claimed_routes: Vec<ClaimedRoute>,
}

impl LiveProbe {
    pub(crate) fn from_parts(
        present: bool,
        capabilities: HardwareCapabilities,
        verified_fields: Vec<VerifiedField>,
    ) -> Result<Self, HardwareError> {
        if has_duplicates(&capabilities.allowed_signing_pools) || has_duplicates(&verified_fields) {
            return Err(HardwareError::schema());
        }
        Ok(Self {
            present,
            capabilities,
            verified_fields,
            claimed_routes: Vec::new(),
        })
    }

    fn validate(&self) -> Result<(), HardwareError> {
        if !valid_branch_claim(&self.capabilities.consensus_branch)
            || !valid_version_claim(&self.capabilities.transaction_version)
            || !valid_version_claim(&self.capabilities.pczt_encoding_version)
            || has_duplicates(&self.capabilities.allowed_signing_pools)
            || has_duplicates(&self.verified_fields)
        {
            return Err(HardwareError::schema());
        }
        Ok(())
    }
}

impl fmt::Debug for LiveProbe {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("LiveProbe([REDACTED])")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecisionStatus {
    DeviceDisconnected,
    CapabilityMissing,
    ProtocolIncompatible,
    Ready,
}

impl DecisionStatus {
    pub fn code(self) -> &'static str {
        match self {
            Self::DeviceDisconnected => "DEVICE_DISCONNECTED",
            Self::CapabilityMissing => "CAPABILITY_MISSING",
            Self::ProtocolIncompatible => "PROTOCOL_INCOMPATIBLE",
            Self::Ready => "READY",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HardwarePrivacy {
    Unavailable,
    Private,
    TransparentNotPrivate,
}

impl HardwarePrivacy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unavailable => "unavailable",
            Self::Private => "private",
            Self::TransparentNotPrivate => "transparent_not_private",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HardwareRoute {
    KeystonePcztV2,
}

impl HardwareRoute {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeystonePcztV2 => "keystone_pczt_v2",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct HardwareDecision {
    pub fingerprint_digest: String,
    pub table_revision: String,
    pub status: DecisionStatus,
    pub privacy: HardwarePrivacy,
    pub capabilities: HardwareCapabilities,
    pub verified_fields: Vec<VerifiedField>,
    pub host_trusting_fields: Vec<VerifiedField>,
    pub route: Option<HardwareRoute>,
    pub route_claims: Vec<ClaimedRoute>,
    pub pay_eligible: bool,
    pub electron_verified_fields: bool,
}

pub struct HardwareRouteMetadata {
    pub route: HardwareRoute,
    pub fingerprint_digest: String,
    pub table_revision: String,
    pub transaction_version: String,
    pub consensus_branch: String,
    pub pczt_encoding_version: String,
    pub signing_pools: Vec<&'static str>,
    pub verified_fields: Vec<&'static str>,
    pub host_trusting_fields: Vec<&'static str>,
}

impl HardwareRouteMetadata {
    pub fn public_field_names(&self) -> [&'static str; 9] {
        [
            "route",
            "fingerprint_digest",
            "table_revision",
            "transaction_version",
            "consensus_branch",
            "pczt_encoding_version",
            "signing_pools",
            "verified_fields",
            "host_trusting_fields",
        ]
    }

    pub fn accepted_artifact_bytes(&self) -> usize {
        0
    }

    pub fn returned_artifact_bytes(&self) -> usize {
        0
    }
}

impl fmt::Debug for HardwareRouteMetadata {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HardwareRouteMetadata")
            .field("route", &self.route)
            .field("fingerprint_digest", &self.fingerprint_digest)
            .field("table_revision", &self.table_revision)
            .field("transaction_version", &self.transaction_version)
            .field("consensus_branch", &self.consensus_branch)
            .field("pczt_encoding_version", &self.pczt_encoding_version)
            .field("signing_pools", &self.signing_pools)
            .field("verified_fields", &self.verified_fields)
            .field("host_trusting_fields", &self.host_trusting_fields)
            .finish()
    }
}

pub(crate) fn decide(
    profiles: &[ReviewedProfile],
    fingerprint: &DeviceFingerprint,
    probe: &LiveProbe,
) -> Result<HardwareDecision, HardwareError> {
    probe.validate()?;
    let digest = fingerprint.digest();

    if !probe.present {
        return Ok(denied_decision(
            digest,
            DecisionStatus::DeviceDisconnected,
            HardwarePrivacy::Unavailable,
        ));
    }

    let Some(profile) = profiles
        .iter()
        .find(|profile| profile.fingerprint == *fingerprint)
    else {
        return Ok(denied_decision(
            digest,
            DecisionStatus::CapabilityMissing,
            HardwarePrivacy::Unavailable,
        ));
    };

    if !protocol_is_exact(profile, probe) {
        let mut decision = narrowed_decision(profile, probe, digest);
        decision.capabilities.clear_private_authority();
        decision.verified_fields.clear();
        decision.host_trusting_fields = VerifiedField::ALL.to_vec();
        decision.status = DecisionStatus::ProtocolIncompatible;
        decision.privacy = HardwarePrivacy::Unavailable;
        return Ok(decision);
    }

    let mut decision = narrowed_decision(profile, probe, digest);
    if complete_private_route(profile.fingerprint.vendor, &decision.capabilities) {
        decision.status = DecisionStatus::Ready;
        decision.privacy = HardwarePrivacy::Private;
        decision.route = Some(HardwareRoute::KeystonePcztV2);
        decision.pay_eligible = true;
    } else {
        decision.status = DecisionStatus::CapabilityMissing;
        if decision.capabilities.can_sign_transparent {
            decision.privacy = HardwarePrivacy::TransparentNotPrivate;
        }
    }
    Ok(decision)
}

pub(crate) fn select_route(
    profiles: &[ReviewedProfile],
    decision: &HardwareDecision,
) -> Result<HardwareRouteMetadata, HardwareError> {
    let profile = profiles
        .iter()
        .find(|profile| {
            profile.fingerprint.digest() == decision.fingerprint_digest
                && profile.table_revision == decision.table_revision
        })
        .ok_or_else(HardwareError::state_corrupt)?;
    let capabilities_within_review = ALL_CAPABILITY_FLAGS
        .iter()
        .all(|flag| !decision.capabilities.contains(*flag) || profile.capabilities.contains(*flag));
    let pools_within_review = decision
        .capabilities
        .allowed_signing_pools
        .iter()
        .all(|pool| profile.capabilities.allowed_signing_pools.contains(pool));
    let fields_within_review = decision
        .verified_fields
        .iter()
        .all(|field| profile.verified_fields.contains(field));
    let expected_host_fields = VerifiedField::ALL
        .iter()
        .copied()
        .filter(|field| !decision.verified_fields.contains(field))
        .collect::<Vec<_>>();
    if decision.status != DecisionStatus::Ready
        || decision.privacy != HardwarePrivacy::Private
        || decision.route != Some(HardwareRoute::KeystonePcztV2)
        || !decision.pay_eligible
        || decision.electron_verified_fields
        || !decision.route_claims.is_empty()
        || !capabilities_within_review
        || !pools_within_review
        || !fields_within_review
        || has_duplicates(&decision.verified_fields)
        || decision.host_trusting_fields != expected_host_fields
        || decision.capabilities.consensus_branch != REQUIRED_BRANCH
        || decision.capabilities.transaction_version != REQUIRED_TRANSACTION_VERSION
        || decision.capabilities.pczt_encoding_version != REQUIRED_PCZT_ENCODING_VERSION
        || !complete_private_route(profile.fingerprint.vendor, &decision.capabilities)
    {
        return Err(HardwareError::state_corrupt());
    }
    Ok(HardwareRouteMetadata {
        route: HardwareRoute::KeystonePcztV2,
        fingerprint_digest: decision.fingerprint_digest.clone(),
        table_revision: decision.table_revision.clone(),
        transaction_version: decision.capabilities.transaction_version.clone(),
        consensus_branch: decision.capabilities.consensus_branch.clone(),
        pczt_encoding_version: decision.capabilities.pczt_encoding_version.clone(),
        signing_pools: decision
            .capabilities
            .allowed_signing_pools
            .iter()
            .copied()
            .map(SigningPool::as_str)
            .collect(),
        verified_fields: decision
            .verified_fields
            .iter()
            .copied()
            .map(VerifiedField::as_str)
            .collect(),
        host_trusting_fields: decision
            .host_trusting_fields
            .iter()
            .copied()
            .map(VerifiedField::as_str)
            .collect(),
    })
}

pub(crate) fn validate_persisted_decision(
    profiles: &[ReviewedProfile],
    decision: &HardwareDecision,
) -> Result<(), HardwareError> {
    if decision.fingerprint_digest.len() != 64
        || !decision
            .fingerprint_digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        || decision.table_revision.is_empty()
        || decision.table_revision.len() > 64
        || !decision
            .table_revision
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'+' | b'-'))
        || !strictly_ordered(&decision.capabilities.allowed_signing_pools)
        || !strictly_ordered(&decision.verified_fields)
        || decision.host_trusting_fields
            != VerifiedField::ALL
                .iter()
                .copied()
                .filter(|field| !decision.verified_fields.contains(field))
                .collect::<Vec<_>>()
    {
        return Err(HardwareError::state_corrupt());
    }
    select_route(profiles, decision)?;
    Ok(())
}

pub(crate) fn decision_narrows(prior: &HardwareDecision, candidate: &HardwareDecision) -> bool {
    candidate.fingerprint_digest == prior.fingerprint_digest
        && candidate.table_revision == prior.table_revision
        && candidate.status == prior.status
        && candidate.privacy == prior.privacy
        && candidate.route == prior.route
        && candidate.pay_eligible == prior.pay_eligible
        && candidate.electron_verified_fields == prior.electron_verified_fields
        && candidate.route_claims == prior.route_claims
        && candidate.capabilities.transaction_version == prior.capabilities.transaction_version
        && candidate.capabilities.consensus_branch == prior.capabilities.consensus_branch
        && candidate.capabilities.pczt_encoding_version == prior.capabilities.pczt_encoding_version
        && ALL_CAPABILITY_FLAGS.iter().all(|flag| {
            !candidate.capabilities.contains(*flag) || prior.capabilities.contains(*flag)
        })
        && candidate
            .capabilities
            .allowed_signing_pools
            .iter()
            .all(|pool| prior.capabilities.allowed_signing_pools.contains(pool))
        && candidate
            .verified_fields
            .iter()
            .all(|field| prior.verified_fields.contains(field))
}

fn narrowed_decision(
    profile: &ReviewedProfile,
    probe: &LiveProbe,
    fingerprint_digest: String,
) -> HardwareDecision {
    let capabilities =
        HardwareCapabilities::intersection(&profile.capabilities, &probe.capabilities);
    let verified_fields = profile
        .verified_fields
        .iter()
        .copied()
        .filter(|field| probe.verified_fields.contains(field))
        .collect::<Vec<_>>();
    let host_trusting_fields = VerifiedField::ALL
        .iter()
        .copied()
        .filter(|field| !verified_fields.contains(field))
        .collect();
    HardwareDecision {
        fingerprint_digest,
        table_revision: profile.table_revision.clone(),
        status: DecisionStatus::CapabilityMissing,
        privacy: HardwarePrivacy::Unavailable,
        capabilities,
        verified_fields,
        host_trusting_fields,
        route: None,
        route_claims: Vec::new(),
        pay_eligible: false,
        electron_verified_fields: false,
    }
}

fn denied_decision(
    fingerprint_digest: String,
    status: DecisionStatus,
    privacy: HardwarePrivacy,
) -> HardwareDecision {
    HardwareDecision {
        fingerprint_digest,
        table_revision: String::new(),
        status,
        privacy,
        capabilities: HardwareCapabilities::default(),
        verified_fields: Vec::new(),
        host_trusting_fields: VerifiedField::ALL.to_vec(),
        route: None,
        route_claims: Vec::new(),
        pay_eligible: false,
        electron_verified_fields: false,
    }
}

fn protocol_is_exact(profile: &ReviewedProfile, probe: &LiveProbe) -> bool {
    profile.capabilities.consensus_branch == REQUIRED_BRANCH
        && probe.capabilities.consensus_branch == REQUIRED_BRANCH
        && profile.capabilities.transaction_version == REQUIRED_TRANSACTION_VERSION
        && probe.capabilities.transaction_version == REQUIRED_TRANSACTION_VERSION
        && profile.capabilities.pczt_encoding_version == REQUIRED_PCZT_ENCODING_VERSION
        && probe.capabilities.pczt_encoding_version == REQUIRED_PCZT_ENCODING_VERSION
}

fn complete_private_route(vendor: DeviceVendor, capabilities: &HardwareCapabilities) -> bool {
    vendor == DeviceVendor::Keystone
        && capabilities.can_receive_private
        && capabilities.can_prepare_tx
        && capabilities.can_sign_spend
        && capabilities.can_sign_ironwood
        && capabilities.can_tx_v6
        && capabilities.can_verify_pczt_on_device
        && capabilities.allowed_signing_pools.len() == 1
        && capabilities.allowed_signing_pools.first() == Some(&SigningPool::Ironwood)
}

fn valid_branch_claim(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() || byte == b'-')
}

fn valid_version_claim(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value.bytes().any(|byte| byte.is_ascii_digit())
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || byte == b'-')
}

fn has_duplicates<T: Eq>(values: &[T]) -> bool {
    values
        .iter()
        .enumerate()
        .any(|(index, value)| values[index + 1..].contains(value))
}

fn strictly_ordered<T: Ord>(values: &[T]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

fn update_length_prefixed(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
}

fn lowercase_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

#[derive(Clone, Eq, PartialEq)]
pub struct HardwareError {
    code: &'static str,
    message: &'static str,
}

impl HardwareError {
    pub(crate) fn schema() -> Self {
        Self {
            code: "SCHEMA",
            message: "Hardware capability input is invalid",
        }
    }

    pub(crate) fn state_corrupt() -> Self {
        Self {
            code: "STATE_CORRUPT",
            message: "Hardware capability state is unavailable",
        }
    }

    pub(crate) fn internal() -> Self {
        Self {
            code: "INTERNAL",
            message: "Hardware capability operation failed",
        }
    }

    pub fn code(&self) -> &'static str {
        self.code
    }
}

impl fmt::Debug for HardwareError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HardwareError")
            .field("code", &self.code)
            .finish()
    }
}

impl fmt::Display for HardwareError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message)
    }
}

impl std::error::Error for HardwareError {}
