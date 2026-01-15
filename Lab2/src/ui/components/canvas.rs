use crate::context::Context;
use eframe::epaint::Stroke;
use egui::{CentralPanel, Color32, Frame, Painter, Response, Sense, Shape};
use geometry::primitives::line2d::Line2D;
use geometry::primitives::point2d::Point2D;
use geometry::shapes::dot::DotMetadata;
use geometry::transformations::euclidean::rotation::EuclideanRotation;

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

    fn create_shapes(ui: &mut egui::Ui, context: &mut Context) -> Vec<Shape> {
        let mut lines = vec![];

        let grid: Vec<Line2D<Point2D>> = context.figures.grid.lines(&context.viewport);
        let mut epicycloid = context.figures.epicycloid.lines();

        context.animations.walker.step(ui, &mut epicycloid);

        context
            .transformations
            .offset
            .handle(vec![&mut context.figures.epicycloid_pipeline]);
        context
            .transformations
            .rotation
            .handle(vec![&mut context.figures.epicycloid_pipeline]);

        context
            .figures
            .epicycloid_pipeline
            .do_tasks(&mut epicycloid);

        // Conversion to shapes
        lines.extend(grid);
        lines.extend(epicycloid);

        let mut shapes = lines
            .iter()
            .map(|line| line.to_pixels(&context.viewport).to_shape())
            .collect::<Vec<Shape>>();

        // Rotation point
        if let Some(dot) = context.transformations.rotation.leading_point() {
            shapes.push(EuclideanRotation::leading_shape(dot, &context.viewport));
        }
        // Walker point
        if let Some(dot) = context.animations.walker.dot(&context.viewport) {
            shapes.push(dot);
        }

        shapes
    }

    fn draw(ui: &mut egui::Ui, context: &mut Context, shapes: Vec<Shape>) -> Response {
        let (response, painter) = Self::initialize_painter(ui, context);
        painter.extend(shapes);

        // Animation
        let epicycloid = &mut context.figures.epicycloid;
        context.animations.epicycloid.run(ui, epicycloid);

        let mut additional_lines = vec![];
        if let Some(normal) = context.animations.walker.normal(epicycloid) {
            additional_lines.push(normal);
        }
        if let Some(tangent) = context.animations.walker.tangent(epicycloid) {
            additional_lines.push(tangent);
        }

        for line in additional_lines {
            painter.add(line.to_pixels(&context.viewport).to_shape());
        }

        if context.animations.walker.is_inflection_points_enabled {
            for point in &epicycloid.stats.inflection_points {
                let dot = point.to_pixels(&context.viewport).to_dot(&DotMetadata {
                    radius: 5.0,
                    fill: Color32::BROWN,
                    stroke: Stroke::new(0.5, Color32::BLACK),
                });
                painter.add(dot);
            }
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
