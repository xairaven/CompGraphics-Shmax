use crate::pipeline::{Operation, Pipeline};
use crate::primitives::point2d::Point2D;
use crate::units::Centimeter;

#[derive(Debug, Default)]
pub struct EuclideanOffset {
    pub is_enabled: bool,

    pub x: Centimeter,
    pub y: Centimeter,
}

impl EuclideanOffset {
    pub fn handle(&mut self, operators: Vec<&mut Pipeline>) {
        if !self.is_enabled {
            return;
        }

        for pipeline in operators {
            pipeline.add_operation(self.create_operation())
        }

        self.reset();
    }

    fn create_operation(&mut self) -> Operation {
        Operation::Offset(OffsetOperation {
            x: self.x,
            y: self.y,
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
pub struct OffsetOperation {
    pub x: Centimeter,
    pub y: Centimeter,
}

impl OffsetOperation {
    pub fn go(&self, point: &mut Point2D) {
        point.x += self.x;
        point.y += self.y;
    }
}
