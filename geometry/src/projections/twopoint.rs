use crate::projections::Projection;
use crate::units::Centimeter;
use nalgebra::Matrix4;

pub struct TwoPointPerspective {
    pub distance: Centimeter,
}

impl Default for TwoPointPerspective {
    fn default() -> Self {
        TwoPointPerspective {
            distance: Centimeter(20.0),
        }
    }
}

impl Projection for TwoPointPerspective {
    fn matrix(&self) -> Matrix4<f64> {
        Matrix4::new(
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
        )
    }
}
