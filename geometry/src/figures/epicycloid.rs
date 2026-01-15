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

    /// Cache
    pub stats: EpicycloidStats,
}

impl Default for Epicycloid {
    fn default() -> Self {
        let mut e = Self {
            fixed_radius: Centimeter(100.0),
            rolling_radius: Centimeter(20.0),
            pen_offset: Centimeter(20.0), // Defaults to strict Epicycloid (d = r)
            rotations: 5,
            step: 0.05,
            stroke: Stroke::new(1.0, egui::Color32::PURPLE),
            stats: EpicycloidStats::default(),
        };
        // Calculate initial stats
        e.calculate_stats();
        e
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

    pub fn calculate_stats(&mut self) {
        let lines = self.lines();

        // Length of arc (sum of lengths..)
        let mut total_length = 0.0;
        // Area (Shoelace formula)
        let mut double_area = 0.0;

        for line in &lines {
            let p1 = line.start;
            let p2 = line.end;

            // Length
            let dx = p1.x.value() - p2.x.value();
            let dy = p1.y.value() - p2.y.value();
            total_length += (dx * dx + dy * dy).sqrt();

            // Area (Green's theorem implementation for polygons)
            double_area += p1.x.value() * p2.y.value() - p2.x.value() * p1.y.value();
        }

        // Inflection points
        let inflections = self.find_inflection_points();

        self.stats = EpicycloidStats {
            area: (double_area / 2.0).abs(),
            length: total_length,
            inflection_points: inflections,
        };
    }

    /// Find inflection points (where vector multiply changes sign)
    fn find_inflection_points(&self) -> Vec<Point2D> {
        let mut points = vec![];
        let max_angle = self.rotations as f64 * 2.0 * std::f64::consts::PI;

        // Using small step for more precise finding of inflections
        let precision_step = 0.01;
        let mut t = 0.0;
        let mut prev_cross_product: f32 = 0.0;

        while t < max_angle {
            let (_, _, dx, dy, ddx, ddy) = self.get_derivatives(t);
            // Vector mul of speed and acceleration
            let cross_product = dx * ddy - dy * ddx;

            if t > 0.0 && cross_product.signum() != prev_cross_product.signum() as f64 {
                // Sign changed -> inflection
                points.push(self.get_point(t));
            }

            prev_cross_product = cross_product as f32;
            t += precision_step;
        }
        points
    }

    /// Returns (x, y, dx, dy, ddx, ddy)
    pub fn get_derivatives(&self, t: f64) -> (f64, f64, f64, f64, f64, f64) {
        let fixed_radius = self.fixed_radius.value();
        let r = self.rolling_radius.value();
        let d = self.pen_offset.value();
        let sum = fixed_radius + r;
        let k = sum / r;

        let (sin_t, cos_t) = t.sin_cos();
        let (sin_kt, cos_kt) = (k * t).sin_cos();

        // Position
        let x = sum * cos_t - d * cos_kt;
        let y = sum * sin_t - d * sin_kt;

        // Speed (first derivative)
        let dx = -sum * sin_t + d * k * sin_kt;
        let dy = sum * cos_t - d * k * cos_kt;

        // Acceleration (Second derivative)
        let ddx = -sum * cos_t + d * k.powi(2) * cos_kt;
        let ddy = -sum * sin_t + d * k.powi(2) * sin_kt;

        (x, y, dx, dy, ddx, ddy)
    }

    pub fn curvature_radius_at(&self, t: f64) -> f64 {
        let (_, _, dx, dy, ddx, ddy) = self.get_derivatives(t);

        let numerator = (dx.powi(2) + dy.powi(2)).powf(1.5);
        let denominator = (dx * ddy - dy * ddx).abs();

        if denominator < 1e-6 {
            f64::INFINITY // Straight line
        } else {
            numerator / denominator
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct EpicycloidStats {
    pub area: f64,
    pub length: f64,
    pub inflection_points: Vec<Point2D>,
}
