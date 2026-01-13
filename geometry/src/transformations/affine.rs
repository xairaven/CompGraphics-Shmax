use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::units::Centimeter;
use nalgebra::Matrix3;

#[derive(Debug)]
pub struct Affine {
    pub is_enabled: bool,

    pub xx: Centimeter,
    pub xy: Centimeter,
    pub yx: Centimeter,
    pub yy: Centimeter,
    pub zero_x: Centimeter,
    pub zero_y: Centimeter,
}

impl Default for Affine {
    fn default() -> Self {
        Self {
            is_enabled: false,

            xx: Centimeter(1.0),
            xy: Centimeter(0.0),
            yx: Centimeter(0.0),
            yy: Centimeter(1.0),
            zero_x: Centimeter(0.0),
            zero_y: Centimeter(0.0),
        }
    }
}

impl Affine {
    pub fn handle(&self, lines: &mut [Line2D<Point2D>]) {
        if !self.is_enabled {
            return;
        }

        let matrix = self.matrix();
        for line in lines.iter_mut() {
            self.transform_point(&mut line.start, &matrix);
            self.transform_point(&mut line.end, &matrix);
        }
    }

    fn transform_point(&self, point: &mut Point2D, matrix: &Matrix3<f64>) {
        let vector = point.to_vector();
        let result = vector * matrix;

        *point = Point2D {
            x: Centimeter(result.x),
            y: Centimeter(result.y),
        };
    }

    fn matrix(&self) -> Matrix3<f64> {
        Matrix3::new(
            self.xx.value(),
            self.xy.value(),
            0.0,
            self.yx.value(),
            self.yy.value(),
            0.0,
            self.zero_x.value(),
            self.zero_y.value(),
            1.0,
        )
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
