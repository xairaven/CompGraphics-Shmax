use crate::primitives::line3d::Line3D;
use crate::primitives::point3d::Point3D;
use crate::units::Centimeter;
use egui::{Color32, Stroke};

#[derive(Debug)]
pub struct Grid3D {
    pub is_enabled: bool,

    pub origin: Point3D,
    pub length: Centimeter,

    pub style: Grid3DStyle,
}

impl Default for Grid3D {
    fn default() -> Self {
        Grid3D {
            is_enabled: true,
            origin: Point3D::new(0.0, 0.0, 0.0),
            length: Centimeter(1.0),
            style: Grid3DStyle::default(),
        }
    }
}

impl Grid3D {
    pub fn lines(&self) -> Vec<Line3D<Point3D>> {
        let x = Line3D {
            start: self.origin,
            end: Point3D {
                x: self.length,
                y: Centimeter(0.0),
                z: Centimeter(0.0),
            },
            stroke: self.style.x_axis,
        };

        let y = Line3D {
            start: self.origin,
            end: Point3D {
                x: Centimeter(0.0),
                y: self.length,
                z: Centimeter(0.0),
            },
            stroke: self.style.y_axis,
        };

        let z = Line3D {
            start: self.origin,
            end: Point3D {
                x: Centimeter(0.0),
                y: Centimeter(0.0),
                z: self.length,
            },
            stroke: self.style.z_axis,
        };

        vec![x, y, z]
    }
}

#[derive(Debug)]
pub struct Grid3DStyle {
    pub x_axis: Stroke,
    pub y_axis: Stroke,
    pub z_axis: Stroke,
}

impl Default for Grid3DStyle {
    fn default() -> Self {
        Grid3DStyle {
            x_axis: Stroke::new(2.0, Color32::RED),
            y_axis: Stroke::new(2.0, Color32::GREEN),
            z_axis: Stroke::new(2.0, Color32::BLUE),
        }
    }
}
