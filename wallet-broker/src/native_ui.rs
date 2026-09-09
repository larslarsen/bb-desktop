use std::cell::RefCell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;

use eframe::egui;
use zeroize::Zeroize;

use crate::native::{
    FileDialogPort, NativeError, NativeSurfacePort, PasswordPrompt, RestoreMetadata, SurfaceResult,
    ZecNativeReview, ZecNativeReviewSurfacePort,
};
use crate::vault::SecretBytes;

pub struct BrokerNativeApp {
    password: String,
    submitted: bool,
    cancelled: bool,
}

impl BrokerNativeApp {
    pub fn new() -> Self {
        Self {
            password: String::new(),
            submitted: false,
            cancelled: false,
        }
    }

    pub fn take_submission(&mut self) -> Result<Option<SecretBytes>, NativeError> {
        if self.cancelled || !self.submitted {
            self.password.zeroize();
            return Ok(None);
        }
        if self.password.is_empty() || self.password.len() > 1_024 {
            self.password.zeroize();
            return Err(NativeError::locked());
        }
        let bytes = self.password.as_bytes().to_vec();
        self.password.zeroize();
        self.submitted = false;
        SecretBytes::new(bytes)
            .map(Some)
            .map_err(|_| NativeError::locked())
    }
}

impl Default for BrokerNativeApp {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for BrokerNativeApp {
    fn drop(&mut self) {
        self.password.zeroize();
    }
}

impl eframe::App for BrokerNativeApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("BitBook wallet authorization");
            ui.add(
                egui::TextEdit::singleline(&mut self.password)
                    .password(true)
                    .char_limit(1_024),
            );
            if ui.button("Authorize").clicked() {
                self.submitted = true;
            }
            if ui.button("Cancel").clicked() {
                self.cancelled = true;
                self.password.zeroize();
            }
        });
    }
}

#[derive(Default)]
pub struct EframeSurface {
    pub app: BrokerNativeApp,
    pub confirmation: bool,
    pub last_restore: Option<RestoreMetadata>,
    pub last_result: Option<SurfaceResult>,
}

impl NativeSurfacePort for EframeSurface {
    fn prompt_password(
        &mut self,
        prompt: PasswordPrompt,
    ) -> Result<Option<SecretBytes>, NativeError> {
        if !prompt.masked
            || prompt.copy_enabled
            || prompt.paste_to_other_surface
            || prompt.accessibility_value_exposed
            || prompt.maximum_utf8_bytes != 1_024
        {
            return Err(NativeError::locked());
        }
        self.app.take_submission()
    }

    fn confirm_restore(&mut self, metadata: RestoreMetadata) -> Result<bool, NativeError> {
        self.last_restore = Some(metadata);
        Ok(self.confirmation)
    }

    fn show_result(&mut self, result: SurfaceResult) {
        self.last_result = Some(result);
    }
}

enum ZecReviewState {
    Pending,
    Confirmed,
    Consumed,
    Denied,
}

struct ZecReviewControls {
    confirm: egui::Rect,
    cancel: egui::Rect,
}

struct ZecReviewDialog {
    review: ZecNativeReview,
    state: ZecReviewState,
}

impl ZecReviewDialog {
    fn new(review: ZecNativeReview) -> Self {
        Self {
            review,
            state: ZecReviewState::Pending,
        }
    }

    fn take_confirmed_review(&mut self) -> Option<ZecNativeReview> {
        match self.state {
            ZecReviewState::Confirmed => {
                self.state = ZecReviewState::Consumed;
                Some(self.review.clone())
            }
            ZecReviewState::Pending | ZecReviewState::Consumed | ZecReviewState::Denied => None,
        }
    }

    fn cancel(&mut self) {
        self.state = ZecReviewState::Denied;
    }

