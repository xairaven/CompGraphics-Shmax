use crate::context::Context;
use egui::{CentralPanel, Color32, Frame, Painter, Response, Sense, Shape};
use geometry::primitives::line2d::Line2D;
use geometry::primitives::line3d::Line3D;
use geometry::primitives::point2d::Point2D;
use geometry::primitives::point3d::Point3D;

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
        let shapes = Self::create_shapes(ui, context);
        Self::draw(ui, context, shapes)
    }

    fn create_shapes(_ui: &mut egui::Ui, context: &mut Context) -> Vec<Shape> {
        let mut lines = vec![];

        let grid: Vec<Line2D<Point2D>> = context
            .figures
            .grid
            .lines()
            .iter()
            .map(|line| line.to_2d(&context.projections.twopoint))
            .collect();

        let surface: Vec<Line3D<Point3D>> = context.figures.surface.lines();
        let mut pivot = context.figures.surface.pivot_point();

        let texture: Vec<Line3D<Point3D>> = context
            .figures
            .surface
            .handle_texture(&context.figures.texture);

        let mut lines3d = [surface, texture].concat();

        context
            .transformations
            .offset
            .handle(vec![&mut context.pipelines.surface]);
        context
            .transformations
            .rotation
            .handle(vec![&mut context.pipelines.surface]);

        context.pipelines.surface.do_tasks(&mut lines3d, &mut pivot);

        // Conversion to shapes
        let lines3d: Vec<Line2D<Point2D>> = lines3d
            .iter()
            .map(|line| line.to_2d(&context.projections.twopoint))
            .collect();

        lines.extend(grid);
        lines.extend(lines3d);

        lines
            .iter()
            .map(|line| line.to_pixels(&context.viewport).to_shape())
            .collect::<Vec<Shape>>()
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
