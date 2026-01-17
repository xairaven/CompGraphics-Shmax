use crate::primitives::line2d::Line2D;
use crate::primitives::line3d::Line3D;
use crate::primitives::point2d::Point2D;
use crate::primitives::point3d::Point3D;
use crate::transformations::affine::symmetry::PointSymmetryOperation;
use crate::transformations::euclidean::offset::OffsetOperation;
use crate::transformations::euclidean::offset3d::Offset3DOperation;
use crate::transformations::euclidean::rotation::RotationOperation;
use crate::transformations::euclidean::rotation3d::Rotation3DOperation;

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

    pub fn make_tasks(&mut self, lines: &mut [Line2D<Point2D>]) {
        self.do_tasks(lines);
        self.clear();
    }

    pub fn make_tasks_point(&mut self, point: &mut Point2D) {
        self.do_tasks_point(point);
        self.clear();
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
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

#[derive(Debug, Default)]
pub struct Pipeline3D {
    buffer: Vec<Operation3D>,
}

impl Pipeline3D {
    pub fn add_operation(&mut self, operation: Operation3D) {
        self.buffer.push(operation);
    }

    pub fn do_tasks(&self, lines: &mut [Line3D<Point3D>], pivot: &mut Point3D) {
        for operation in &self.buffer {
            operation.go(lines, pivot);
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

#[derive(Debug)]
pub enum Operation3D {
    Offset(Offset3DOperation),
    Rotation(Rotation3DOperation),
}

impl Operation3D {
    pub fn go(&self, lines: &mut [Line3D<Point3D>], pivot: &mut Point3D) {
        match self {
            Self::Offset(operation) => {
                for line in lines.iter_mut() {
                    operation.go(&mut line.start);
                    operation.go(&mut line.end);
                }
                operation.go(pivot);
            },
            Self::Rotation(operation) => {
                for line in lines.iter_mut() {
                    operation.go(&mut line.start, pivot);
                    operation.go(&mut line.end, pivot);
                }
            },
        }
    }
}
