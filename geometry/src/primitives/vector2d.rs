use crate::units::{Centimeter, Pixel};
use egui::Vec2;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector2D {
    pub x: Centimeter,
    pub y: Centimeter,
}

impl Vector2D {
    pub const fn zero() -> Self {
        Self {
            x: Centimeter(0.0),
            y: Centimeter(0.0),
        }
    }
}

/// Defines "vector" of point - for example, if we need drag delta, we don't care
/// about current screen system offset, whatever
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector2DPixel {
    pub x: Pixel,
    pub y: Pixel,
}

impl From<Vec2> for Vector2DPixel {
    fn from(value: Vec2) -> Self {
        Self {
            x: Pixel(value.x as f64),
            y: Pixel(value.y as f64),
        }
    }
}
