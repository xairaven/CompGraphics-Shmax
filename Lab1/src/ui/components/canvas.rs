use crate::context::Context;
use egui::{CentralPanel, Color32, Frame, Painter, Response, Sense, Shape};
use geometry::primitives::line2d::Line2D;
use geometry::primitives::point2d::Point2D;
use geometry::transformations::affine::symmetry::AffinePointSymmetry;
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
        context
            .transformations
            .symmetry
            .handle(vec![&mut context.figures.detail_pipeline]);

        context.figures.detail_pipeline.do_tasks(&mut detail);

        let rotation_point = context.transformations.rotation.leading_point();
        let symmetry_point = context.transformations.symmetry.leading_point();

        // Other transformations that applied, but not saved
        Self::global_transform_lines(&mut detail, context);
        Self::global_transform_lines(&mut grid, context);

        // Conversion to shapes
        lines.extend(grid);
        lines.extend(detail);

        let mut shapes = lines
            .iter()
            .map(|line| line.to_pixels(&context.viewport).to_shape())
            .collect::<Vec<Shape>>();

        // Rotation point
        if let Some(mut dot) = rotation_point {
            Self::global_transform_point(&mut dot, context);
            shapes.push(EuclideanRotation::leading_shape(dot, &context.viewport));
        }
        // Symmetry point
        if let Some(mut dot) = symmetry_point {
            Self::global_transform_point(&mut dot, context);
            shapes.push(AffinePointSymmetry::leading_shape(dot, &context.viewport));
        }

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

    fn global_transform_lines(lines: &mut [Line2D<Point2D>], context: &mut Context) {
        context.transformations.affine.handle(lines);
        context.transformations.scale.handle(lines);
        context.transformations.projective.handle(lines);
    }

    fn global_transform_point(point: &mut Point2D, context: &mut Context) {
        context.transformations.affine.transform_point(point);
        context.transformations.scale.transform_point(point);
        context.transformations.projective.transform_point(point);
    }
}
