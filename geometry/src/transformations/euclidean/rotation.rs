use crate::math::angle::Angle;
use crate::metadata::dot::DotMetadata;
use crate::pipeline::{Operation, Pipeline};
use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::units::Centimeter;
use crate::viewport::Viewport;
use egui::{Color32, Shape, Stroke};
use nalgebra::Matrix3;

#[derive(Debug, Default)]
pub struct EuclideanRotation {
    pub is_enabled: bool,

    pub x: Centimeter,
    pub y: Centimeter,
    pub angle: f64,
}

impl EuclideanRotation {
    pub fn handle(&mut self, operators: Vec<&mut Pipeline>) {
        if !self.is_enabled {
            return;
        }

        for pipeline in operators {
            pipeline.add_operation(self.create_operation())
        }

        self.reset();
    }

    pub fn draw_dot(&self, viewport: &Viewport) -> Option<Shape> {
        if self.x.0 == 0.0 && self.y.0 == 0.0 {
            return None;
        }

        let point = Point2D {
            x: self.x,
            y: self.y,
        }
        .to_pixels(viewport)
        .to_dot(&DotMetadata {
            radius: 5.0,
            fill: Color32::RED,
            stroke: Stroke::new(0.5, Color32::BLACK),
        });

        Some(point)
    }

    fn create_operation(&mut self) -> Operation {
        Operation::Rotation(RotationOperation {
            pivot: Point2D {
                x: self.x,
                y: self.y,
            },
            angle: Angle::from_degree(self.angle),
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
pub struct RotationOperation {
    pub pivot: Point2D,
    pub angle: Angle,
}

impl RotationOperation {
    pub fn go(&self, lines: &mut [Line2D<Point2D>]) {
        for line in lines.iter_mut() {
            self.rotate(&mut line.start);
            self.rotate(&mut line.end);
        }
    }

    pub fn rotate(&self, point: &mut Point2D) {
        let radian = self.angle.radian();

        let vector = point.to_vector();
        let matrix = Matrix3::new(
            f64::cos(radian),
            f64::sin(radian),
            0.0,
            -f64::sin(radian),
            f64::cos(radian),
            0.0,
            (-self.pivot.x * (f64::cos(radian) - 1.0) + self.pivot.y * f64::sin(radian))
                .into(),
            (-self.pivot.y * (f64::cos(radian) - 1.0) - self.pivot.x * f64::sin(radian))
                .into(),
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
