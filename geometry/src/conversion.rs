use crate::primitives::point2d::{Point2D, Point2DPixel};
use crate::units::{Centimeter, Pixel};
use crate::viewport::Viewport;

impl Centimeter {
    pub fn to_pixels(self, viewport: &Viewport) -> Pixel {
        Pixel(
            self.value() / viewport.geometry.unit_length.value()
                * viewport.geometry.pixels_per_centimeter,
        )
    }
}

impl Pixel {
    pub fn to_centimeters(self, viewport: &Viewport) -> Centimeter {
        Centimeter(
            self.value() / viewport.geometry.pixels_per_centimeter
                * viewport.geometry.unit_length.value(),
        )
    }
}

impl Point2D {
    pub fn to_pixels(self, viewport: &Viewport) -> Point2DPixel {
        let x = viewport.state.zero_point.x.value()
            + (self.x.value() / viewport.geometry.unit_length.value()
                * viewport.geometry.pixels_per_centimeter)
            + viewport.geometry.offset.x.value();

        let y = viewport.state.zero_point.y.value()
            - (self.y.value() / viewport.geometry.unit_length.value()
                * viewport.geometry.pixels_per_centimeter)
            + viewport.geometry.offset.y.value();

        Point2DPixel::new(x, y)
    }
}

impl Point2DPixel {
    pub fn to_centimeters(self, viewport: &Viewport) -> Point2D {
        let x = (self.x.value() * viewport.geometry.unit_length.value())
            / viewport.geometry.pixels_per_centimeter
            - viewport.state.zero_point.x.value()
            + viewport.geometry.offset.x.value();

        let y = (self.y.value() * viewport.geometry.unit_length.value())
            / viewport.geometry.pixels_per_centimeter
            + viewport.state.zero_point.y.value()
            + viewport.geometry.offset.y.value();

        Point2D::new(x, y)
    }
}
