use crate::context::Context;
use egui::{CentralPanel, Color32, Frame, InputState, Painter, Sense, Shape};
use geometry::figures::grid::Grid2DBuilder;
use geometry::primitives::point2d::Point2D;
use geometry::space::{Shapeable, SpaceSize};

#[derive(Debug, Default)]
pub struct CanvasComponent {}

impl CanvasComponent {
    pub fn show(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        CentralPanel::default().show_inside(ui, |ui| {
            Frame::canvas(ui.style())
                .fill(Color32::WHITE)
                .show(ui, |ui| {
                    ui.input(|i| Self::handle_scroll(context, i));
                    Self::pipeline(ui, context);
                });
        });
    }

    fn pipeline(ui: &mut egui::Ui, context: &mut Context) {
        let shapes = Self::create_shapes(ui, context);
        Self::draw(ui, context, shapes);
    }

    fn create_shapes(_ui: &mut egui::Ui, context: &mut Context) -> Vec<Shape> {
        let mut lines = vec![];

        let grid = Grid2DBuilder::default()
            .with_negative_enabled(false)
            .build()
            .lines(&context.space);
        lines.extend(grid);

        lines
            .iter()
            .map(|line| line.screen_shape(&context.space))
            .collect()
    }

    fn draw(ui: &mut egui::Ui, context: &mut Context, shapes: Vec<Shape>) {
        let painter = Self::initialize_painter(ui, context);
        painter.extend(shapes);
    }

    fn initialize_painter(ui: &mut egui::Ui, context: &mut Context) -> Painter {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) =
            ui.allocate_painter(painter_size, Sense::click_and_drag());

        // Setting zero point
        context.space.settings.zero_point =
            Point2D::from(response.rect.center()).with_space_screen();
        // Setting canvas size
        context.space.settings.size = SpaceSize {
            width: response.rect.max.x as f64,
            height: response.rect.max.y as f64,
        };

        painter
    }

    fn handle_scroll(context: &mut Context, input_state: &InputState) {
        let delta = input_state.smooth_scroll_delta.y;

        context.space.state.pixels_per_centimeter += (delta as f64) * 0.1;
    }
}
