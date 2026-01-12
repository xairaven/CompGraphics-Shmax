use crate::math::angle::Angle;
use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::units::Centimeter;
use egui::Stroke;

#[derive(Debug, Clone, Copy)]
pub enum ShapeType {
    // A complete closed circle
    Full,
    // Part of a circle (arc).
    // start - angle where the arc starts.
    // sweep - length of the arc in radians (positive).
    Arc { start: Angle, sweep: Angle },
}

#[derive(Debug, Clone, Copy)]
pub struct CircularShape {
    pub center: Point2D,
    pub radius: Centimeter,
    pub shape_type: ShapeType,
    pub stroke: Stroke,
}

impl CircularShape {
    /// Constructs an arc from two points and a radius.
    /// The arc goes from p1 to p2 COUNTERCLOCKWISE.
    /// The center is calculated automatically.
    pub fn from_points_and_radius(
        p1: Point2D, p2: Point2D, requested_radius: Centimeter, stroke: Stroke,
    ) -> Self {
        let (x1, y1) = (p1.x.value(), p1.y.value());
        let (x2, y2) = (p2.x.value(), p2.y.value());

        let (dx, dy) = (x2 - x1, y2 - y1);

        let dist_sq = dx * dx + dy * dy;
        let dist = dist_sq.sqrt();

        // Verify radius validity
        // The distance between points cannot be greater than the diameter (2R).
        // If the given radius is too small, we increase it to the minimum possible (half the distance).
        let mut radius_val = requested_radius.value();
        if dist > radius_val * 2.0 {
            radius_val = dist / 2.0;
        }

        // Find midpoint of the chord
        // We seek a point at distance R from p1 and p2. There are two such points.
        // We choose the one that provides counterclockwise movement from p1 to p2.
        let mid_x = (x1 + x2) / 2.0;
        let mid_y = (y1 + y2) / 2.0;

        // Distance from midpoint of chord to center (by Pythagorean theorem)
        let half_chord = dist / 2.0;
        let dist_to_center = (radius_val.powi(2) - half_chord.powi(2)).max(0.0).sqrt();

        // Offset perpendicular to the chord: (-dy, dx)
        // We divide by dist to get a unit vector, then multiply by dist_to_center
        let offset_x = -dy * (dist_to_center / dist);
        let offset_y = dx * (dist_to_center / dist);

        let center_x = mid_x + offset_x;
        let center_y = mid_y + offset_y;

        let center = Point2D {
            x: Centimeter(center_x),
            y: Centimeter(center_y),
        };

        // Calculating angles
        // atan2 returns the angle of the vector from center to the point
        let start_rad = (y1 - center_y).atan2(x1 - center_x);
        let end_rad = (y2 - center_y).atan2(x2 - center_x);

        let mut sweep_rad = end_rad - start_rad;
        if sweep_rad <= 0.0 {
            sweep_rad += std::f64::consts::TAU;
        }

        Self {
            center,
            radius: Centimeter(radius_val),
            shape_type: ShapeType::Arc {
                start: Angle::from_radian(start_rad),
                sweep: Angle::from_radian(sweep_rad),
            },
            stroke,
        }
    }

    pub fn lines(&self, resolution: usize) -> Vec<Line2D<Point2D>> {
        let points = self.polyline(resolution);

        points
            .windows(2)
            .map(|pair| Line2D {
                start: pair[0],
                end: pair[1],
                stroke: self.stroke,
            })
            .collect::<Vec<Line2D<Point2D>>>()
    }

    pub fn resize_by_radius(p1: &mut Point2D, p2: &mut Point2D, radius: &Centimeter) {
        let mid_x = (p1.x.value() + p2.x.value()) / 2.0;
        let mid_y = (p1.y.value() + p2.y.value()) / 2.0;

        let dx = p2.x.value() - p1.x.value();
        let dy = p2.y.value() - p1.y.value();
        let current_len = (dx.powi(2) + dy.powi(2)).sqrt();

        if current_len < 1e-6 {
            return;
        }

        let ux = dx / current_len;
        let uy = dy / current_len;

        let r_val = radius.value();

        *p1 = Point2D {
            x: Centimeter(mid_x - ux * r_val),
            y: Centimeter(mid_y - uy * r_val),
        };

        *p2 = Point2D {
            x: Centimeter(mid_x + ux * r_val),
            y: Centimeter(mid_y + uy * r_val),
        };
    }

    pub fn radius_by_points(p1: &Point2D, p2: &Point2D) -> Centimeter {
        let dx = p2.x.value() - p1.x.value();
        let dy = p2.y.value() - p1.y.value();
        let dist = (dx.powi(2) + dy.powi(2)).sqrt();

        Centimeter(dist / 2.0)
    }

    fn start_end_angles(&self) -> (f64, f64) {
        match self.shape_type {
            ShapeType::Full => (0.0, std::f64::consts::TAU),
            ShapeType::Arc { start, sweep } => {
                let start_rad = start.radian();
                let end_rad = start_rad + sweep.radian();
                (start_rad, end_rad)
            },
        }
    }

    // Resolution is the number of segments for a full circle.
    fn polyline(&self, resolution: usize) -> Vec<Point2D> {
        let (start_angle, end_angle) = self.start_end_angles();

        // Calculating sweep again from angles for consistency
        let sweep = end_angle - start_angle;

        // Correct the number of steps according to the arc length.
        // If the arc is short, there will be fewer points. If full - equal to resolution.
        let fraction = sweep.abs() / std::f64::consts::TAU;
        let step_count = ((resolution as f64 * fraction).ceil() as usize).max(2);

        let step_size = sweep / step_count as f64;

        let mut points = Vec::with_capacity(step_count + 1);
        let r = self.radius.value();
        let cx = self.center.x.value();
        let cy = self.center.y.value();

        for i in 0..=step_count {
            let theta = start_angle + (step_size * i as f64);

            points.push(Point2D {
                x: Centimeter(cx + r * theta.cos()),
                y: Centimeter(cy + r * theta.sin()),
            });
        }

        points
    }
}
