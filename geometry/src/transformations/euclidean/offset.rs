use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::units::Centimeter;

#[derive(Debug, Default)]
pub struct EuclideanOffset {
    pub is_enabled: bool,

    pub x: Centimeter,
    pub y: Centimeter,

    pub cache_x: Centimeter,
    pub cache_y: Centimeter,
}

impl EuclideanOffset {
    pub fn process_lines(&mut self, lines: &mut [Line2D<Point2D>]) {
        if self.is_enabled {
            self.cache_x += self.x;
            self.cache_y += self.y;
            self.is_enabled = false;
            self.x = Centimeter(0.0);
            self.y = Centimeter(0.0);
        }

        if self.cache_x.0 == 0.0 && self.cache_y.0 == 0.0 {
            return;
        }

        for line in lines.iter_mut() {
            line.start.x += self.cache_x;
            line.start.y += self.cache_y;

            line.end.x += self.cache_x;
            line.end.y += self.cache_y;
        }
    }

    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn reset(&mut self) {
        *self = Default::default();
    }
}
