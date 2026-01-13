use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;

#[derive(Debug)]
pub struct AffineScaling {
    pub is_enabled: bool,

    pub mx: f64,
    pub my: f64,
}

impl Default for AffineScaling {
    fn default() -> Self {
        Self {
            is_enabled: false,
            mx: 1.0,
            my: 1.0,
        }
    }
}

impl AffineScaling {
    pub fn handle(&self, lines: &mut [Line2D<Point2D>]) {
        if !self.is_enabled {
            return;
        }

        for line in lines.iter_mut() {
            self.transform_point(&mut line.start);
            self.transform_point(&mut line.end);
        }
    }

    pub fn transform_point(&self, point: &mut Point2D) {
        if !self.is_enabled {
            return;
        }

        point.x *= self.mx;
        point.y *= self.my;
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
