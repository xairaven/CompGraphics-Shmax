use crate::math::angle::Angle;
use crate::primitives::point2d::Point2D;
use crate::units::Centimeter;

#[derive(Debug, Clone, Copy)]
pub enum ShapeType {
    // A complete closed circle
    Full,
    // A semicircle defined by a rotation angle (in radians)
    // The angle usually points perpendicular to the cut diameter
    Semi { angle: Angle },
}

#[derive(Debug, Clone, Copy)]
pub struct CircularShape {
    pub center: Point2D,
    pub radius: Centimeter,
    pub shape_type: ShapeType,
}

impl CircularShape {
    pub fn circle(center: Point2D, radius: Centimeter) -> Self {
        Self {
            center,
            radius,
            shape_type: ShapeType::Full,
        }
    }

    pub fn semicircle(center: Point2D, radius: Centimeter, angle: Angle) -> Self {
        Self {
            center,
            radius,
            shape_type: ShapeType::Semi { angle },
        }
    }
}
