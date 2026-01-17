use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::primitives::point3d::Pointable3D;
use crate::projections::Projection;
use egui::Stroke;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line3D<T>
where
    T: Pointable3D,
{
    pub start: T,
    pub end: T,

    pub stroke: Stroke,
}

impl<T> Line3D<T>
where
    T: Pointable3D,
{
    pub fn new(start: T, end: T, stroke: Stroke) -> Self {
        Self { start, end, stroke }
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

    pub fn to_2d<M: Projection>(&self, projection: &M) -> Line2D<Point2D> {
        Line2D {
            start: self.start.to_2d(projection),
            end: self.end.to_2d(projection),
            stroke: self.stroke,
        }
    }
}
