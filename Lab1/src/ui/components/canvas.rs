use crate::context::Context;
use egui::{CentralPanel, Color32, Frame, Painter, Response, Sense, Shape};
use geometry::figures::grid::Grid2DBuilder;

#[derive(Debug, Default)]
pub struct CanvasComponent;

pub const IS_NEGATIVE_ENABLED: bool = false;

impl CanvasComponent {
    pub fn show(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        CentralPanel::default().show_inside(ui, |ui| {
            Frame::canvas(ui.style())
                .fill(Color32::WHITE)
                .show(ui, |ui| {
                    ui.input(|i| context.viewport.handle_scroll(i));
                    let response = Self::pipeline(ui, context);
                    context.viewport.handle_pan(ui, response);
                });
        });
    }

    fn pipeline(ui: &mut egui::Ui, context: &mut Context) -> Response {
        let shapes = Self::create_shapes(ui, context);
        Self::draw(ui, context, shapes)
    }

    fn create_shapes(_ui: &mut egui::Ui, context: &mut Context) -> Vec<Shape> {
        let mut lines = vec![];

        let grid = Grid2DBuilder::default()
            .with_negative_enabled(IS_NEGATIVE_ENABLED)
            .with_units(
                context.viewport.geometry.unit_length.value(),
                context.viewport.geometry.unit_length.value(),
            )
            .build()
            .lines(&context.viewport);
        lines.extend(grid);

        lines
            .iter()
            .map(|line| line.to_pixels(&context.viewport).to_shape())
            .collect()
    }

    fn draw(ui: &mut egui::Ui, context: &mut Context, shapes: Vec<Shape>) -> Response {
        let (response, painter) = Self::initialize_painter(ui, context);
        painter.extend(shapes);

        response
    }

    fn initialize_painter(
        ui: &mut egui::Ui, context: &mut Context,
    ) -> (Response, Painter) {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) =
            ui.allocate_painter(painter_size, Sense::click_and_drag());

        context
            .viewport
            .update_state(&response, IS_NEGATIVE_ENABLED);

        (response, painter)
    }
}
