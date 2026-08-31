use bitbook_wallet_broker::session::{
    AUTHORIZATION_IDLE_MILLIS, ClockError, MonotonicClock, SessionEvent, SessionManager,
};
use bitbook_wallet_broker::vault::{SecretBytes, WipeEvent, WipeObserver};

const ACCOUNT_A: &str = "00112233445566778899aabbccddeeff";
const ACCOUNT_B: &str = "ffeeddccbbaa99887766554433221100";

#[derive(Default)]
struct Clock {
    now: u64,
    fail: Option<ClockError>,
}

impl MonotonicClock for Clock {
    fn now_millis(&mut self) -> Result<u64, ClockError> {
        match self.fail {
            Some(error) => Err(error),
            None => Ok(self.now),
        }
    }
}

#[derive(Default)]
struct Wipes(Vec<WipeEvent>);

impl WipeObserver for Wipes {
    fn observe(&mut self, event: WipeEvent) {
        self.0.push(event);
    }
}

fn manager(now: u64) -> SessionManager<Clock, Wipes> {
    SessionManager::new(Clock { now, fail: None }, Wipes::default())
}

fn unlock(manager: &mut SessionManager<Clock, Wipes>, account: &str) {
    manager
        .unlock(
            account,
            SecretBytes::new(b"CANARY_WAL004_IN_MEMORY_SPEND_MATERIAL".to_vec()).unwrap(),
        )
        .unwrap();
}

#[test]
fn idle_timeout_is_exactly_fifteen_minutes_and_locks_at_boundary() {
    assert_eq!(AUTHORIZATION_IDLE_MILLIS, 15 * 60 * 1_000);
    let mut sessions = manager(10_000);
    unlock(&mut sessions, ACCOUNT_A);
    sessions.clock_mut().now = 10_000 + AUTHORIZATION_IDLE_MILLIS - 1;
    sessions.check_deadlines().unwrap();
    assert!(sessions.is_unlocked(ACCOUNT_A));
    sessions.clock_mut().now += 1;
    sessions.check_deadlines().unwrap();
    assert!(!sessions.is_unlocked(ACCOUNT_A));
    assert!(sessions.wipe_observer().0.iter().any(|event| {
        event.label == "session-spend-material" && event.all_zero
    }));
}

#[test]
fn only_successful_native_authorization_resets_idle_deadline() {
    let mut sessions = manager(1_000);
    unlock(&mut sessions, ACCOUNT_A);
    sessions.clock_mut().now = 500_000;
    sessions
        .handle(ACCOUNT_A, SessionEvent::NativeAuthorizationSucceeded)
        .unwrap();
    sessions.clock_mut().now = 500_000 + AUTHORIZATION_IDLE_MILLIS - 1;
    sessions.check_deadlines().unwrap();
    assert!(sessions.is_unlocked(ACCOUNT_A));
    sessions.clock_mut().now += 1;
    sessions.check_deadlines().unwrap();
    assert!(!sessions.is_unlocked(ACCOUNT_A));
}

#[test]
fn late_native_authorization_at_existing_deadline_times_out_and_wipes() {
    let started = 41_000;
    let mut sessions = manager(started);
    unlock(&mut sessions, ACCOUNT_A);
    sessions.clock_mut().now = started + AUTHORIZATION_IDLE_MILLIS;

    assert_eq!(
        sessions
            .handle(ACCOUNT_A, SessionEvent::NativeAuthorizationSucceeded)
            .unwrap_err()
            .code(),
        "TIMEOUT"
    );
    assert!(!sessions.is_unlocked(ACCOUNT_A));
    let wipe = sessions.wipe_observer().0.last().unwrap();
    assert_eq!(wipe.label, "session-spend-material");
    assert!(wipe.length > 0 && wipe.all_zero);
}

#[test]
fn invalid_account_unlock_is_schema_and_wipes_supplied_material() {
    const INVALID_ACCOUNT: &str = "00112233445566778899AABBCCDDEEFF";
    const MATERIAL: &[u8] = b"CANARY_WAL004_INVALID_ACCOUNT_MATERIAL";
    let mut sessions = manager(73_000);

    assert_eq!(
        sessions
            .unlock(
                INVALID_ACCOUNT,
                SecretBytes::new(MATERIAL.to_vec()).unwrap(),
            )
            .unwrap_err()
            .code(),
        "SCHEMA"
    );
    assert!(!sessions.is_unlocked(INVALID_ACCOUNT));
    assert_eq!(sessions.authorization_deadline(INVALID_ACCOUNT), None);
    assert!(sessions.wipe_observer().0.iter().any(|event| {
        event.label == "session-spend-material"
            && event.length == MATERIAL.len()
            && event.all_zero
    }));
}

