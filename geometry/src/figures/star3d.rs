use crate::primitives::line3d::Line3D;
use crate::primitives::point3d::Point3D;
use crate::units::Centimeter;
use egui::{Color32, Stroke};

#[derive(Debug)]
pub struct Star3D {
    pub radius: Centimeter,
    pub thickness: Centimeter,

    pub stroke: Stroke,
}

impl Default for Star3D {
    fn default() -> Self {
        Self {
            radius: Centimeter(5.0),
            thickness: Centimeter(2.5),

            stroke: Stroke::new(5.0, Color32::BLACK),
        }
    }
}

impl Star3D {
    pub fn lines(&self) -> Vec<Line3D<Point3D>> {
        let mut points: Vec<Point3D> = Vec::with_capacity(10);
        let mut upper_points: Vec<Point3D> = Vec::with_capacity(10);

        let mut lines: Vec<Line3D<Point3D>> = Vec::with_capacity(30);

        let radius = self.radius;
        let inner_radius = radius / 2.0;

        let initial_thickness = Centimeter(0.0);
        let thickness = self.thickness;

        for k in 0..=4 {
            let angle =
                k as f64 * 2.0 * std::f64::consts::PI / 5.0 + std::f64::consts::PI / 2.0;
            let offset_angle = angle + 2.0 * std::f64::consts::PI / 10.0;

            let outer = self.create_point(angle, radius, initial_thickness);
            let inner = self.create_point(offset_angle, inner_radius, initial_thickness);

            let upper_outer = self.create_point(angle, radius, thickness);
            let upper_inner = self.create_point(offset_angle, inner_radius, thickness);

            points.push(outer);
            points.push(inner);
            upper_points.push(upper_outer);
            upper_points.push(upper_inner);

            // Add vertical lines directly
            lines.push(Line3D::new(outer, upper_outer, self.stroke));
            lines.push(Line3D::new(inner, upper_inner, self.stroke));
        }

        // Close the loops
        points.push(points[0]);
        upper_points.push(upper_points[0]);

        // Add horizontal lines
        points.windows(2).for_each(|pair| {
            lines.push(Line3D::new(pair[0], pair[1], self.stroke));
        });
        upper_points.windows(2).for_each(|pair| {
            lines.push(Line3D::new(pair[0], pair[1], self.stroke));
        });

        lines
    }

    // https://math.stackexchange.com/questions/3582342/coordinates-of-the-vertices-of-a-five-pointed-star
    fn create_point(
        &self, angle: f64, radius: Centimeter, thickness: Centimeter,
    ) -> Point3D {
        Point3D {
            x: radius * f64::cos(angle),
            y: radius * f64::sin(angle),
            z: thickness,
        }
    }

    pub fn pivot_point(&self) -> Point3D {
        Point3D::new(0.0, 0.0, self.thickness.value() / 2.0)
    }

    pub fn reset(&mut self) {
        *self = Default::default();
    }
}
