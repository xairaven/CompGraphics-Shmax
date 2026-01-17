use crate::pipeline::{Operation3D, Pipeline3D};
use crate::primitives::point3d::Point3D;
use crate::units::Centimeter;

#[derive(Debug, Default)]
pub struct EuclideanOffset3D {
    pub is_enabled: bool,

    pub x: Centimeter,
    pub y: Centimeter,
    pub z: Centimeter,
}

impl EuclideanOffset3D {
    pub fn handle(&mut self, operators: Vec<&mut Pipeline3D>) {
        if !self.is_enabled {
            return;
        }

        for pipeline in operators {
            pipeline.add_operation(self.create_operation())
        }

        self.reset();
    }

    fn create_operation(&mut self) -> Operation3D {
        Operation3D::Offset(Offset3DOperation {
            x: self.x,
            y: self.y,
            z: self.z,
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
pub struct Offset3DOperation {
    pub x: Centimeter,
    pub y: Centimeter,
    pub z: Centimeter,
}

impl Offset3DOperation {
    pub fn go(&self, point: &mut Point3D) {
        point.x += self.x;
        point.y += self.y;
        point.z += self.z;
    }
}
