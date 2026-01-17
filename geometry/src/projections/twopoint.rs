use crate::math::angle::Angle;
use crate::projections::Projection;
use crate::units::Centimeter;
use nalgebra::Matrix4;

#[derive(Debug)]
pub struct TwoPointPerspective {
    pub angle: f64,
    pub distance: Centimeter,
}

impl Default for TwoPointPerspective {
    fn default() -> Self {
        Self {
            angle: 45.0,
            distance: Centimeter(50.0),
        }
    }
}

impl Projection for TwoPointPerspective {
    fn matrix(&self) -> Matrix4<f64> {
        let radians = Angle::from_degree(self.angle).radian();

        // Rotation Y matrix
        let rotation = Matrix4::new(
            radians.cos(),
            0.0,
            radians.sin(),
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            -radians.sin(),
            0.0,
            radians.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        // Perspective matrix
        let perspective = Matrix4::new(
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            -1.0 / self.distance.value(),
            1.0,
        );

        rotation * perspective
    }
}
