use crate::shapes::dot::DotMetadata;
use crate::shapes::shape::ShapeMetadata;
use crate::units::{Centimeter, Pixel};
use egui::epaint::CircleShape;
use egui::{Pos2, Shape};
use nalgebra::SMatrix;

pub trait Pointable2D: Clone {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: Centimeter,
    pub y: Centimeter,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: Centimeter(x),
            y: Centimeter(y),
        }
    }

    pub fn zero() -> Self {
        Self {
            x: Centimeter(0.0),
            y: Centimeter(0.0),
        }
    }

    pub fn to_vector(self) -> SMatrix<f64, 1, 3> {
        SMatrix::<f64, 1, 3>::new(*self.x, *self.y, 1.0)
    }

    pub fn to_uv(self, unit_length: Centimeter) -> Self {
        let factor = (std::f64::consts::PI / 6.0) / (*unit_length);

        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    pub fn scale(self, scale_factor: f64) -> Self {
        Self {
            x: self.x * scale_factor,
            y: self.y * scale_factor,
        }
    }
}

impl Pointable2D for Point2D {
    fn x(&self) -> f64 {
        *self.x
    }

    fn y(&self) -> f64 {
        *self.y
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point2DPixel {
    pub x: Pixel,
    pub y: Pixel,
}

impl Point2DPixel {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: Pixel(x),
            y: Pixel(y),
        }
    }

    pub fn zero() -> Self {
        Self {
            x: Pixel(0.0),
            y: Pixel(0.0),
        }
    }

    pub fn to_shape(self, metadata: &ShapeMetadata) -> Shape {
        Shape::circle_filled(self.into(), metadata.radius, metadata.color)
    }

    pub fn to_dot(self, metadata: &DotMetadata) -> Shape {
        let mut shape = CircleShape::filled(self.into(), metadata.radius, metadata.fill);
        shape.stroke = metadata.stroke;

        Shape::Circle(shape)
    }
}

impl From<Point2DPixel> for Pos2 {
    fn from(point: Point2DPixel) -> Self {
        Pos2::from([point.x.0 as f32, point.y.0 as f32])
    }
}

impl From<Pos2> for Point2DPixel {
    fn from(pos: Pos2) -> Self {
        Self {
            x: Pixel(pos.x as f64),
            y: Pixel(pos.y as f64),
        }
    }
}

impl Pointable2D for Point2DPixel {
    fn x(&self) -> f64 {
        *self.x
    }

    fn y(&self) -> f64 {
        *self.y
    }
}
