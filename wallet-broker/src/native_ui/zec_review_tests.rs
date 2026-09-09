use eframe::egui;

use crate::native::ZecNativeReview;

use super::{ZecReviewControls, ZecReviewDialog};

const SCREEN: egui::Rect =
    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1280.0, 800.0));
const FRAME_DT: f64 = 1.0 / 60.0;

struct PersistentUi {
    ctx: egui::Context,
    time: f64,
}

impl PersistentUi {
    fn new() -> Self {
        Self {
            ctx: egui::Context::default(),
            time: 0.0,
        }
    }

    fn run(
        &mut self,
        dialog: &mut ZecReviewDialog,
        events: Vec<egui::Event>,
        close_requested: bool,
    ) -> ZecReviewControls {
        let mut input = egui::RawInput {
            screen_rect: Some(SCREEN),
            time: Some(self.time),
            events,
            ..egui::RawInput::default()
        };
        if let Some(info) = input.viewports.get_mut(&egui::ViewportId::ROOT) {
            info.inner_rect = Some(SCREEN);
            if close_requested {
                info.events.push(egui::ViewportEvent::Close);
            }
        }
        let mut controls = None;
        let output = self.ctx.run_ui(input, |ui| {
            controls = Some(dialog.ui(ui));
        });
        output.drop_without_applying_deltas();
        self.time += FRAME_DT;
        controls.expect("production dialog.ui must run on each frame")
    }
}

fn synthetic_review(receiver: &str, amount_zat: &str) -> ZecNativeReview {
    ZecNativeReview {
        handle: "synthetic-handle".to_owned(),
        session_id: "synthetic-session".to_owned(),
        account_id: "synthetic-account".to_owned(),
        network: "synthetic-network".to_owned(),
        request_id: "synthetic-request".to_owned(),
        intent_hash: "synthetic-intent-hash".to_owned(),
        receiver: receiver.to_owned(),
        amount_zat: amount_zat.to_owned(),
        fee_zat: "0".to_owned(),
        fee_bound_zat: "0".to_owned(),
        memo_sha256: "synthetic-memo-sha256".to_owned(),
        expires_at: "synthetic-expiry".to_owned(),
        transaction_version: 0,
        consensus_branch: 0,
        spend_pool: "synthetic-spend-pool".to_owned(),
        output_pool: "synthetic-output-pool".to_owned(),
        review_hash: "synthetic-review-hash".to_owned(),
    }
}

fn assert_rendered_button(rect: egui::Rect) {
    assert!(rect.is_finite());
    assert!(rect.is_positive());
    assert!(SCREEN.contains_rect(rect));
}

fn assert_usable_controls(controls: &ZecReviewControls) {
    assert_rendered_button(controls.confirm);
    assert_rendered_button(controls.cancel);
}

fn primary_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

fn escape_press() -> egui::Event {
    egui::Event::Key {
        key: egui::Key::Escape,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }
}

fn click_at(
    ui: &mut PersistentUi,
    dialog: &mut ZecReviewDialog,
    pos: egui::Pos2,
) -> ZecReviewControls {
    let _ = ui.run(dialog, vec![egui::Event::PointerMoved(pos)], false);
    let _ = ui.run(dialog, vec![primary_button(pos, true)], false);
    ui.run(dialog, vec![primary_button(pos, false)], false)
}

fn first_render(ui: &mut PersistentUi, dialog: &mut ZecReviewDialog) -> ZecReviewControls {
    let controls = ui.run(dialog, Vec::new(), false);
    assert_usable_controls(&controls);
    controls
}

fn outside_point(controls: &ZecReviewControls) -> egui::Pos2 {
    let pos = egui::pos2(SCREEN.max.x - 8.0, SCREEN.max.y - 8.0);
    assert!(SCREEN.contains(pos));
    assert!(!controls.confirm.contains(pos));
    assert!(!controls.cancel.contains(pos));
    pos
}

#[test]
fn no_input_frames_and_outside_click_yield_no_confirmed_review() {
    let mut dialog = ZecReviewDialog::new(synthetic_review("receiver-idle", "0"));
    let mut ui = PersistentUi::new();
    let controls = first_render(&mut ui, &mut dialog);
    for _ in 0..3 {
        let _ = ui.run(&mut dialog, Vec::new(), false);
        assert!(dialog.take_confirmed_review().is_none());
    }
    let _ = click_at(&mut ui, &mut dialog, outside_point(&controls));
    assert!(dialog.take_confirmed_review().is_none());
}

