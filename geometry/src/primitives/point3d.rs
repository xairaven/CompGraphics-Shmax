use crate::primitives::point2d::Point2D;
use crate::projections::Projection;
use crate::units::Centimeter;
use nalgebra::SMatrix;

pub trait Pointable3D: Clone {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn to_2d<M: Projection>(&self, projection: &M) -> Point2D;
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: Centimeter,
    pub y: Centimeter,
    pub z: Centimeter,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: Centimeter(x),
            y: Centimeter(y),
            z: Centimeter(z),
        }
    }

    pub const fn zero() -> Self {
        Self {
            x: Centimeter(0.0),
            y: Centimeter(0.0),
            z: Centimeter(0.0),
        }
    }

    pub fn to_vector(&self) -> SMatrix<f64, 1, 4> {
        SMatrix::<f64, 1, 4>::new(self.x.value(), self.y.value(), self.z.value(), 1.0)
    }
}

impl Pointable3D for Point3D {
    fn x(&self) -> f64 {
        self.x.value()
    }

    fn y(&self) -> f64 {
        self.y.value()
    }

    fn z(&self) -> f64 {
        self.z.value()
    }

    fn to_2d<M: Projection>(&self, projection: &M) -> Point2D {
        let vector = self.to_vector() * projection.matrix();

        Point2D::new(vector.x, vector.y)
    }
}
