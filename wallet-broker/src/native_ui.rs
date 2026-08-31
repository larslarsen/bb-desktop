use eframe::egui;
use zeroize::Zeroize;

use crate::native::{
    FileDialogPort, NativeError, NativeSurfacePort, PasswordPrompt, RestoreMetadata, SurfaceResult,
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
