use crate::convertible::Convertible;
use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::space::SpaceContext;
use egui::Stroke;

pub mod defaults {
    use egui::{Color32, Stroke};

    pub const UNIT_LENGTH: f64 = 1.0;
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
    pub units: (Point2D, Point2D),

    pub axes_strokes: (Stroke, Stroke),
    pub grid_stroke: Stroke,
}

impl Default for Grid2DBuilder {
    fn default() -> Self {
        Self {
            is_negative_enabled: true,
            origin: Point2D::new(0.0, 0.0),
            units: (
                Point2D::new(defaults::UNIT_LENGTH, 0.0),
                Point2D::new(0.0, defaults::UNIT_LENGTH),
            ),
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

    pub fn with_units(mut self, unit_x: Point2D, unit_y: Point2D) -> Self {
        self.units = (unit_x, unit_y);
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
    pub units: (Point2D, Point2D),

    pub axes_strokes: (Stroke, Stroke),
    pub grid_stroke: Stroke,
}

impl Grid2D {
    pub fn lines(&self, space_context: &SpaceContext) -> Vec<Line2D> {
        // Grid sides (optimization): left & right
        let width = (
            (space_context.settings.size.width - space_context.settings.zero_point.x
                + space_context.state.offset.0)
                .pixels_to_centimeters(space_context),
            (space_context.settings.size.width
                - space_context.settings.zero_point.x
                - space_context.state.offset.0)
                .pixels_to_centimeters(space_context),
        );

        let height = (
            (space_context.settings.size.height
                - space_context.settings.zero_point.y
                - space_context.state.offset.1)
                .pixels_to_centimeters(space_context),
            (space_context.settings.size.height - space_context.settings.zero_point.y
                + space_context.state.offset.1)
                .pixels_to_centimeters(space_context),
        );

        let mut lines = vec![];

        let ticks_x = (
            (width.0 - (width.0 % self.units.0.x)) / self.units.0.x,
            (width.1 - (width.1 % self.units.0.x)) / self.units.0.x,
        );
        let ticks_y = (
            (height.0 - (height.0 % self.units.1.y)) / self.units.1.y,
            (height.1 - (height.1 % self.units.1.y)) / self.units.1.y,
        );

        let axis_x = Line2D {
            start: Point2D::new(
                if self.is_negative_enabled {
                    -width.0
                } else {
                    0.0
                },
                self.units.0.y,
            ),
            end: Point2D::new(width.1, self.units.0.y),
            stroke: self.axes_strokes.0,
        };
        let axis_y = Line2D {
            start: Point2D::new(
                self.units.1.x,
                if self.is_negative_enabled {
                    -height.0
                } else {
                    0.0
                },
            ),
            end: Point2D::new(self.units.1.x, height.1),
            stroke: self.axes_strokes.1,
        };

        // OY Grid
        for i in (-ticks_x.0 as i32)..=(ticks_x.1 as i32) {
            if i == 0 {
                continue;
            }

            let x = self.units.0.x * (i as f64);

            let start = Point2D::new(x, axis_y.start.y);
            let end = Point2D::new(x, axis_y.end.y);

            lines.push(Line2D::new(start, end, self.grid_stroke));
        }

        // OX Grid
        for i in (-ticks_y.0 as i32)..=(ticks_y.1 as i32) {
            if i == 0 {
                continue;
            }

            let y = self.units.1.y * (i as f64);

            let start = Point2D::new(axis_x.start.x, y);
            let end = Point2D::new(axis_x.end.x, y);

            lines.push(Line2D::new(start, end, self.grid_stroke));
        }

        // Pushing main axes
        lines.push(axis_x);
        lines.push(axis_y);

        lines
    }
}
