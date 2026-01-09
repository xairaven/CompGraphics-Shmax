use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::viewport::Viewport;
use egui::Stroke;

pub mod defaults {
    use egui::{Color32, Stroke};

    pub const AXIS_RED: Stroke = Stroke {
        width: 2.0,
        color: Color32::RED,
    };
    pub const AXIS_GREEN: Stroke = Stroke {
        width: 2.0,
        color: Color32::GREEN,
    };
    pub const GRID_GRAY: Stroke = Stroke {
        width: 0.8,
        color: Color32::GRAY,
    };
}

pub struct Grid2DBuilder {
    pub is_negative_enabled: bool,

    pub origin: Point2D,
    pub units: Point2D,

    pub axes_strokes: (Stroke, Stroke),
    pub grid_stroke: Stroke,
}

impl Default for Grid2DBuilder {
    fn default() -> Self {
        Self {
            is_negative_enabled: true,
            origin: Point2D::new(0.0, 0.0),
            units: Point2D::new(1.0, 1.0),
            axes_strokes: (defaults::AXIS_RED, defaults::AXIS_GREEN),
            grid_stroke: defaults::GRID_GRAY,
        }
    }
}

impl Grid2DBuilder {
    pub fn with_origin(mut self, origin: Point2D) -> Self {
        self.origin = origin;
        self
    }

    pub fn with_units(mut self, unit_x: f64, unit_y: f64) -> Self {
        self.units = Point2D::new(unit_x, unit_y);
        self
    }

    pub fn with_axes_strokes(mut self, stroke_x: Stroke, stroke_y: Stroke) -> Self {
        self.axes_strokes = (stroke_x, stroke_y);
        self
    }

    pub fn with_grid_stroke(mut self, stroke: Stroke) -> Self {
        self.grid_stroke = stroke;
        self
    }

    pub fn with_negative_enabled(mut self, is_enabled: bool) -> Self {
        self.is_negative_enabled = is_enabled;
        self
    }

    pub fn build(self) -> Grid2D {
        Grid2D {
            is_enabled: true,
            is_negative_enabled: self.is_negative_enabled,
            origin: self.origin,
            units: self.units,
            axes_strokes: self.axes_strokes,
            grid_stroke: self.grid_stroke,
        }
    }
}

pub struct Grid2D {
    pub is_enabled: bool,
    pub is_negative_enabled: bool,

    pub origin: Point2D,
    pub units: Point2D,

    pub axes_strokes: (Stroke, Stroke),
    pub grid_stroke: Stroke,
}

impl Grid2D {
    pub fn lines(&self, viewport: &Viewport) -> Vec<Line2D<Point2D>> {
        let mut lines = vec![];

        lines
    }
}
