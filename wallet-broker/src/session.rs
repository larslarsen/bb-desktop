use std::collections::HashMap;

use crate::vault::{SecretBytes, WipeObserver, valid_account_id};

pub const AUTHORIZATION_IDLE_MILLIS: u64 = 15 * 60 * 1_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClockError {
    Unavailable,
}

pub trait MonotonicClock {
    fn now_millis(&mut self) -> Result<u64, ClockError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionEvent {
    NativeAuthorizationSucceeded,
    StatusPolled,
    AccountsListed,
    SnapshotPublished,
    SyncEvent,
    BackupPathBrowsed,
    NativeAuthorizationFailed,
    NativeAuthorizationCancelled,
    ManualLock,
    AppBackgrounded,
    ScreenLocked,
    BrokerQuit,
    BrokerRestarted,
    OperationErrored,
    AccountReplaced,
    RestoreSucceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionError {
    code: &'static str,
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code)
    }
}

impl std::error::Error for SessionError {}

impl SessionError {
    fn timeout() -> Self {
        Self { code: "TIMEOUT" }
    }

    fn schema() -> Self {
        Self { code: "SCHEMA" }
    }

    pub fn code(&self) -> &'static str {
        self.code
    }
}

struct Session {
    spend_material: SecretBytes,
    deadline: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionStatus {
    pub broker: &'static str,
}

pub struct SessionManager<C: MonotonicClock, W: WipeObserver> {
    clock: C,
    wipe_observer: W,
    sessions: HashMap<String, Session>,
    last_now: Option<u64>,
}

impl<C: MonotonicClock, W: WipeObserver> SessionManager<C, W> {
    pub fn new(clock: C, wipe_observer: W) -> Self {
        Self {
            clock,
            wipe_observer,
            sessions: HashMap::new(),
            last_now: None,
        }
    }

    pub fn clock_mut(&mut self) -> &mut C {
        &mut self.clock
    }

    pub fn wipe_observer(&self) -> &W {
        &self.wipe_observer
    }

    pub fn unlock(
        &mut self,
        account_id: &str,
        mut spend_material: SecretBytes,
    ) -> Result<(), SessionError> {
        if !valid_account_id(account_id) {
            spend_material.wipe_with("session-spend-material", &mut self.wipe_observer);
            return Err(SessionError::schema());
        }
        let now = match self.read_clock() {
            Ok(now) => now,
            Err(error) => {
                spend_material.wipe_with("session-spend-material", &mut self.wipe_observer);
                return Err(error);
            }
        };
        let deadline = match now.checked_add(AUTHORIZATION_IDLE_MILLIS) {
            Some(deadline) => deadline,
            None => {
                let mut material = spend_material;
                material.wipe_with("session-spend-material", &mut self.wipe_observer);
                return Err(SessionError::timeout());
            }
        };
        if let Some(mut replaced) = self.sessions.insert(
            account_id.to_owned(),
            Session {
                spend_material,
                deadline,
            },
        ) {
            replaced
                .spend_material
                .wipe_with("session-spend-material", &mut self.wipe_observer);
        }
        Ok(())
    }

    pub fn handle(&mut self, account_id: &str, event: SessionEvent) -> Result<(), SessionError> {
        if matches!(
            event,
            SessionEvent::AppBackgrounded
                | SessionEvent::ScreenLocked
                | SessionEvent::BrokerQuit
                | SessionEvent::BrokerRestarted
        ) {
            self.lock_all();
            return Ok(());
        }
        if !valid_account_id(account_id) {
            return Err(SessionError::schema());
        }
        match event {
            SessionEvent::NativeAuthorizationSucceeded => {
                let now = self.read_clock()?;
                if self
                    .sessions
                    .get(account_id)
                    .is_some_and(|session| now >= session.deadline)
                {
                    self.lock_account(account_id);
                    return Err(SessionError::timeout());
                }
                let deadline = match now.checked_add(AUTHORIZATION_IDLE_MILLIS) {
                    Some(deadline) => deadline,
                    None => {
                        self.lock_account(account_id);
                        return Err(SessionError::timeout());
                    }
                };
                if let Some(session) = self.sessions.get_mut(account_id) {
                    session.deadline = deadline;
                }
            }
            SessionEvent::AppBackgrounded
            | SessionEvent::ScreenLocked
            | SessionEvent::BrokerQuit
            | SessionEvent::BrokerRestarted => {}
            SessionEvent::ManualLock
            | SessionEvent::OperationErrored
            | SessionEvent::AccountReplaced
            | SessionEvent::RestoreSucceeded => self.lock_account(account_id),
            SessionEvent::StatusPolled
            | SessionEvent::AccountsListed
            | SessionEvent::SnapshotPublished
            | SessionEvent::SyncEvent
            | SessionEvent::BackupPathBrowsed
            | SessionEvent::NativeAuthorizationFailed
            | SessionEvent::NativeAuthorizationCancelled => {}
        }
        Ok(())
    }

    pub fn check_deadlines(&mut self) -> Result<(), SessionError> {
        let now = self.read_clock()?;
        let expired: Vec<String> = self
            .sessions
            .iter()
            .filter(|(_, session)| now >= session.deadline)
            .map(|(account, _)| account.clone())
            .collect();
        for account in expired {
            self.lock_account(&account);
        }
        Ok(())
    }

    pub fn is_unlocked(&self, account_id: &str) -> bool {
        self.sessions.contains_key(account_id)
    }

    pub fn authorization_deadline(&self, account_id: &str) -> Option<u64> {
        self.sessions
            .get(account_id)
            .map(|session| session.deadline)
    }

    pub fn status(&self, account_id: &str) -> SessionStatus {
        SessionStatus {
            broker: if self.is_unlocked(account_id) {
                "ready"
            } else {
                "locked"
            },
        }
    }

    pub fn status_requires_spend_secret(&self) -> bool {
        false
    }

    fn read_clock(&mut self) -> Result<u64, SessionError> {
        let now = match self.clock.now_millis() {
            Ok(now) => now,
            Err(_) => {
                self.lock_all();
                return Err(SessionError::timeout());
            }
        };
        if self.last_now.is_some_and(|last| now < last) {
            self.lock_all();
            return Err(SessionError::timeout());
        }
        self.last_now = Some(now);
        Ok(now)
    }

    fn lock_account(&mut self, account_id: &str) {
        if let Some(mut session) = self.sessions.remove(account_id) {
            session
                .spend_material
                .wipe_with("session-spend-material", &mut self.wipe_observer);
        }
    }

    fn lock_all(&mut self) {
        let accounts: Vec<String> = self.sessions.keys().cloned().collect();
        for account in accounts {
            self.lock_account(&account);
        }
    }
}
