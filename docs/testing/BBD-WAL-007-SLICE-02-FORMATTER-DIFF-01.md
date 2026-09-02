# BBD-WAL-007 Slice-2 Exact Formatter Diff 01

Source: Hermes's retained Rust 1.98 `cargo fmt --check` stdout/stderr, supplied by the
owner and cross-checked against Hermes's raw terminal record after the summarized stop
record proved incomplete.

The command was not rerun to create this record. Its exit code was 1. The exact source
changes requested by the formatter are:

```diff
Diff in /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/xmr/process.rs:510:
         self.port.create_private_layout(&paths)?;

         let (rpc_username, username_origin) = random_secret(&mut self.port, ENTROPY_BYTES)?;
-        let (mut rpc_password, mut password_origin) =
-            random_secret(&mut self.port, ENTROPY_BYTES)?;
+        let (mut rpc_password, mut password_origin) = random_secret(&mut self.port, ENTROPY_BYTES)?;
         while rpc_password.expose() == rpc_username.expose() {
             (rpc_password, password_origin) = random_secret(&mut self.port, ENTROPY_BYTES)?;
         }
Diff in /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/xmr/process.rs:566:
         {
             return Err(XmrError::unavailable());
         }
-        let observation = self.port.readiness(
-            self.child.as_mut().ok_or_else(XmrError::internal)?,
-            plan,
-        )?;
+        let observation = self
+            .port
+            .readiness(self.child.as_mut().ok_or_else(XmrError::internal)?, plan)?;
         if observation.elapsed_millis > READINESS_TIMEOUT_SECS * 1_000 {
             return Err(XmrError::unavailable());
         }
Diff in /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/xmr/process.rs:576:
-        if observation.malformed
-            || !observation.authenticated
-            || !observation.exact_version
-        {
+        if observation.malformed || !observation.authenticated || !observation.exact_version {
             return Err(XmrError::protocol_incompatible());
         }
         self.readiness_authenticated = true;
Diff in /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/xmr/process.rs:705:
     }
 }

-fn reserve_port<P: ProcessPort>(
-    port: &mut P,
-) -> Result<(u16, P::Reservation, bool), XmrError> {
+fn reserve_port<P: ProcessPort>(port: &mut P) -> Result<(u16, P::Reservation, bool), XmrError> {
     for _ in 0..MAX_PORT_ATTEMPTS {
         let (candidate, from_entropy) = port.next_port_candidate()?;
         if !(u32::from(PORT_MIN)..=u32::from(PORT_MAX)).contains(&candidate) {
Diff in /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/xmr/process.rs:1006:
         let started = Instant::now();
         let timeout = Duration::from_millis(timeout_millis);
         loop {
-            if child.try_wait().map_err(|_| XmrError::internal())?.is_some() {
+            if child
+                .try_wait()
+                .map_err(|_| XmrError::internal())?
+                .is_some()
+            {
                 return Ok(true);
             }
             if started.elapsed() >= timeout {
Diff in /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/xmr/process.rs:1041:
         .nth(3)
         .ok_or_else(XmrError::request_schema)?;
     let metadata = fs::symlink_metadata(root).map_err(|_| XmrError::state_corrupt())?;
-    if !metadata.file_type().is_dir()
-        || metadata.permissions().mode() & 0o777 != DIRECTORY_MODE
-    {
+    if !metadata.file_type().is_dir() || metadata.permissions().mode() & 0o777 != DIRECTORY_MODE {
         return Err(XmrError::state_corrupt());
     }
     Ok(())
Diff in /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/xmr/test_support.rs:11:
 pub use crate::xmr::model::{HostPlatform, XmrNetwork};
 use crate::xmr::process::{
     DerivedPaths, EntropyOrigin, ProcessCoordinator, ProcessManager, ProcessPort,
-    ReadinessObservation, ReservationFailure, WalletRpcProcessPlan,
-    next_os_port_for_test_port,
+    ReadinessObservation, ReservationFailure, WalletRpcProcessPlan, next_os_port_for_test_port,
 };

 #[derive(Clone, Copy, Debug, Eq, PartialEq)]
Diff in /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/xmr/test_support.rs:914:
             account_id: account.to_owned(),
             network: parsed.clone(),
             root: root.to_path_buf(),
-            manager: Some(process_manager(account, parsed, root, RecordingProcessPort::new())),
+            manager: Some(process_manager(
+                account,
+                parsed,
+                root,
+                RecordingProcessPort::new(),
+            )),
             coordinator: None,
             zec_alive: false,
             social_alive: false,
Diff in /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/xmr/test_support.rs:1035:
     }

     pub fn child_count(&self) -> usize {
-        self.manager
-            .as_ref()
-            .map_or_else(
-                || self.coordinator.as_ref().expect("process pool").len(),
-                ProcessManager::child_count,
-            )
+        self.manager.as_ref().map_or_else(
+            || self.coordinator.as_ref().expect("process pool").len(),
+            ProcessManager::child_count,
+        )
     }

-    pub fn start_account(
-        &mut self,
-        account: &str,
-        network: XmrNetwork,
-    ) -> Result<(), XmrError> {
+    pub fn start_account(&mut self, account: &str, network: XmrNetwork) -> Result<(), XmrError> {
         let manager = process_manager(
             account,
             Ok(network),
Diff in /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/xmr/test_support.rs:1129:
         self.manager().credentials_wiped()
     }

-    pub fn fail_account(
-        &mut self,
-        account: &str,
-        fault: ProcessFault,
-    ) -> Result<(), XmrError> {
+    pub fn fail_account(&mut self, account: &str, fault: ProcessFault) -> Result<(), XmrError> {
         let coordinator = self.coordinator.as_mut().expect("process pool");
         let manager = coordinator
             .manager_mut(account)
Diff in /home/lars/OpenBazaar/bb-desktop/wallet-broker/tests/xmr_process.rs:260:
     }
     assert_eq!(process.child_count(), 4);
     assert_eq!(process.account_spawn_count(&format!("{:032x}", 0)), 1);
-    process
-        .poll_account_health(&format!("{:032x}", 0))
-        .unwrap();
+    process.poll_account_health(&format!("{:032x}", 0)).unwrap();
     assert_eq!(process.account_spawn_count(&format!("{:032x}", 0)), 1);
     assert_eq!(
         process
Diff in /home/lars/OpenBazaar/bb-desktop/wallet-broker/tests/xmr_process.rs:402:
             ProcessFault::ExecutableRemoved | ProcessFault::ExecutableChanged => {
                 process.start().and_then(|()| process.poll_health())
             }
-            ProcessFault::BrokerExit => {
-                process.start().and_then(|()| process.broker_exit_for_test())
-            }
+            ProcessFault::BrokerExit => process
+                .start()
+                .and_then(|()| process.broker_exit_for_test()),
             _ => process.start(),
         };
         assert_eq!(process.child_count(), 0, "fault {fault:?}");
```

The formatter output ended with `FMT=1` and exit code 1.
