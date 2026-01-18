use crate::primitives::line2d::Line2D;
use crate::primitives::line3d::Line3D;
use crate::primitives::point2d::Point2D;
use crate::primitives::point3d::Point3D;
use crate::units::Centimeter;
use egui::Stroke;

#[derive(Debug)]
pub struct Surface {
    pub height: Centimeter,
    pub radius_base: Centimeter,
    pub mesh: usize,
    pub style: SurfaceStyle,
}

impl Default for Surface {
    fn default() -> Self {
        Self {
            height: Centimeter(10.0),
            radius_base: Centimeter(5.0),
            mesh: 20,
            style: SurfaceStyle::default(),
        }
    }
}

impl Surface {
    pub fn lines(&self) -> Vec<Line3D<Point3D>> {
        let mut lines = Vec::new();
        let steps = self.mesh;

        for i in 0..=steps {
            let v = i as f64 / steps as f64;

            for j in 0..steps {
                let u1 = j as f64 / steps as f64;
                let u2 = ((j + 1) % steps) as f64 / steps as f64;

                let p1 = self.point_at(u1, v);
                let p2 = self.point_at(u2, v);

                lines.push(Line3D::new(p1, p2, self.style.stroke));

                if i > 0 {
                    let v_prev = (i - 1) as f64 / steps as f64;
                    let p_prev = self.point_at(u1, v_prev);
                    lines.push(Line3D::new(p_prev, p1, self.style.stroke));
                }
            }
        }
        lines
    }

    pub fn point_at(&self, u: f64, v: f64) -> Point3D {
        let theta = u * 2.0 * std::f64::consts::PI;

        let current_radius = self.radius_base.value() * (1.0 - v);

        let y = self.height.value() * v;

        Point3D {
            x: Centimeter(current_radius * theta.cos()),
            y: Centimeter(y),
            z: Centimeter(current_radius * theta.sin()),
        }
    }

    pub fn map_texture(&self, texture_lines: &[Line2D<Point2D>]) -> Vec<Line3D<Point3D>> {
        if texture_lines.is_empty() {
            return vec![];
        }

        let (min_x, max_x, min_y, max_y) = self.calculate_bounds(texture_lines);

        let width = max_x - min_x;
        let height = max_y - min_y;

        let width = if width.abs() < 1e-6 { 1.0 } else { width };
        let height = if height.abs() < 1e-6 { 1.0 } else { height };

        texture_lines
            .iter()
            .map(|line| {
                let start_uv = self.normalize(line.start, min_x, min_y, width, height);
                let end_uv = self.normalize(line.end, min_x, min_y, width, height);

                if (start_uv.0 - end_uv.0).abs() > 0.8 {
                    return Line3D::with_transparent(Point3D::zero(), Point3D::zero());
                }

                let p1_3d = self.point_at(start_uv.0, start_uv.1);
                let p2_3d = self.point_at(end_uv.0, end_uv.1);

                Line3D::new(p1_3d, p2_3d, line.stroke)
            })
            .collect()
    }

    fn normalize(
        &self, p: Point2D, min_x: f64, min_y: f64, w: f64, h: f64,
    ) -> (f64, f64) {
        let u = (p.x.value() - min_x) / w;
        let v = (p.y.value() - min_y) / h;

        (u, v)
    }

    fn calculate_bounds(&self, lines: &[Line2D<Point2D>]) -> (f64, f64, f64, f64) {
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for line in lines {
            for p in [line.start, line.end] {
                let x = p.x.value();
                let y = p.y.value();

                if x < min_x {
                    min_x = x;
                }
                if x > max_x {
                    max_x = x;
                }
                if y < min_y {
                    min_y = y;
                }
                if y > max_y {
                    max_y = y;
                }
            }
        }
        (min_x, max_x, min_y, max_y)
    }
}

#[derive(Debug)]
pub struct SurfaceStyle {
    pub stroke: Stroke,
}

impl Default for SurfaceStyle {
    fn default() -> Self {
        Self {
            stroke: Stroke::new(2.0, egui::Color32::BLACK),
        }
    }
}
