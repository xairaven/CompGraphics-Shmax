use crate::context::Context;
use egui::{CentralPanel, Color32, Frame, Painter, Response, Sense, Shape};
use geometry::primitives::line2d::Line2D;
use geometry::primitives::point2d::Point2D;

#[derive(Debug, Default)]
pub struct CanvasComponent;

impl CanvasComponent {
    pub fn show(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        CentralPanel::default().show_inside(ui, |ui| {
            Frame::canvas(ui.style())
                .fill(Color32::WHITE)
                .show(ui, |ui| {
                    ui.input(|i| {
                        if context.viewport.handle_scroll(i) {
                            context.figures.regenerate_fractal(&context.viewport);
                        }
                    });
                    let response = Self::pipeline(ui, context);
                    if context.viewport.handle_pan(ui, response) {
                        context.figures.regenerate_fractal(&context.viewport);
                    };
                });
        });
    }

    fn pipeline(ui: &mut egui::Ui, context: &mut Context) -> Response {
        let shapes = Self::create_shapes(ui, context);
        Self::draw(ui, context, shapes)
    }

    fn create_shapes(_ui: &mut egui::Ui, context: &mut Context) -> Vec<Shape> {
        let mut lines = vec![];

        let grid: Vec<Line2D<Point2D>> = context.figures.grid.lines(&context.viewport);
        let fractal = context.figures.points.clone();

        // Conversion to shapes
        lines.extend(grid);

        let mut shapes = lines
            .iter()
            .map(|line| line.to_pixels(&context.viewport).to_shape())
            .collect::<Vec<Shape>>();

        shapes.extend(fractal);

        shapes
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

        context.viewport.update_state(&response);

        (response, painter)
    }
}
