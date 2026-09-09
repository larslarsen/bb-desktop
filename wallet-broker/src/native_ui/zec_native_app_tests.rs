use std::cell::RefCell;
use std::rc::Rc;

use eframe::egui;

use crate::native::ZecNativeReview;

use super::{ZecReviewControls, ZecReviewDialog, ZecReviewNativeApp};

const FRAME_DT: f64 = 1.0 / 60.0;
const NATIVE_SIZE: [f32; 2] = [520.0, 720.0];
const SMALL_SIZE: [f32; 2] = [360.0, 480.0];
const APP_SIZE: [f32; 2] = [1280.0, 800.0];
const CONFIRM_LABEL: &str = "Confirm Zcash send";
const CANCEL_LABEL: &str = "Cancel";

struct PersistentUi {
    ctx: egui::Context,
    time: f64,
    screen: egui::Rect,
}

struct AppObservation {
    close_commands: usize,
    controls: Option<(egui::Rect, egui::Rect)>,
}

impl PersistentUi {
    fn new(size: [f32; 2]) -> Self {
        Self {
            ctx: egui::Context::default(),
            time: 0.0,
            screen: egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(size[0], size[1])),
        }
    }

    fn take_input(
        &mut self,
        events: Vec<egui::Event>,
        close_requested: bool,
    ) -> egui::RawInput {
        let mut input = egui::RawInput {
            screen_rect: Some(self.screen),
            time: Some(self.time),
            events,
            ..egui::RawInput::default()
        };
        if let Some(info) = input.viewports.get_mut(&egui::ViewportId::ROOT) {
            info.inner_rect = Some(self.screen);
            if close_requested {
                info.events.push(egui::ViewportEvent::Close);
            }
        }
        input
    }

    fn finish_frame(&mut self, output: egui::FullOutput) {
        output.drop_without_applying_deltas();
        self.time += FRAME_DT;
    }

    fn run_dialog(
        &mut self,
        dialog: &mut ZecReviewDialog,
        events: Vec<egui::Event>,
        close_requested: bool,
    ) -> ZecReviewControls {
        let input = self.take_input(events, close_requested);
        let mut controls = None;
        let output = self.ctx.run_ui(input, |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                controls = Some(dialog.ui(ui));
            });
        });
        self.finish_frame(output);
        controls.expect("production dialog.ui must run on each frame")
    }

    fn run_app(
        &mut self,
        app: &mut ZecReviewNativeApp,
        events: Vec<egui::Event>,
        close_requested: bool,
    ) -> AppObservation {
        let input = self.take_input(events, close_requested);
        let mut frame = eframe::Frame::_new_kittest();
        let output = self.ctx.run_ui(input, |ui| {
            eframe::App::ui(app, ui, &mut frame);
        });
        let observation = AppObservation {
            close_commands: root_close_commands(&output),
            controls: painted_app_controls(&output),
        };
        self.finish_frame(output);
        observation
    }
}

fn public_test_id(digit: char) -> String {
    std::iter::repeat(digit).take(64).collect()
}

fn long_review() -> ZecNativeReview {
    let mut receiver = String::from("synthetic-receiver:");
    receiver.push_str(&"A".repeat(512));
    ZecNativeReview {
        handle: public_test_id('1'),
        session_id: public_test_id('2'),
        account_id: "0123456789abcdef0123456789abcdef".to_owned(),
        network: "testnet".to_owned(),
        request_id: public_test_id('3'),
        intent_hash: public_test_id('4'),
        receiver,
        amount_zat: "1".to_owned(),
        fee_zat: "1".to_owned(),
        fee_bound_zat: "2".to_owned(),
        memo_sha256: public_test_id('5'),
        expires_at: "2026-12-31T23:59:59Z".to_owned(),
        transaction_version: 6,
        consensus_branch: 0,
        spend_pool: "orchard".to_owned(),
        output_pool: "orchard".to_owned(),
        review_hash: public_test_id('6'),
    }
}

fn short_review(receiver: &str, amount_zat: &str) -> ZecNativeReview {
    ZecNativeReview {
        handle: "synthetic-handle".to_owned(),
        session_id: "synthetic-session".to_owned(),
        account_id: "synthetic-account".to_owned(),
        network: "testnet".to_owned(),
        request_id: "synthetic-request".to_owned(),
        intent_hash: "synthetic-intent-hash".to_owned(),
        receiver: receiver.to_owned(),
        amount_zat: amount_zat.to_owned(),
        fee_zat: "1".to_owned(),
        fee_bound_zat: "2".to_owned(),
        memo_sha256: "synthetic-memo-sha256".to_owned(),
        expires_at: "2026-12-31T23:59:59Z".to_owned(),
        transaction_version: 6,
        consensus_branch: 0,
        spend_pool: "orchard".to_owned(),
        output_pool: "orchard".to_owned(),
        review_hash: "synthetic-review-hash".to_owned(),
    }
}

