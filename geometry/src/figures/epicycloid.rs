use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::units::Centimeter;
use egui::Stroke;

#[derive(Debug)]
pub struct Epicycloid {
    /// R: Radius of the fixed (center) circle
    pub fixed_radius: Centimeter,

    /// r: Radius of the rolling circle
    pub rolling_radius: Centimeter,

    /// d: Distance from the center of the rolling circle to the drawing pen.
    /// For a strict Epicycloid, this should equal rolling_radius.
    pub pen_offset: Centimeter,

    /// Number of full rotations around the fixed circle (2 * PI * rotations)
    pub rotations: u64,

    /// Step size for t (in radians). Smaller = smoother line, higher cost.
    pub step: f64,

    stroke: Stroke,
}

impl Default for Epicycloid {
    fn default() -> Self {
        Self {
            fixed_radius: Centimeter(100.0),
            rolling_radius: Centimeter(20.0),
            pen_offset: Centimeter(20.0), // Defaults to strict Epicycloid (d = r)
            rotations: 5,
            step: 0.05,

            stroke: Stroke::new(1.0, egui::Color32::PURPLE),
        }
    }
}

impl Epicycloid {
    pub fn lines(&self) -> Vec<Line2D<Point2D>> {
        let mut lines = vec![];

        let max_angle = self.rotations as f64 * 2.0 * std::f64::consts::PI;
        let mut t = 0.0;

        // Calculate the starting point
        let mut current_pos = self.get_point(0.0);

        while t < max_angle {
            t += self.step;

            // Calculate next point
            let next_pos = self.get_point(t);

            // Create a line segment from previous point to next point
            // Assuming Line2D::new(start, end) exists
            lines.push(Line2D::new(current_pos, next_pos, self.stroke));

            current_pos = next_pos;
        }

        lines
    }

    /// Helper to calculate a single point at angle t
    fn get_point(&self, t: f64) -> Point2D {
        // Pre-calculate sum of radii to reduce ops
        let sum_r = self.fixed_radius + self.rolling_radius;
        let k = sum_r.value() / self.rolling_radius.value();

        // Parametric equations:
        // x = (R + r) * cos(t) - d * cos(((R + r) / r) * t)
        // y = (R + r) * sin(t) - d * sin(((R + r) / r) * t)

        // Assuming Point2D has a generic constructor or public fields.
        // Adjust 'new' to whatever your Point2D implementation uses.
        Point2D {
            x: sum_r * t.cos() - self.pen_offset * (k * t).cos(),
            y: sum_r * t.sin() - self.pen_offset * (k * t).sin(),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
