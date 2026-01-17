use crate::projections::Projection;
use nalgebra::Matrix4;

#[derive(Debug)]
pub struct TwoPointPerspective {
    // Coefficient for the vanishing point along the X axis (left/right wall)
    pub q: f64,
    // Coefficient for the vanishing point along the Z axis (depth)
    pub r: f64,
}

impl Default for TwoPointPerspective {
    fn default() -> Self {
        Self { q: 0.002, r: 0.002 }
    }
}

impl Projection for TwoPointPerspective {
    fn matrix(&self) -> Matrix4<f64> {
        Matrix4::new(
            1.0, 0.0, 0.0, self.q, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, self.r, 0.0, 0.0,
            0.0, 1.0,
        )
    }
}
