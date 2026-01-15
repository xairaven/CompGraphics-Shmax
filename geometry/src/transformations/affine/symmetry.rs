use crate::pipeline::{Operation, Pipeline};
use crate::primitives::point2d::Point2D;
use crate::shapes::dot::DotMetadata;
use crate::units::Centimeter;
use crate::viewport::Viewport;
use egui::{Color32, Shape, Stroke};
use nalgebra::Matrix3;

#[derive(Debug, Default)]
pub struct AffinePointSymmetry {
    pub is_enabled: bool,

    pub x: Centimeter,
    pub y: Centimeter,
}

impl AffinePointSymmetry {
    pub fn handle(&mut self, operators: Vec<&mut Pipeline>) {
        if !self.is_enabled {
            return;
        }

        for pipeline in operators {
            pipeline.add_operation(self.create_operation())
        }

        self.reset();
    }

    pub fn leading_point(&self) -> Option<Point2D> {
        if self.x.0 == 0.0 && self.y.0 == 0.0 {
            return None;
        }

        Some(Point2D {
            x: self.x,
            y: self.y,
        })
    }

    pub fn leading_shape(point: Point2D, viewport: &Viewport) -> Shape {
        point.to_pixels(viewport).to_dot(&DotMetadata {
            radius: 5.0,
            fill: Color32::PURPLE,
            stroke: Stroke::new(0.5, Color32::BLACK),
        })
    }

    fn create_operation(&mut self) -> Operation {
        Operation::PointSymmetry(PointSymmetryOperation {
            point: Point2D {
                x: self.x,
                y: self.y,
            },
        })
    }

    pub fn run(&mut self) {
        self.is_enabled = true;
    }

    pub fn reset(&mut self) {
        *self = Default::default();
    }
}

#[derive(Debug)]
pub struct PointSymmetryOperation {
    pub point: Point2D,
}

impl PointSymmetryOperation {
    pub fn go(&self, point: &mut Point2D) {
        self.process_point(point);
    }

    pub fn process_point(&self, point: &mut Point2D) {
        let vector = point.to_vector();
        let matrix = Matrix3::new(
            -1.0,
            0.0,
            0.0,
            0.0,
            -1.0,
            0.0,
            2.0 * self.point.x.value(),
            2.0 * self.point.y.value(),
            1.0,
        );

        let result = vector * matrix;

        let result = Point2D {
            x: Centimeter(result.x),
            y: Centimeter(result.y),
        };

        *point = result;
    }
}
