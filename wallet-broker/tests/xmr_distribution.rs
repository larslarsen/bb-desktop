use bitbook_wallet_broker::xmr::distribution::{
    ARCHIVE_MEMBER, ARCHIVE_NAME, ARCHIVE_SHA256, EXECUTABLE_BYTES, EXECUTABLE_SHA256,
    LINUX_PIN_ID, MAX_SELECTED_PATH_BYTES, MONEROD_ARCHIVE_MEMBER, MONEROD_BYTES, MONEROD_SHA256,
    VERIFIED_VERSION,
};
use bitbook_wallet_broker::xmr::test_support::{
    DistributionFault, DistributionRig, HostPlatform, InstallationKind, SelectedEntry,
};

const NORMAL_PATH: &str = "/opt/monero-gui-v0.18.5.2/extras/monero-wallet-rpc";
const PORTABLE_PATH: &str = "/media/removable/monero/extras/monero-wallet-rpc";

#[test]
fn exact_outer_and_inner_release_provenance_is_frozen() {
    assert_eq!(ARCHIVE_NAME, "monero-gui-linux-x64-v0.18.5.2.tar.bz2");
    assert_eq!(
        ARCHIVE_SHA256,
        "294017a5aa1ee86420b0c62fe4046000f42438375a8559d9ff55e41e5c6cbbcd"
    );
    assert_eq!(
        ARCHIVE_MEMBER,
        "monero-gui-v0.18.5.2/extras/monero-wallet-rpc"
    );
    assert_eq!(EXECUTABLE_BYTES, 29_026_368);
    assert_eq!(
        EXECUTABLE_SHA256,
        "c1e3aff7c72837e6f29045c439b772a82b5cd7324c8b831fa825a6ce2019a656"
    );
    assert_eq!(
        VERIFIED_VERSION,
        "Monero 'Fluorine Fermi' (v0.18.5.1-release)"
    );
    assert_eq!(MONEROD_ARCHIVE_MEMBER, "monero-gui-v0.18.5.2/monerod");
    assert_eq!(MONEROD_BYTES, 24_112_840);
    assert_eq!(
        MONEROD_SHA256,
        "9b3b2676ea7868c1a7186feea9569c2cf7683ae79d2fcc769c846a91c810a1f5"
    );
    assert_eq!(LINUX_PIN_ID, "monero-gui-linux-x64-v0.18.5.2");
}

#[test]
fn only_linux_x86_64_can_select_or_launch_xmr() {
    for platform in [
        HostPlatform::LinuxAarch64,
        HostPlatform::WindowsX86_64,
        HostPlatform::MacosX86_64,
        HostPlatform::MacosAarch64,
        HostPlatform::Unknown,
    ] {
        let mut rig = DistributionRig::new(platform);
        assert_eq!(rig.enroll(NORMAL_PATH).unwrap_err().code(), "UNAVAILABLE");
        assert!(rig.operations().is_empty(), "platform {platform:?}");
        assert!(rig.selection_record().is_none());
        assert_eq!(rig.child_count(), 0);
    }
}

#[test]
fn normal_and_portable_are_identical_explicit_selection_contracts() {
    for (kind, path) in [
        (InstallationKind::Normal, NORMAL_PATH),
        (InstallationKind::Portable, PORTABLE_PATH),
    ] {
        let mut rig = DistributionRig::reviewed_linux(kind, path);
        rig.enroll(path).unwrap();
        assert_eq!(
            rig.operations(),
            [
                "lstat",
                "regular",
                "length",
                "executable",
                "sha256",
                "version",
                "record"
            ]
        );
        let record = rig.selection_record().unwrap();
        assert_eq!(record.schema_version, 1);
        assert_eq!(record.platform_pin_id, LINUX_PIN_ID);
        assert_eq!(record.selected_path, path);
        assert_eq!(record.mode, 0o600);
        assert!(record.was_atomically_replaced);
        assert!(!record.parent_was_scanned);
    }
}