    fn is_pending(&self) -> bool {
        matches!(self.state, ZecReviewState::Pending)
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> ZecReviewControls {
        let body_height = (ui.available_height() - reserved_confirm_cancel_height(ui)).max(0.0);
        egui::ScrollArea::vertical()
            .max_height(body_height)
            .id_salt("zec_review_body")
            .auto_shrink([true, true])
            .show(ui, |ui| {
                render_immutable_zec_review(ui, &self.review);
            });
        let confirm = ui.button("Confirm Zcash send");
        let cancel = ui.button("Cancel");
        let close_or_escape = ui.input(|input| {
            input.viewport().close_requested() || input.key_pressed(egui::Key::Escape)
        });
        if close_or_escape || cancel.clicked() {
            self.cancel();
        } else if confirm.clicked() && self.is_pending() {
            self.state = ZecReviewState::Confirmed;
        }
        ZecReviewControls {
            confirm: confirm.rect,
            cancel: cancel.rect,
        }
    }
}

struct ZecReviewNativeApp {
    dialog: Rc<RefCell<ZecReviewDialog>>,
    // Later frames from this programmatic close must not cancel a completed confirmation.
    closing: bool,
}

impl eframe::App for ZecReviewNativeApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            if !self.closing {
                self.dialog.borrow_mut().ui(ui);
            }
        });
        if self.closing {
            return;
        }
        if !self.dialog.borrow().is_pending() {
            self.closing = true;
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

fn run_synchronous_zec_review(review: &ZecNativeReview) -> Result<bool, NativeError> {
    let dialog = Rc::new(RefCell::new(ZecReviewDialog::new(review.clone())));
    let app_dialog = Rc::clone(&dialog);
    let run = catch_unwind(AssertUnwindSafe(move || {
        eframe::run_native(
            "Confirm Zcash send",
            eframe::NativeOptions {
                run_and_return: true,
                viewport: egui::ViewportBuilder::default()
                    .with_title("Confirm Zcash send")
                    .with_inner_size([520.0, 720.0])
                    .with_resizable(false),
                ..eframe::NativeOptions::default()
            },
            Box::new(move |_cc| {
                Ok(Box::new(ZecReviewNativeApp {
                    dialog: app_dialog,
                    closing: false,
                }))
            }),
        )
    }));
    match run {
        Ok(Ok(())) => Ok(matches!(
            dialog.borrow_mut().take_confirmed_review(),
            Some(confirmed) if confirmed == *review
        )),
        Ok(Err(_)) | Err(_) => {
            dialog.borrow_mut().cancel();
            Err(NativeError::locked())
        }
    }
}

impl ZecNativeReviewSurfacePort for EframeSurface {
    fn confirm_zec_review(&mut self, review: &ZecNativeReview) -> Result<bool, NativeError> {
        run_synchronous_zec_review(review)
    }
}

fn reserved_confirm_cancel_height(ui: &egui::Ui) -> f32 {
    let spacing = ui.spacing();
    let button_height = (ui.text_style_height(&egui::TextStyle::Button)
        + 2.0 * spacing.button_padding.y)
        .max(spacing.interact_size.y);
    2.0 * button_height + 2.0 * spacing.item_spacing.y
}

fn render_immutable_zec_review(ui: &mut egui::Ui, review: &ZecNativeReview) {
    ui.heading("Confirm Zcash send");
    ui.label(format!("handle={}", review.handle));
    ui.label(format!("session={}", review.session_id));
    ui.label(format!("account={}", review.account_id));
    ui.label(format!("network={}", review.network));
    ui.label(format!("request={}", review.request_id));
    ui.label(format!("intent={}", review.intent_hash));
    ui.label(format!("receiver={}", review.receiver));
    ui.label(format!("amount_zat={}", review.amount_zat));
    ui.label(format!("fee_zat={}", review.fee_zat));
    ui.label(format!("fee_bound_zat={}", review.fee_bound_zat));
    ui.label(format!("memo_sha256={}", review.memo_sha256));
    ui.label(format!("expires_at={}", review.expires_at));
    ui.label(format!("tx_version={}", review.transaction_version));
    ui.label(format!("consensus_branch={:08x}", review.consensus_branch));
    ui.label(format!("spend_pool={}", review.spend_pool));
    ui.label(format!("output_pool={}", review.output_pool));
    ui.label(format!("review_hash={}", review.review_hash));
}

#[derive(Default)]
pub struct RfdDialog;

impl FileDialogPort for RfdDialog {
    fn choose_new_backup(&mut self) -> Result<Option<String>, NativeError> {
        rfd::FileDialog::new()
            .add_filter("BitBook encrypted vault", &["vault"])
            .save_file()
            .map(|path| {
                path.to_str()
                    .map(str::to_owned)
                    .ok_or_else(NativeError::locked)
            })
            .transpose()
    }

    fn choose_existing_backup(&mut self) -> Result<Option<String>, NativeError> {
        rfd::FileDialog::new()
            .add_filter("BitBook encrypted vault", &["vault"])
            .pick_file()
            .map(|path| {
                path.to_str()
                    .map(str::to_owned)
                    .ok_or_else(NativeError::locked)
            })
            .transpose()
    }
}

impl RfdDialog {
    pub fn choose_xmr_wallet_rpc(&mut self) -> Result<Option<String>, NativeError> {
        rfd::FileDialog::new()
            .set_title("Select monero-wallet-rpc")
            .pick_file()
            .map(|path| {
                path.to_str()
                    .map(str::to_owned)
                    .ok_or_else(NativeError::schema)
            })
            .transpose()
    }
}

#[cfg(test)]
mod zec_review_tests;

#[cfg(test)]
mod zec_native_app_tests;
