use std::io::Write;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, Mutex};

use crate::vault::{WipeEvent, WipeObserver};
use crate::zec::ZecError;
use crate::zec::test_support::cleanup_lifecycle_tests::prepared_artifact;

use super::{ExtractedTransaction, ObservedSigningArtifact, mutex_lock};

const INTENDED_PANIC: &str = "cleanup-lifecycle-intended-panic";
const EXTRACTED_BYTE: u8 = 0x5a;
const EXTRACTED_LENGTH: usize = 64;
const AUTHORITATIVE_PCZT_LABEL: &str = "zec-authoritative-pczt";
const EXTRACTED_TRANSACTION_LABEL: &str = "zec-extracted-transaction";

#[derive(Clone)]
struct WipeRecorder {
    inner: Arc<Mutex<Vec<WipeEvent>>>,
}

impl WipeRecorder {
    fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn events(&self) -> Vec<WipeEvent> {
        mutex_lock(&self.inner).clone()
    }
}

impl WipeObserver for WipeRecorder {
    fn observe(&mut self, event: WipeEvent) {
        mutex_lock(&self.inner).push(event);
    }
}

fn is_intended_panic(payload: &(dyn std::any::Any + Send)) -> bool {
    payload
        .downcast_ref::<&str>()
        .is_some_and(|value| *value == INTENDED_PANIC)
        || payload
            .downcast_ref::<String>()
            .is_some_and(|value| value == INTENDED_PANIC)
}

fn raw_is_nonempty_nonzero(secret: &crate::vault::SecretBytes) -> bool {
    secret.expose(|bytes| !bytes.is_empty() && bytes.iter().any(|byte| *byte != 0))
}

fn extracted_matches_pattern(extracted: &ExtractedTransaction<'_>) -> bool {
    extracted.as_slice().len() == EXTRACTED_LENGTH
        && extracted
            .as_slice()
            .iter()
            .all(|byte| *byte == EXTRACTED_BYTE)
}

fn assert_single_wipe(recorder: &WipeRecorder, label: &'static str, length: usize) {
    let events = recorder.events();
    assert_eq!(events.len(), 1, "label={label} length={length}");
    assert_eq!(events[0].label, label, "label={label} length={length}");
    assert_eq!(events[0].length, length, "label={label} length={length}");
    assert!(events[0].all_zero, "label={label} length={length}");
    let again = recorder.events();
    assert_eq!(again.len(), 1, "label={label} length={length}");
}

#[test]
fn pczt_owner_wipes_on_error_and_unwind() {
    let recorder = WipeRecorder::new();
    let mut length = 0usize;
    let error = (|| -> Result<(), ZecError> {
        let mut observer = recorder.clone();
        let artifact = prepared_artifact("cleanup-lifecycle-pczt-error");
        length = artifact.raw.len();
        assert!(
            raw_is_nonempty_nonzero(&artifact.raw),
            "label={AUTHORITATIVE_PCZT_LABEL} length={length}"
        );
        let _owner = ObservedSigningArtifact::from_prepared(artifact, &mut observer);
        assert!(
            recorder.events().is_empty(),
            "label={AUTHORITATIVE_PCZT_LABEL} length={length}"
        );
        Err(ZecError::internal())
    })();
    assert_eq!(error.expect_err("pczt owner error").code(), "INTERNAL");
    assert_single_wipe(&recorder, AUTHORITATIVE_PCZT_LABEL, length);

    let recorder = WipeRecorder::new();
    let mut length = 0usize;
    let mut reached = false;
    let result = catch_unwind(AssertUnwindSafe(|| {
        let mut observer = recorder.clone();
        let artifact = prepared_artifact("cleanup-lifecycle-pczt-panic");
        length = artifact.raw.len();
        assert!(
            raw_is_nonempty_nonzero(&artifact.raw),
            "label={AUTHORITATIVE_PCZT_LABEL} length={length}"
        );
        let _owner = ObservedSigningArtifact::from_prepared(artifact, &mut observer);
        assert!(
            recorder.events().is_empty(),
            "label={AUTHORITATIVE_PCZT_LABEL} length={length}"
        );
        reached = true;
        panic!("{INTENDED_PANIC}");
    }));
    assert!(reached, "label={AUTHORITATIVE_PCZT_LABEL} length={length}");
    assert!(
        is_intended_panic(result.expect_err("intended panic").as_ref()),
        "label={AUTHORITATIVE_PCZT_LABEL} length={length}"
    );
    assert_single_wipe(&recorder, AUTHORITATIVE_PCZT_LABEL, length);
}

#[test]
fn extracted_owner_wipes_on_error_and_unwind() {
    let recorder = WipeRecorder::new();
    let error = (|| -> Result<(), ZecError> {
        let mut observer = recorder.clone();
        let mut extracted = ExtractedTransaction::new(&mut observer);
        Write::write_all(&mut extracted, &[EXTRACTED_BYTE; EXTRACTED_LENGTH])
            .expect("write extracted fixture");
        assert!(
            extracted_matches_pattern(&extracted),
            "label={EXTRACTED_TRANSACTION_LABEL} length={EXTRACTED_LENGTH}"
        );
        assert!(
            recorder.events().is_empty(),
            "label={EXTRACTED_TRANSACTION_LABEL} length={EXTRACTED_LENGTH}"
        );
        Err(ZecError::internal())
    })();
    assert_eq!(error.expect_err("extracted owner error").code(), "INTERNAL");
    assert_single_wipe(&recorder, EXTRACTED_TRANSACTION_LABEL, EXTRACTED_LENGTH);

    let recorder = WipeRecorder::new();
    let mut reached = false;
    let result = catch_unwind(AssertUnwindSafe(|| {
        let mut observer = recorder.clone();
        let mut extracted = ExtractedTransaction::new(&mut observer);
        Write::write_all(&mut extracted, &[EXTRACTED_BYTE; EXTRACTED_LENGTH])
            .expect("write extracted fixture");
        assert!(
            extracted_matches_pattern(&extracted),
            "label={EXTRACTED_TRANSACTION_LABEL} length={EXTRACTED_LENGTH}"
        );
        assert!(
            recorder.events().is_empty(),
            "label={EXTRACTED_TRANSACTION_LABEL} length={EXTRACTED_LENGTH}"
        );
        reached = true;
        panic!("{INTENDED_PANIC}");
    }));
    assert!(
        reached,
        "label={EXTRACTED_TRANSACTION_LABEL} length={EXTRACTED_LENGTH}"
    );
    assert!(
        is_intended_panic(result.expect_err("intended panic").as_ref()),
        "label={EXTRACTED_TRANSACTION_LABEL} length={EXTRACTED_LENGTH}"
    );
    assert_single_wipe(&recorder, EXTRACTED_TRANSACTION_LABEL, EXTRACTED_LENGTH);
}
