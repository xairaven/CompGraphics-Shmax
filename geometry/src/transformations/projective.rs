use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::units::Centimeter;

#[derive(Debug)]
pub struct Projective {
    pub is_enabled: bool,

    pub xx: f64,
    pub xy: f64,
    pub wx: f64,
    pub yx: f64,
    pub yy: f64,
    pub wy: f64,
    pub zero_x: f64,
    pub zero_y: f64,
    pub w_zero: f64,
}

impl Default for Projective {
    fn default() -> Self {
        Self {
            is_enabled: false,

            xx: 500.0,
            xy: 0.0,
            wx: 2.0,
            yx: 0.0,
            yy: 500.0,
            wy: 2.0,
            zero_x: 0.0,
            zero_y: 0.0,
            w_zero: 500.0,
        }
    }
}

impl Projective {
    pub fn handle(&self, lines: &mut [Line2D<Point2D>]) {
        if !self.is_enabled {
            return;
        }

        for line in lines.iter_mut() {
            self.transform_point(&mut line.start);
            self.transform_point(&mut line.end);
        }
    }

    fn transform_point(&self, point: &mut Point2D) {
        *point = Point2D {
            x: Centimeter(
                (self.zero_x * self.w_zero
                    + self.xx * self.wx * point.x.value()
                    + self.xy * self.wy * point.y.value())
                    / (self.w_zero
                        + self.wx * point.x.value()
                        + self.wy * point.y.value()),
            ),
            y: Centimeter(
                (self.zero_y * self.w_zero
                    + self.yx * self.wx * point.x.value()
                    + self.yy * self.wy * point.y.value())
                    / (self.w_zero
                        + self.wx * point.x.value()
                        + self.wy * point.y.value()),
            ),
        };
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
