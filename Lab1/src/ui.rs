use crate::errors::ProjectError;

pub struct Ui {
    min_width: f32,
    min_height: f32,
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            min_width: 950.0,
            min_height: 550.0,
        }
    }
}

impl Ui {
    pub fn native_panic_message(error: ProjectError) {
        rfd::MessageDialog::new()
            .set_title("Critical Error")
            .set_description(error.to_string())
            .set_level(rfd::MessageLevel::Error)
            .show();
    }
}

pub mod themes;
