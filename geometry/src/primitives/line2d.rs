use crate::primitives::point2d::{Point2D, Point2DPixel, Pointable2D};
use crate::viewport::Viewport;
use egui::{Shape, Stroke};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2D<T>
where
    T: Pointable2D,
{
    pub start: T,
    pub end: T,

    pub stroke: Stroke,
}

impl<T> Line2D<T>
where
    T: Pointable2D,
{
    pub fn new(start: T, end: T, stroke: Stroke) -> Self {
        Self { start, end, stroke }
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(
            (self.end.x() - self.start.x()).powf(2.0)
                + (self.end.y() - self.start.y()).powf(2.0),
        )
    }

    pub fn with_transparent(start: T, end: T) -> Self {
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

impl Line2D<Point2D> {
    pub fn to_pixels(self, viewport: &Viewport) -> Line2D<Point2DPixel> {
        Line2D {
            start: self.start.to_pixels(viewport),
            end: self.end.to_pixels(viewport),
            stroke: self.stroke,
        }
    }
}

impl Line2D<Point2DPixel> {
    pub fn to_centimeters(self, viewport: &Viewport) -> Line2D<Point2D> {
        Line2D {
            start: self.start.to_centimeters(viewport),
            end: self.end.to_centimeters(viewport),
            stroke: self.stroke,
        }
    }

    pub fn to_shape(&self) -> Shape {
        Shape::line(vec![self.start.into(), self.end.into()], self.stroke)
    }
}
