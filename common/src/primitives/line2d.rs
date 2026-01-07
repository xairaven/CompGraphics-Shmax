use crate::primitives::point2d::Point2D;
use egui::{Shape, Stroke};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,

    pub stroke: Stroke,
}

impl Line2D {
    pub fn new(start: Point2D, end: Point2D, stroke: Stroke) -> Self {
        Self { start, end, stroke }
    }

    pub fn to_shape(&self) -> Shape {
        Shape::line(vec![self.start.into(), self.end.into()], self.stroke)
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(
            (self.end.x - self.start.x).powf(2.0) + (self.end.y - self.start.y).powf(2.0),
        )
    }

    pub fn with_transparent(start: Point2D, end: Point2D) -> Self {
        Self {
            start,
            end,
            stroke: Stroke::default(),
        }
    }

    pub fn is_transparent(&self) -> bool {
        self.stroke == Stroke::default()
    }
}
