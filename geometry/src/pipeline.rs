use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::transformations::affine::symmetry::PointSymmetryOperation;
use crate::transformations::euclidean::offset::OffsetOperation;
use crate::transformations::euclidean::rotation::RotationOperation;

#[derive(Debug, Default)]
pub struct Pipeline {
    buffer: Vec<Operation>,
}

impl Pipeline {
    pub fn add_operation(&mut self, operation: Operation) {
        self.buffer.push(operation);
    }

    pub fn do_tasks(&self, lines: &mut [Line2D<Point2D>]) {
        for operation in &self.buffer {
            operation.go(lines);
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    Offset(OffsetOperation),
    Rotation(RotationOperation),
    PointSymmetry(PointSymmetryOperation),
}

impl Operation {
    pub fn go(&self, lines: &mut [Line2D<Point2D>]) {
        match self {
            Self::Offset(operation) => operation.go(lines),
            Self::Rotation(operation) => operation.go(lines),
            Self::PointSymmetry(operation) => operation.go(lines),
        }
    }
}
