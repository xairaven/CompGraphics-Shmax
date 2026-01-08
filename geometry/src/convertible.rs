use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::space::{Space, SpaceContext};
use egui::Vec2;

pub trait Convertible {
    fn centimeters_to_pixels(self, context: &SpaceContext) -> Self;
    fn pixels_to_centimeters(self, context: &SpaceContext) -> Self;
}

impl Convertible for f64 {
    fn centimeters_to_pixels(self, context: &SpaceContext) -> f64 {
        let value = self;
        value / context.settings.unit_length * context.state.pixels_per_centimeter
    }

    fn pixels_to_centimeters(self, context: &SpaceContext) -> f64 {
        let value = self;
        value / context.state.pixels_per_centimeter * context.settings.unit_length
    }
}

impl Convertible for Point2D {
    fn centimeters_to_pixels(self, context: &SpaceContext) -> Point2D {
        debug_assert!(self.space() == Space::Local);

        let x = context.settings.zero_point.x
            + (self.x / context.settings.unit_length
                * context.state.pixels_per_centimeter)
            + context.state.offset.0;
        let y = context.settings.zero_point.y
            - (self.y / context.settings.unit_length
                * context.state.pixels_per_centimeter)
            + context.state.offset.1;

        Point2D::new(x, y).with_space_screen()
    }

    fn pixels_to_centimeters(self, context: &SpaceContext) -> Point2D {
        debug_assert!(self.space() == Space::Screen);

        let x = (self.x * context.settings.unit_length
            / context.state.pixels_per_centimeter)
            - context.settings.zero_point.x
            + context.state.offset.0;
        let y = (self.y * context.settings.unit_length
            / context.state.pixels_per_centimeter)
            + context.settings.zero_point.y
            + context.state.offset.1;

        Point2D::new(x, y).with_space_local()
    }
}

impl Convertible for Line2D {
    fn centimeters_to_pixels(self, context: &SpaceContext) -> Self {
        Self {
            start: self.start.centimeters_to_pixels(context),
            end: self.end.centimeters_to_pixels(context),
            stroke: self.stroke,
        }
    }

    fn pixels_to_centimeters(self, context: &SpaceContext) -> Self {
        Self {
            start: self.start.pixels_to_centimeters(context),
            end: self.end.pixels_to_centimeters(context),
            stroke: self.stroke,
        }
    }
}

impl Convertible for Vec2 {
    fn centimeters_to_pixels(self, context: &SpaceContext) -> Self {
        let x = self.x as f64;
        let x = x.centimeters_to_pixels(context) as f32;

        let y = self.y as f64;
        let y = y.centimeters_to_pixels(context) as f32;

        Vec2::new(x, -y)
    }

    fn pixels_to_centimeters(self, context: &SpaceContext) -> Self {
        let x = self.x as f64;
        let x = x.pixels_to_centimeters(context) as f32;

        let y = self.y as f64;
        let y = y.pixels_to_centimeters(context) as f32;

        Vec2::new(x, -y)
    }
}
