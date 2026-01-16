use crate::primitives::point2d::{Point2D, Point2DPixel};
use crate::primitives::vector2d::{Vector2D, Vector2DPixel};
use crate::units::{Centimeter, Pixel};
use crate::viewport::Viewport;

impl Centimeter {
    pub fn to_pixels_x(self, viewport: &Viewport) -> Pixel {
        let value = viewport.state.zero_point.x.value()
            + (self.value() * viewport.geometry.pixels_per_centimeter)
            + viewport.geometry.offset.x.value();

        Pixel(value)
    }

    pub fn to_pixels_y(self, viewport: &Viewport) -> Pixel {
        let value = viewport.state.zero_point.y.value()
            - (self.value() * viewport.geometry.pixels_per_centimeter)
            + viewport.geometry.offset.y.value();

        Pixel(value)
    }

    pub fn to_pixels_vector_x(self, viewport: &Viewport) -> Pixel {
        Pixel(self.value() * viewport.geometry.pixels_per_centimeter)
    }

    pub fn to_pixels_vector_y(self, viewport: &Viewport) -> Pixel {
        Pixel(-self.value() * viewport.geometry.pixels_per_centimeter)
    }
}

impl Pixel {
    pub fn to_centimeter_vector_x(self, viewport: &Viewport) -> Centimeter {
        Centimeter(self.value() / viewport.geometry.pixels_per_centimeter)
    }

    pub fn to_centimeter_vector_y(self, viewport: &Viewport) -> Centimeter {
        Centimeter(-self.value() / viewport.geometry.pixels_per_centimeter)
    }

    pub fn to_centimeters_x(self, viewport: &Viewport) -> Centimeter {
        let value = (self.value()
            - viewport.state.zero_point.x.value()
            - viewport.geometry.offset.x.value())
            / viewport.geometry.pixels_per_centimeter;

        Centimeter(value)
    }

    pub fn to_centimeters_y(self, viewport: &Viewport) -> Centimeter {
        let value = (-self.value()
            + viewport.state.zero_point.y.value()
            + viewport.geometry.offset.y.value())
            / viewport.geometry.pixels_per_centimeter;

        Centimeter(value)
    }
}

impl Point2D {
    pub fn to_pixels(self, viewport: &Viewport) -> Point2DPixel {
        let x = self.x.to_pixels_x(viewport);
        let y = self.y.to_pixels_y(viewport);

        Point2DPixel { x, y }
    }
}

impl Point2DPixel {
    pub fn to_centimeters(self, viewport: &Viewport) -> Point2D {
        let x = self.x.to_centimeters_x(viewport);
        let y = self.y.to_centimeters_y(viewport);

        Point2D { x, y }
    }
}

impl Vector2D {
    pub fn to_pixels(self, viewport: &Viewport) -> Vector2DPixel {
        Vector2DPixel {
            x: self.x.to_pixels_vector_x(viewport),
            y: self.y.to_pixels_vector_y(viewport),
        }
    }
}

impl Vector2DPixel {
    pub fn to_centimeters(self, viewport: &Viewport) -> Vector2D {
        Vector2D {
            x: self.x.to_centimeter_vector_x(viewport),
            y: self.y.to_centimeter_vector_y(viewport),
        }
    }
}
