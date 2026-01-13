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
                    ui.input(|i| context.viewport.handle_scroll(i));
                    let response = Self::pipeline(ui, context);
                    context.viewport.handle_pan(ui, response);
                });
        });
    }

    fn pipeline(ui: &mut egui::Ui, context: &mut Context) -> Response {
        context.fixating_grid();
        let shapes = Self::create_shapes(ui, context);
        Self::draw(ui, context, shapes)
    }

    fn create_shapes(_ui: &mut egui::Ui, context: &mut Context) -> Vec<Shape> {
        let mut lines = vec![];

        let mut grid: Vec<Line2D<Point2D>> =
            context.figures.grid.lines(&context.viewport);

        let mut detail = context.figures.detail.lines();

        context
            .transformations
            .offset
            .handle(vec![&mut context.figures.detail_pipeline]);
        context
            .transformations
            .rotation
            .handle(vec![&mut context.figures.detail_pipeline]);

        context.figures.detail_pipeline.do_tasks(&mut detail);

        // Other transformations that applied, but not saved
        context.transformations.affine.handle(&mut grid);
        context.transformations.affine.handle(&mut detail);
        context.transformations.scale.handle(&mut grid);
        context.transformations.scale.handle(&mut detail);

        // Conversion to shapes
        lines.extend(grid);
        lines.extend(detail);

        lines
            .iter()
            .map(|line| line.to_pixels(&context.viewport).to_shape())
            .collect()
    }

    fn draw(ui: &mut egui::Ui, context: &mut Context, shapes: Vec<Shape>) -> Response {
        let (response, painter) = Self::initialize_painter(ui, context);
        painter.extend(shapes);

        let rotation_dot = context.transformations.rotation.draw_dot(&context.viewport);
        if let Some(dot) = rotation_dot {
            painter.add(dot);
        }

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