fn assert_rendered_button(viewport: egui::Rect, rect: egui::Rect) {
    assert!(rect.is_finite());
    assert!(rect.is_positive());
    assert!(viewport.contains_rect(rect));
}

fn assert_usable_controls(viewport: egui::Rect, controls: &ZecReviewControls) {
    assert_rendered_button(viewport, controls.confirm);
    assert_rendered_button(viewport, controls.cancel);
}

fn primary_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

fn click_dialog(
    ui: &mut PersistentUi,
    dialog: &mut ZecReviewDialog,
    pos: egui::Pos2,
) -> ZecReviewControls {
    let _ = ui.run_dialog(dialog, vec![egui::Event::PointerMoved(pos)], false);
    let _ = ui.run_dialog(dialog, vec![primary_button(pos, true)], false);
    ui.run_dialog(dialog, vec![primary_button(pos, false)], false)
}

fn first_dialog_render(ui: &mut PersistentUi, dialog: &mut ZecReviewDialog) -> ZecReviewControls {
    let controls = ui.run_dialog(dialog, Vec::new(), false);
    assert_usable_controls(ui.screen, &controls);
    controls
}

fn collect_exact_text(
    shape: &egui::Shape,
    clip_rect: egui::Rect,
    label: &str,
    out: &mut Vec<(egui::Rect, egui::Rect)>,
) {
    match shape {
        egui::Shape::Text(text) if text.galley.text() == label => {
            out.push((text.visual_bounding_rect(), clip_rect));
        }
        egui::Shape::Vec(nested) => {
            for child in nested {
                collect_exact_text(child, clip_rect, label, out);
            }
        }
        _ => {}
    }
}

fn collect_label(output: &egui::FullOutput, label: &str) -> Vec<(egui::Rect, egui::Rect)> {
    let mut painted = Vec::new();
    for clipped in &output.shapes {
        collect_exact_text(&clipped.shape, clipped.clip_rect, label, &mut painted);
    }
    painted
}

fn visible_last_exact_label(output: &egui::FullOutput, label: &str) -> egui::Rect {
    let painted = collect_label(output, label);
    let (visual, clip) = *painted.last().expect("exact label must be painted");
    let visible = visual.intersect(clip);
    assert!(visible.is_finite());
    assert!(visible.is_positive());
    visible
}

fn painted_app_controls(output: &egui::FullOutput) -> Option<(egui::Rect, egui::Rect)> {
    let confirm_hits = collect_label(output, CONFIRM_LABEL);
    if confirm_hits.is_empty() {
        return None;
    }
    assert!(confirm_hits.len() > 1);
    let first = confirm_hits[0].0;
    let (visual, clip) = confirm_hits[confirm_hits.len() - 1];
    assert!(visual.min.y > first.min.y);
    let confirm = visual.intersect(clip);
    assert!(confirm.is_finite());
    assert!(confirm.is_positive());
    let cancel = visible_last_exact_label(output, CANCEL_LABEL);
    assert!(confirm != cancel);
    assert!(!confirm.contains(cancel.center()));
    assert!(!cancel.contains(confirm.center()));
    Some((confirm, cancel))
}

fn root_close_commands(output: &egui::FullOutput) -> usize {
    output
        .viewport_output
        .get(&egui::ViewportId::ROOT)
        .map(|viewport| {
            viewport
                .commands
                .iter()
                .filter(|command| matches!(command, egui::ViewportCommand::Close))
                .count()
        })
        .unwrap_or(0)
}

fn new_native_app(review: ZecNativeReview) -> (Rc<RefCell<ZecReviewDialog>>, ZecReviewNativeApp) {
    let dialog = Rc::new(RefCell::new(ZecReviewDialog::new(review)));
    let app = ZecReviewNativeApp {
        dialog: Rc::clone(&dialog),
        closing: false,
    };
    (dialog, app)
}

fn click_app(
    ui: &mut PersistentUi,
    app: &mut ZecReviewNativeApp,
    pos: egui::Pos2,
    release_closes: bool,
) -> [AppObservation; 3] {
    [
        ui.run_app(app, vec![egui::Event::PointerMoved(pos)], false),
        ui.run_app(app, vec![primary_button(pos, true)], false),
        ui.run_app(app, vec![primary_button(pos, false)], release_closes),
    ]
}