#[test]
fn confirm_click_returns_exact_owned_review_and_cannot_rearm() {
    let original = synthetic_review("receiver-owned", "17");
    let mut caller_clone = original.clone();
    let mut dialog = ZecReviewDialog::new(caller_clone.clone());
    caller_clone.receiver = "receiver-mutated-after-new".to_owned();
    caller_clone.amount_zat = "99".to_owned();
    caller_clone.review_hash = "mutated-review-hash".to_owned();

    let mut ui = PersistentUi::new();
    let controls = first_render(&mut ui, &mut dialog);
    let _ = click_at(&mut ui, &mut dialog, controls.confirm.center());
    assert!(dialog.take_confirmed_review() == Some(original.clone()));
    assert!(dialog.take_confirmed_review().is_none());
    let _ = click_at(&mut ui, &mut dialog, controls.confirm.center());
    assert!(dialog.take_confirmed_review().is_none());
    assert!(caller_clone.receiver != original.receiver);
}

#[test]
fn cancel_click_denies_and_blocks_later_confirm() {
    let mut dialog = ZecReviewDialog::new(synthetic_review("receiver-cancel", "3"));
    let mut ui = PersistentUi::new();
    let controls = first_render(&mut ui, &mut dialog);
    let _ = click_at(&mut ui, &mut dialog, controls.cancel.center());
    assert!(dialog.take_confirmed_review().is_none());
    let _ = click_at(&mut ui, &mut dialog, controls.confirm.center());
    assert!(dialog.take_confirmed_review().is_none());
}

#[test]
fn viewport_close_denies_including_same_frame_confirm_release() {
    let review = synthetic_review("receiver-close", "5");

    let mut dialog = ZecReviewDialog::new(review.clone());
    let mut ui = PersistentUi::new();
    let controls = first_render(&mut ui, &mut dialog);
    let _ = ui.run(&mut dialog, Vec::new(), true);
    assert!(dialog.take_confirmed_review().is_none());
    let _ = click_at(&mut ui, &mut dialog, controls.confirm.center());
    assert!(dialog.take_confirmed_review().is_none());

    let mut dialog = ZecReviewDialog::new(review);
    let mut ui = PersistentUi::new();
    let controls = first_render(&mut ui, &mut dialog);
    let confirm = controls.confirm.center();
    let _ = ui.run(&mut dialog, vec![egui::Event::PointerMoved(confirm)], false);
    let _ = ui.run(&mut dialog, vec![primary_button(confirm, true)], false);
    let _ = ui.run(&mut dialog, vec![primary_button(confirm, false)], true);
    assert!(dialog.take_confirmed_review().is_none());
    let _ = click_at(&mut ui, &mut dialog, confirm);
    assert!(dialog.take_confirmed_review().is_none());
}

#[test]
fn escape_denies_and_blocks_later_confirm() {
    let mut dialog = ZecReviewDialog::new(synthetic_review("receiver-escape", "7"));
    let mut ui = PersistentUi::new();
    let controls = first_render(&mut ui, &mut dialog);
    let _ = ui.run(&mut dialog, vec![escape_press()], false);
    assert!(dialog.take_confirmed_review().is_none());
    let _ = click_at(&mut ui, &mut dialog, controls.confirm.center());
    assert!(dialog.take_confirmed_review().is_none());
}

#[test]
fn fresh_dialog_is_independent_and_cancel_clears_undrained_confirm() {
    let review_a = synthetic_review("receiver-a", "11");
    let review_b = synthetic_review("receiver-b", "22");
    assert!(review_a.review_hash == review_b.review_hash);
    assert!(review_a.receiver != review_b.receiver);
    assert!(review_a.amount_zat != review_b.amount_zat);

    let mut first = ZecReviewDialog::new(review_a.clone());
    let mut ui = PersistentUi::new();
    let controls = first_render(&mut ui, &mut first);
    let _ = click_at(&mut ui, &mut first, controls.confirm.center());
    assert!(first.take_confirmed_review() == Some(review_a.clone()));

    let mut second = ZecReviewDialog::new(review_b.clone());
    let mut ui = PersistentUi::new();
    let controls = first_render(&mut ui, &mut second);
    assert!(second.take_confirmed_review().is_none());
    let _ = click_at(&mut ui, &mut second, controls.confirm.center());
    assert!(second.take_confirmed_review() == Some(review_b));

    let mut third = ZecReviewDialog::new(review_a);
    let mut ui = PersistentUi::new();
    let controls = first_render(&mut ui, &mut third);
    let _ = click_at(&mut ui, &mut third, controls.confirm.center());
    third.cancel();
    assert!(third.take_confirmed_review().is_none());
}
