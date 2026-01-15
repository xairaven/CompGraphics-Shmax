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

    pub fn do_tasks_point(&self, point: &mut Point2D) {
        for operation in &self.buffer {
            operation.go_point(point);
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
        for line in lines.iter_mut() {
            self.go_point(&mut line.start);
            self.go_point(&mut line.end);
        }
    }

    pub fn go_point(&self, point: &mut Point2D) {
        match self {
            Self::Offset(operation) => operation.go(point),
            Self::Rotation(operation) => operation.go(point),
            Self::PointSymmetry(operation) => operation.go(point),
        }
    }
}
