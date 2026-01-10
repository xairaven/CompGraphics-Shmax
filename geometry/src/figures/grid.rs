use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::units::{Centimeter, Pixel};
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
    fn find_minimum(
        screen_bound: Pixel, user_bound: Option<Centimeter>, viewport: &Viewport,
    ) -> Centimeter {
        let screen_bound = screen_bound.to_centimeters(viewport);
        user_bound.map_or(screen_bound, |b| Centimeter(b.min(screen_bound.value())))
    }

    pub fn find_minimums(&self, viewport: &Viewport) -> (Centimeter, Centimeter) {
        (
            Self::find_minimum(viewport.state.bounds.minimum_x, self.x.0, viewport),
            Self::find_minimum(viewport.state.bounds.minimum_y, self.x.0, viewport),
        )
    }

    pub fn find_maximums(&self, viewport: &Viewport) -> (Centimeter, Centimeter) {
        (
            Self::find_minimum(viewport.state.bounds.maximum_x, self.x.1, viewport),
            Self::find_minimum(viewport.state.bounds.maximum_y, self.x.1, viewport),
        )
    }
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

        let (minimum_x, minimum_y) = self.bounds.find_minimums(viewport);
        let (maximum_x, maximum_y) = self.bounds.find_maximums(viewport);

        let axis_x = Line2D {
            start: Point2D {
                x: minimum_x,
                y: self.origin.y,
            },
            end: Point2D {
                x: maximum_x,
                y: self.origin.y,
            },
            stroke: self.axes_strokes.0,
        };
        lines.push(axis_x);

        let axis_y = Line2D {
            start: Point2D {
                x: self.origin.x,
                y: minimum_y,
            },
            end: Point2D {
                x: self.origin.x,
                y: maximum_y,
            },
            stroke: self.axes_strokes.1,
        };
        lines.push(axis_y);

        lines
    }
}