fn long_review_keeps_controls_usable(size: [f32; 2]) {
    let original = long_review();
    let mut dialog = ZecReviewDialog::new(original.clone());
    let mut ui = PersistentUi::new(size);
    let controls = first_dialog_render(&mut ui, &mut dialog);
    let _ = click_dialog(&mut ui, &mut dialog, controls.confirm.center());
    assert!(dialog.take_confirmed_review() == Some(original.clone()));
    assert!(dialog.take_confirmed_review().is_none());

    let mut dialog = ZecReviewDialog::new(original);
    let mut ui = PersistentUi::new(size);
    let controls = first_dialog_render(&mut ui, &mut dialog);
    let _ = click_dialog(&mut ui, &mut dialog, controls.cancel.center());
    assert!(dialog.take_confirmed_review().is_none());
    let _ = click_dialog(&mut ui, &mut dialog, controls.confirm.center());
    assert!(dialog.take_confirmed_review().is_none());
}

#[test]
fn long_review_at_native_size_keeps_controls_usable() {
    long_review_keeps_controls_usable(NATIVE_SIZE);
}

#[test]
fn long_review_at_small_viewport_keeps_controls_usable() {
    long_review_keeps_controls_usable(SMALL_SIZE);
}

#[test]
fn native_app_confirm_closes_once_and_preserves_owned_review() {
    let original = short_review("synthetic-receiver-owned", "17");
    let (dialog, mut app) = new_native_app(original.clone());
    let mut ui = PersistentUi::new(APP_SIZE);

    let mut confirm = None;
    for _ in 0..3 {
        let frame = ui.run_app(&mut app, Vec::new(), false);
        assert!(frame.close_commands == 0);
        assert!(dialog.borrow_mut().take_confirmed_review().is_none());
        let (confirm_rect, _) = frame
            .controls
            .expect("actual Confirm and Cancel must be painted");
        confirm = Some(confirm_rect);
    }
    let confirm = confirm
        .expect("actual Confirm button must be painted")
        .center();

    let [moved, pressed, confirmed] = click_app(&mut ui, &mut app, confirm, false);
    assert!(moved.close_commands == 0);
    assert!(pressed.close_commands == 0);
    assert!(confirmed.close_commands == 1);

    let close_requested = ui.run_app(&mut app, Vec::new(), true);
    assert!(close_requested.close_commands == 0);
    let idle = ui.run_app(&mut app, Vec::new(), false);
    assert!(idle.close_commands == 0);
    assert!(dialog.borrow_mut().take_confirmed_review() == Some(original.clone()));
    assert!(dialog.borrow_mut().take_confirmed_review().is_none());

    let (fresh_dialog, mut fresh_app) =
        new_native_app(short_review("synthetic-receiver-fresh", "0"));
    let mut fresh_ui = PersistentUi::new(APP_SIZE);
    let pending = fresh_ui.run_app(&mut fresh_app, Vec::new(), false);
    assert!(pending.close_commands == 0);
    assert!(fresh_dialog.borrow_mut().take_confirmed_review().is_none());
}

#[test]
fn native_app_close_release_denies_and_closes_once() {
    let original = short_review("synthetic-receiver-close", "5");
    let (dialog, mut app) = new_native_app(original);
    let mut ui = PersistentUi::new(APP_SIZE);

    let first = ui.run_app(&mut app, Vec::new(), false);
    assert!(first.close_commands == 0);
    assert!(dialog.borrow_mut().take_confirmed_review().is_none());
    let (confirm, _) = first
        .controls
        .expect("actual Confirm and Cancel must be painted");
    let confirm = confirm.center();

    let [moved, pressed, denied] = click_app(&mut ui, &mut app, confirm, true);
    assert!(moved.close_commands == 0);
    assert!(pressed.close_commands == 0);
    assert!(denied.close_commands == 1);
    assert!(dialog.borrow_mut().take_confirmed_review().is_none());

    let close_again = ui.run_app(&mut app, Vec::new(), true);
    assert!(close_again.close_commands == 0);
    assert!(dialog.borrow_mut().take_confirmed_review().is_none());
    let idle = ui.run_app(&mut app, Vec::new(), false);
    assert!(idle.close_commands == 0);
    assert!(dialog.borrow_mut().take_confirmed_review().is_none());

    let [moved, pressed, released] = click_app(&mut ui, &mut app, confirm, false);
    assert!(moved.close_commands == 0);
    assert!(pressed.close_commands == 0);
    assert!(released.close_commands == 0);
    assert!(dialog.borrow_mut().take_confirmed_review().is_none());
}
