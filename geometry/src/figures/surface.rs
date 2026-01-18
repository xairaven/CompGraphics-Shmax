use crate::figures::texture::Texture;
use crate::primitives::line2d::Line2D;
use crate::primitives::line3d::Line3D;
use crate::primitives::point2d::Point2D;
use crate::primitives::point3d::Point3D;
use crate::units::{Centimeter, Percent};
use egui::Stroke;

#[derive(Debug)]
pub struct Surface {
    pub height: Centimeter,
    pub radius_base: Centimeter,
    pub mesh: usize,
    pub is_texture_enabled: bool,
    pub texture_scale_width: Percent,
    pub texture_scale_height: Percent,
    pub texture_offset_angle: Percent,
    pub texture_offset_height: Percent,
    pub style: SurfaceStyle,
}

impl Default for Surface {
    fn default() -> Self {
        Self {
            height: Centimeter(10.0),
            radius_base: Centimeter(5.0),
            mesh: 20,
            texture_scale_width: Percent(0.3),
            texture_scale_height: Percent(0.5),
            texture_offset_angle: Percent(0.0),
            texture_offset_height: Percent(0.5),
            is_texture_enabled: false,
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

    pub fn handle_texture(&self, texture: &Texture) -> Vec<Line3D<Point3D>> {
        if !self.is_texture_enabled {
            return Vec::new();
        }

        let lines = texture.lines();
        self.map_texture(&lines)
    }

    pub fn map_texture(&self, texture_lines: &[Line2D<Point2D>]) -> Vec<Line3D<Point3D>> {
        if texture_lines.is_empty() {
            return vec![];
        }

        let (min_x, max_x, min_y, max_y) = self.calculate_bounds(texture_lines);
        let width = (max_x - min_x).abs().max(1e-6);
        let height = (max_y - min_y).abs().max(1e-6);

        texture_lines
            .iter()
            .map(|line| {
                let start_uv_raw =
                    self.normalize(line.start, min_x, min_y, width, height);
                let end_uv_raw = self.normalize(line.end, min_x, min_y, width, height);

                let start_uv_mapped = self.apply_uv_transform(start_uv_raw);
                let end_uv_mapped = self.apply_uv_transform(end_uv_raw);

                if (start_uv_mapped.0 - end_uv_mapped.0).abs() > 0.8 {
                    return Line3D::with_transparent(Point3D::zero(), Point3D::zero());
                }

                let p1_3d = self.point_at(start_uv_mapped.0, start_uv_mapped.1);
                let p2_3d = self.point_at(end_uv_mapped.0, end_uv_mapped.1);

                Line3D::new(p1_3d, p2_3d, line.stroke)
            })
            .collect()
    }

    fn apply_uv_transform(&self, uv: (f64, f64)) -> (f64, f64) {
        let (u_raw, v_raw) = uv;
        let (scale_u, scale_v) = (self.texture_scale_width, self.texture_scale_height);
        let (offset_u, offset_v) =
            (self.texture_offset_angle, self.texture_offset_height);

        let u_final = offset_u.value() + (u_raw - 0.5) * scale_u.value();
        let v_final = offset_v.value() + (v_raw - 0.5) * scale_v.value();

        (u_final, v_final)
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

    pub fn pivot_point(&self) -> Point3D {
        Point3D {
            x: Centimeter(0.0),
            y: Centimeter(self.height.value() / 2.0),
            z: Centimeter(0.0),
        }
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
