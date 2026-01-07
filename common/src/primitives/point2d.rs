use crate::math::angle::Angle;
use crate::settings::dot::DotSettings;
use crate::settings::shape::ShapeSettings;
use crate::space::Space;
use egui::epaint::CircleShape;
use egui::{Pos2, Shape};
use nalgebra::{Matrix3, SMatrix};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,

    space: Space,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Point2D {
            x,
            y,
            space: Space::Local,
        }
    }

    pub fn to_vector(&self) -> SMatrix<f64, 1, 3> {
        SMatrix::<f64, 1, 3>::new(self.x, self.y, 1.0)
    }

    pub fn to_uv(&self, unit_length: f64) -> Self {
        let factor = (std::f64::consts::PI / 6.0) / unit_length;

        Self::new(self.x * factor, self.y * factor)
    }

    pub fn scale(self, scale_factor: f64) -> Self {
        Self::new(self.x * scale_factor, self.y * scale_factor)
    }

    pub fn offset(self, dx: f64, dy: f64) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }

    pub fn rotate(self, angle: Angle, pivot: Point2D) -> Self {
        let radian = angle.radian();

        let vector = self.to_vector();
        let matrix = Matrix3::new(
            f64::cos(radian),
            f64::sin(radian),
            0.0,
            -f64::sin(radian),
            f64::cos(radian),
            0.0,
            -pivot.x * (f64::cos(radian) - 1.0) + pivot.y * f64::sin(radian),
            -pivot.y * (f64::cos(radian) - 1.0) - pivot.x * f64::sin(radian),
            1.0,
        );

        let result = vector * matrix;

        Self {
            x: result.x,
            y: result.y,
            space: self.space,
        }
    }

    pub fn to_shape(self, settings: &ShapeSettings) -> Shape {
        Shape::circle_filled(self.into(), settings.radius, settings.color)
    }

    pub fn to_dot(self, settings: &DotSettings) -> Shape {
        let mut shape = CircleShape::filled(self.into(), settings.radius, settings.fill);
        shape.stroke = settings.stroke;

        Shape::Circle(shape)
    }

    pub fn space(&self) -> Space {
        self.space
    }

    pub fn with_space_local(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            space: Space::Local,
        }
    }

    pub fn with_space_screen(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            space: Space::Screen,
        }
    }
}

impl From<Point2D> for Pos2 {
    fn from(point: Point2D) -> Self {
        Pos2::from([point.x as f32, point.y as f32])
    }
}

impl From<Pos2> for Point2D {
    fn from(pos: Pos2) -> Self {
        Point2D {
            x: pos.x as f64,
            y: pos.y as f64,
            space: Space::Local,
        }
    }
}