#[test]
fn selected_path_length_covers_immediately_below_at_and_above_limit() {
    assert_eq!(MAX_SELECTED_PATH_BYTES, 4_096);
    for (length, accepted) in [
        (MAX_SELECTED_PATH_BYTES - 1, true),
        (MAX_SELECTED_PATH_BYTES, true),
        (MAX_SELECTED_PATH_BYTES + 1, false),
    ] {
        let path = format!("/{}", "a".repeat(length - 1));
        let mut rig = DistributionRig::reviewed_linux(InstallationKind::Normal, &path);
        let result = rig.enroll(&path);
        assert_eq!(result.is_ok(), accepted, "length {length}");
        if !accepted {
            assert_eq!(result.unwrap_err().code(), "SCHEMA");
            assert!(rig.operations().is_empty());
            assert!(rig.selection_record().is_none());
        }
    }
}

#[test]
fn relative_non_utf8_and_nul_paths_fail_before_filesystem_access() {
    for invalid in [
        "monero-wallet-rpc",
        "../monero-wallet-rpc",
        "",
        "/bad\0name",
    ] {
        let mut rig = DistributionRig::reviewed_linux(InstallationKind::Normal, NORMAL_PATH);
        assert_eq!(rig.enroll(invalid).unwrap_err().code(), "SCHEMA");
        assert!(rig.operations().is_empty());
    }
    let mut rig = DistributionRig::reviewed_linux(InstallationKind::Normal, NORMAL_PATH);
    assert_eq!(
        rig.enroll_non_utf8(&[0xff, 0xfe]).unwrap_err().code(),
        "SCHEMA"
    );
    assert!(rig.operations().is_empty());
}

#[test]
fn lstat_rejects_symlink_and_every_nonregular_entry_without_following() {
    for entry in [
        SelectedEntry::Symlink,
        SelectedEntry::Directory,
        SelectedEntry::Fifo,
        SelectedEntry::Socket,
        SelectedEntry::BlockDevice,
        SelectedEntry::CharacterDevice,
    ] {
        let mut rig = DistributionRig::with_entry(entry, NORMAL_PATH);
        assert_eq!(
            rig.enroll(NORMAL_PATH).unwrap_err().code(),
            "PROTOCOL_INCOMPATIBLE"
        );
        assert_eq!(rig.operations(), ["lstat"]);
        assert!(!rig.followed_final_component());
        assert!(!rig.version_was_probed());
        assert_eq!(rig.child_count(), 0);
    }
}

#[test]
fn byte_length_boundary_precedes_hash_version_and_execution() {
    for (length, accepted) in [
        (EXECUTABLE_BYTES - 1, false),
        (EXECUTABLE_BYTES, true),
        (EXECUTABLE_BYTES + 1, false),
    ] {
        let mut rig = DistributionRig::reviewed_linux(InstallationKind::Normal, NORMAL_PATH);
        rig.set_observed_length(length);
        let result = rig.enroll(NORMAL_PATH);
        assert_eq!(result.is_ok(), accepted, "length {length}");
        if !accepted {
            assert_eq!(result.unwrap_err().code(), "PROTOCOL_INCOMPATIBLE");
            assert_eq!(rig.operations(), ["lstat", "regular", "length"]);
            assert_eq!(rig.hashed_bytes(), 0);
            assert!(!rig.version_was_probed());
            assert_eq!(rig.child_count(), 0);
        }
    }
}

#[test]
fn nonexecutable_selected_file_fails_before_hash_version_or_child_start() {
    let mut rig = DistributionRig::reviewed_linux(InstallationKind::Normal, NORMAL_PATH);
    rig.set_effective_user_executable(false);
    assert_eq!(
        rig.enroll(NORMAL_PATH).unwrap_err().code(),
        "PROTOCOL_INCOMPATIBLE"
    );
    assert_eq!(
        rig.operations(),
        ["lstat", "regular", "length", "executable"]
    );
    assert_eq!(rig.hashed_bytes(), 0);
    assert!(!rig.version_was_probed());
    assert_eq!(rig.child_count(), 0);
    assert!(rig.selection_record().is_none());
}

