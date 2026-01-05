use crate::config::Config;
use crate::context::Context;
use crate::ui::modals::ModalsHandler;
use egui::CentralPanel;

pub struct App {
    pub context: Context,

    modals_handler: ModalsHandler,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        Self::set_theme(cc, &config);
        Self {
            context: Context::default(),
            modals_handler: ModalsHandler::default(),
        }
    }

    fn set_theme(cc: &eframe::CreationContext<'_>, config: &Config) {
        cc.egui_ctx.set_theme(config.theme);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // ..

            self.modals_handler.handle_errors(ui, &self.context);
        });
    }
}
