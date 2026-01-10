use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::units::Centimeter;
use crate::viewport::Viewport;
use egui::Stroke;

pub mod defaults {
    use crate::primitives::point2d::Point2D;
    use crate::units::Centimeter;
    use egui::{Color32, Stroke};

    pub const ORIGIN: Point2D = Point2D {
        x: Centimeter(0.0),
        y: Centimeter(0.0),
    };
    pub const UNITS: Point2D = Point2D {
        x: Centimeter(1.0),
        y: Centimeter(1.0),
    };

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
    pub origin: Point2D,
    pub units: Point2D,

    pub bounds_x: (Option<Centimeter>, Option<Centimeter>),
    pub bounds_y: (Option<Centimeter>, Option<Centimeter>),

    pub axes_strokes: (Stroke, Stroke),
    pub grid_stroke: Stroke,
}

impl Default for Grid2DBuilder {
    fn default() -> Self {
        Self {
            origin: defaults::ORIGIN,
            units: defaults::UNITS,

            bounds_x: (None, None),
            bounds_y: (None, None),

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

    pub fn with_bounds_x(
        mut self, min: Option<Centimeter>, max: Option<Centimeter>,
    ) -> Self {
        self.bounds_x = (min, max);
        self
    }

    pub fn with_bounds_y(
        mut self, min: Option<Centimeter>, max: Option<Centimeter>,
    ) -> Self {
        self.bounds_y = (min, max);
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

    pub fn build(self) -> Grid2D {
        Grid2D {
            is_enabled: true,
            origin: self.origin,
            units: self.units,
            bounds: GridBounds {
                x: (self.bounds_x.0, self.bounds_x.1),
                y: (self.bounds_y.0, self.bounds_y.1),
            },
            axes_strokes: self.axes_strokes,
            grid_stroke: self.grid_stroke,
        }
    }
}

pub struct GridBounds {
    pub x: (Option<Centimeter>, Option<Centimeter>),
    pub y: (Option<Centimeter>, Option<Centimeter>),
}

impl GridBounds {
    pub fn view_bounds(&self, viewport: &Viewport) -> ViewGridBounds {
        // Canvas bounds in centimeters
        let v = viewport.state.bounds.to_centimeters(viewport);

        ViewGridBounds {
            minimum_x: self
                .x
                .0
                .map_or(v.minimum_x, |c| Centimeter(c.max(v.minimum_x.value()))),
            maximum_x: self
                .x
                .1
                .map_or(v.maximum_x, |c| Centimeter(c.min(v.maximum_x.value()))),
            minimum_y: self
                .y
                .0
                .map_or(v.minimum_y, |c| Centimeter(c.max(v.minimum_y.value()))),
            maximum_y: self
                .y
                .1
                .map_or(v.maximum_y, |c| Centimeter(c.min(v.maximum_y.value()))),
        }
    }
}

pub struct ViewGridBounds {
    pub minimum_x: Centimeter,
    pub maximum_x: Centimeter,
    pub minimum_y: Centimeter,
    pub maximum_y: Centimeter,
}

pub struct Grid2D {
    pub is_enabled: bool,

    pub origin: Point2D,
    pub units: Point2D,

    pub bounds: GridBounds,

    pub axes_strokes: (Stroke, Stroke),
    pub grid_stroke: Stroke,
}

impl Grid2D {
    pub fn lines(&self, viewport: &Viewport) -> Vec<Line2D<Point2D>> {
        let mut lines = vec![];

        if !self.is_enabled {
            return lines;
        }

        // Minimum and maximum bounds in centimeters for the viewport, clamped by the grid bounds
        let view_bounds = self.bounds.view_bounds(viewport);

        let axis_x = Line2D {
            start: Point2D {
                x: view_bounds.minimum_x,
                y: self.origin.y,
            },
            end: Point2D {
                x: view_bounds.maximum_x,
                y: self.origin.y,
            },
            stroke: self.axes_strokes.0,
        };
        lines.push(axis_x);

        let axis_y = Line2D {
            start: Point2D {
                x: self.origin.x,
                y: view_bounds.minimum_y,
            },
            end: Point2D {
                x: self.origin.x,
                y: view_bounds.maximum_y,
            },
            stroke: self.axes_strokes.1,
        };
        lines.push(axis_y);

        lines
    }
}