#[test]
fn polling_sync_backup_browsing_and_failed_or_cancelled_prompts_never_extend() {
    for event in [
        SessionEvent::StatusPolled,
        SessionEvent::AccountsListed,
        SessionEvent::SnapshotPublished,
        SessionEvent::SyncEvent,
        SessionEvent::BackupPathBrowsed,
        SessionEvent::NativeAuthorizationFailed,
        SessionEvent::NativeAuthorizationCancelled,
    ] {
        let mut sessions = manager(2_000);
        unlock(&mut sessions, ACCOUNT_A);
        sessions.clock_mut().now = 2_000 + AUTHORIZATION_IDLE_MILLIS - 1;
        sessions.handle(ACCOUNT_A, event).unwrap();
        sessions.clock_mut().now += 1;
        sessions.check_deadlines().unwrap();
        assert!(!sessions.is_unlocked(ACCOUNT_A), "{event:?} extended authorization");
    }
}

#[test]
fn every_forced_lock_event_wipes_spend_material() {
    for event in [
        SessionEvent::ManualLock,
        SessionEvent::AppBackgrounded,
        SessionEvent::ScreenLocked,
        SessionEvent::BrokerQuit,
        SessionEvent::BrokerRestarted,
        SessionEvent::OperationErrored,
        SessionEvent::AccountReplaced,
        SessionEvent::RestoreSucceeded,
    ] {
        let mut sessions = manager(5_000);
        unlock(&mut sessions, ACCOUNT_A);
        sessions.handle(ACCOUNT_A, event).unwrap();
        assert!(!sessions.is_unlocked(ACCOUNT_A), "{event:?} left session unlocked");
        let wipe = sessions.wipe_observer().0.last().unwrap();
        assert_eq!(wipe.label, "session-spend-material");
        assert!(wipe.length > 0 && wipe.all_zero);
    }
}

#[test]
fn separate_accounts_have_isolated_deadlines_and_wipes() {
    let mut sessions = manager(8_000);
    unlock(&mut sessions, ACCOUNT_A);
    sessions.clock_mut().now = 100_000;
    unlock(&mut sessions, ACCOUNT_B);
    sessions.clock_mut().now = 100_001;
    sessions.handle(ACCOUNT_A, SessionEvent::ManualLock).unwrap();
    assert!(!sessions.is_unlocked(ACCOUNT_A));
    assert!(sessions.is_unlocked(ACCOUNT_B));
    sessions.clock_mut().now = 100_000 + AUTHORIZATION_IDLE_MILLIS;
    sessions.check_deadlines().unwrap();
    assert!(!sessions.is_unlocked(ACCOUNT_B));
}

#[test]
fn backward_clock_fails_all_sessions_locked() {
    let mut sessions = manager(50_000);
    unlock(&mut sessions, ACCOUNT_A);
    sessions.clock_mut().now = 49_999;
    assert_eq!(sessions.check_deadlines().unwrap_err().code(), "TIMEOUT");
    assert!(!sessions.is_unlocked(ACCOUNT_A));
}

#[test]
fn overflowing_deadline_fails_locked_instead_of_wrapping() {
    let mut sessions = manager(u64::MAX - AUTHORIZATION_IDLE_MILLIS + 1);
    assert_eq!(
        sessions
            .unlock(
                ACCOUNT_A,
                SecretBytes::new(b"CANARY_WAL004_IN_MEMORY_SPEND_MATERIAL".to_vec()).unwrap(),
            )
            .unwrap_err()
            .code(),
        "TIMEOUT"
    );
    assert!(!sessions.is_unlocked(ACCOUNT_A));
}

#[test]
fn monotonic_clock_error_fails_locked() {
    let mut sessions = manager(12_000);
    unlock(&mut sessions, ACCOUNT_A);
    sessions.clock_mut().fail = Some(ClockError::Unavailable);
    assert_eq!(sessions.check_deadlines().unwrap_err().code(), "TIMEOUT");
    assert!(!sessions.is_unlocked(ACCOUNT_A));
}

#[test]
fn status_and_account_listing_never_request_spend_secret() {
    let mut sessions = manager(20_000);
    assert_eq!(sessions.status(ACCOUNT_A).broker, "locked");
    assert!(!sessions.status_requires_spend_secret());
    unlock(&mut sessions, ACCOUNT_A);
    let before = sessions.authorization_deadline(ACCOUNT_A).unwrap();
    assert_eq!(sessions.status(ACCOUNT_A).broker, "ready");
    assert_eq!(sessions.authorization_deadline(ACCOUNT_A), Some(before));
}