#[test]
fn executable_hash_and_exact_version_each_fail_closed_in_order() {
    let mut changed = DistributionRig::reviewed_linux(InstallationKind::Normal, NORMAL_PATH);
    changed
        .set_executable_sha256("00e3aff7c72837e6f29045c439b772a82b5cd7324c8b831fa825a6ce2019a656");
    assert_eq!(
        changed.enroll(NORMAL_PATH).unwrap_err().code(),
        "PROTOCOL_INCOMPATIBLE"
    );
    assert_eq!(
        changed.operations(),
        ["lstat", "regular", "length", "executable", "sha256"]
    );
    assert_eq!(changed.hashed_bytes(), EXECUTABLE_BYTES);
    assert!(!changed.version_was_probed());
    assert_eq!(changed.child_count(), 0);

    for version in [
        "Monero 'Fluorine Fermi' (v0.18.5.1-release) extra",
        "Monero 'Fluorine Fermi' (v0.18.5.2-release)",
        "v0.18.5.1-release",
    ] {
        let mut rig = DistributionRig::reviewed_linux(InstallationKind::Normal, NORMAL_PATH);
        rig.set_version_output(version);
        assert_eq!(
            rig.enroll(NORMAL_PATH).unwrap_err().code(),
            "PROTOCOL_INCOMPATIBLE"
        );
        assert!(rig.version_was_probed());
        assert!(rig.selection_record().is_none());
        assert_eq!(rig.child_count(), 0);
    }
}

#[test]
fn corrupt_partial_unknown_or_wrong_mode_selection_record_never_launches() {
    for fault in [
        DistributionFault::PartialRecord,
        DistributionFault::UnknownRecordField,
        DistributionFault::UnknownSchema,
        DistributionFault::WrongRecordMode,
        DistributionFault::SymlinkedRecord,
        DistributionFault::RecordSync,
        DistributionFault::DirectorySync,
    ] {
        let mut rig = DistributionRig::reviewed_linux(InstallationKind::Normal, NORMAL_PATH);
        rig.arm_fault(fault);
        let error = rig.enroll(NORMAL_PATH).unwrap_err();
        assert!(matches!(error.code(), "STATE_CORRUPT" | "INTERNAL"));
        assert_eq!(rig.child_count(), 0);
        assert!(!rig.reported_success());
    }
}

#[test]
fn launch_reverifies_exact_path_and_portable_removal_or_change_stops_child() {
    for mutation in ["missing", "identity", "size", "mtime"] {
        let mut rig = DistributionRig::reviewed_linux(InstallationKind::Portable, PORTABLE_PATH);
        rig.enroll(PORTABLE_PATH).unwrap();
        rig.launch().unwrap();
        assert_eq!(rig.child_count(), 1);
        rig.mutate_selected_file(mutation);
        assert_eq!(rig.poll_selected_file().unwrap_err().code(), "UNAVAILABLE");
        assert_eq!(rig.child_count(), 0);
        assert_eq!(
            rig.last_teardown_reason(),
            Some("selected-executable-changed")
        );
        assert!(!rig.attempted_alternate_path());
    }

    let mut rig = DistributionRig::reviewed_linux(InstallationKind::Normal, NORMAL_PATH);
    rig.enroll(NORMAL_PATH).unwrap();
    rig.clear_operations();
    rig.launch().unwrap();
    assert_eq!(
        &rig.operations()[..6],
        [
            "lstat",
            "regular",
            "length",
            "executable",
            "sha256",
            "version"
        ]
    );
}

#[test]
fn no_path_search_download_fallback_or_path_disclosure_exists() {
    let canary = "/private/CANARY_WAL007_SELECTED_PATH/monero-wallet-rpc";
    let mut rig = DistributionRig::reviewed_linux(InstallationKind::Portable, canary);
    rig.arm_fault(DistributionFault::MissingExecutable);
    assert_eq!(rig.enroll(canary).unwrap_err().code(), "UNAVAILABLE");
    assert!(!rig.attempted_path_search());
    assert!(!rig.attempted_parent_scan());
    assert!(!rig.attempted_download());
    assert!(!rig.attempted_alternate_path());
    let observable = format!(
        "error={:?};logs={:?};diagnostics={:?}",
        rig.last_error(),
        rig.logs(),
        rig.diagnostics()
    );
    assert!(!observable.contains(canary));
    assert!(!observable.contains("CANARY_WAL007_SELECTED_PATH"));
    assert_eq!(rig.child_count(), 0);
}
