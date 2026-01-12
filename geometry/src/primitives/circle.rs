use crate::math::angle::Angle;
use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::units::Centimeter;
use egui::Stroke;

#[derive(Debug, Clone, Copy)]
pub enum ShapeType {
    // A complete closed circle
    Full,
    // A semicircle defined by a rotation angle (in radians)
    // The angle usually points perpendicular to the cut diameter
    Semi { angle: Angle },
}

#[derive(Debug, Clone, Copy)]
pub struct CircularShape {
    pub center: Point2D,
    pub radius: Centimeter,
    pub shape_type: ShapeType,
    pub stroke: Stroke,
}

impl CircularShape {
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

    pub fn endpoints(&self) -> (Point2D, Point2D) {
        let (start_angle, end_angle) = self.start_end_angles();

        // Calculating coordinates for these two angles
        let r = self.radius.value();
        let cx = self.center.x.value();
        let cy = self.center.y.value();

        let start_point = Point2D {
            x: Centimeter(cx + r * start_angle.cos()),
            y: Centimeter(cy + r * start_angle.sin()),
        };

        let end_point = Point2D {
            x: Centimeter(cx + r * end_angle.cos()),
            y: Centimeter(cy + r * end_angle.sin()),
        };

        (start_point, end_point)
    }

    fn start_end_angles(&self) -> (f64, f64) {
        let (start_angle, end_angle) = match self.shape_type {
            ShapeType::Full => (0.0, std::f64::consts::TAU), // TAU = 2 * PI
            ShapeType::Semi { angle } => {
                let radians = angle.radian();
                // Start at (angle - 90°) to (angle + 90°)
                (
                    radians - std::f64::consts::FRAC_PI_2,
                    radians + std::f64::consts::FRAC_PI_2,
                )
            },
        };

        (start_angle, end_angle)
    }

    // Resolution is the number of segments for a full circle.
    fn polyline(&self, resolution: usize) -> Vec<Point2D> {
        let (start_angle, end_angle) = self.start_end_angles();

        // Calculating factual number of steps for the arc
        // If it's a semicircle, steps will be half of resolution
        let sweep = end_angle - start_angle;
        let step_count =
            ((sweep / std::f64::consts::TAU) * resolution as f64).ceil() as usize;
        let step_size = sweep / step_count as f64;

        let mut points = Vec::with_capacity(step_count + 1);

        // Generating arc points
        for i in 0..=step_count {
            let theta = start_angle + (step_size * i as f64);

            // x = cx + r * cos(theta)
            // y = cy + r * sin(theta)
            points.push(Point2D {
                x: Centimeter(self.center.x.value() + self.radius.value() * theta.cos()),
                y: Centimeter(self.center.y.value() + self.radius.value() * theta.sin()),
            });
        }

        points
    }
}
