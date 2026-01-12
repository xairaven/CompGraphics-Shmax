use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::transformations::euclidean::offset::OffsetOperation;

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
}

impl Operation {
    pub fn go(&self, lines: &mut [Line2D<Point2D>]) {
        match self {
            Self::Offset(offset) => offset.go(lines),
        }
    }
}
