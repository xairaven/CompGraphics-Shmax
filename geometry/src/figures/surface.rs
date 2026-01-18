use crate::primitives::line3d::Line3D;
use crate::primitives::point3d::Point3D;
use crate::units::Centimeter;
use egui::Stroke;

#[derive(Debug)]
pub struct Surface {
    pub height: Centimeter,
    pub radius_x: Centimeter,
    pub radius_y: Centimeter,
    pub mesh: f64,

    pub style: SurfaceStyle,
}

impl Default for Surface {
    fn default() -> Self {
        Self {
            height: Centimeter(5.0),
            radius_x: Centimeter(3.0),
            radius_y: Centimeter(3.0),
            mesh: 20.0,
            style: SurfaceStyle::default(),
        }
    }
}

impl Surface {
    pub fn lines(&self) -> Vec<Line3D<Point3D>> {
        let mut lines = Vec::new();

        let u_steps = self.mesh as usize;
        let v_steps = (self.mesh / 2.0) as usize;

        let apex = Point3D::new(0.0, 0.0, self.height.value());

        // 1. Горизонтальні кільця
        for j in 0..=v_steps {
            let v = j as f64 / v_steps as f64;
            let z = self.height * v;
            let scale = 1.0 - v;

            let rx = self.radius_x.value() * scale;
            let ry = self.radius_y.value() * scale;

            for i in 0..u_steps {
                let theta1 = (std::f64::consts::PI * 2.0 * i as f64) / u_steps as f64;
                let theta2 = (std::f64::consts::PI * 2.0 * ((i + 1) % u_steps) as f64)
                    / u_steps as f64;

                let p1 = Point3D::new(rx * theta1.cos(), ry * theta1.sin(), z.value());
                let p2 = Point3D::new(rx * theta2.cos(), ry * theta2.sin(), z.value());

                lines.push(Line3D::new(p1, p2, self.style.stroke));
            }
        }

        for i in 0..u_steps {
            let theta = (std::f64::consts::PI * 2.0 * i as f64) / u_steps as f64;

            let base_p = Point3D::new(
                self.radius_x.value() * theta.cos(),
                self.radius_y.value() * theta.sin(),
                0.0,
            );

            lines.push(Line3D::new(base_p, apex, self.style.stroke));
        }

        lines
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
