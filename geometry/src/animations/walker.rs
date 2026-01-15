use crate::animations::Direction;
use crate::figures::epicycloid::Epicycloid;
use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::shapes::dot::DotMetadata;
use crate::units::Centimeter;
use crate::viewport::Viewport;
use egui::{Color32, Shape, Stroke};

pub const STEP_RANGE: std::ops::RangeInclusive<u32> = 1..=10;

#[derive(Debug)]
pub struct CurveWalker {
    pub is_enabled: bool,
    pub is_visible: bool,
    pub step: u32,

    pub is_normal_enabled: bool,
    pub is_tangent_enabled: bool,
    pub lines_size: Centimeter,

    current_point: Point2D,
    current_array_index: usize,
    current_array_length: usize,

    direction: Direction,
}

impl Default for CurveWalker {
    fn default() -> Self {
        Self {
            is_enabled: false,
            is_visible: false,
            step: 1,

            current_point: Point2D::new(0.0, 0.0),
            current_array_index: 0,
            current_array_length: 0,

            direction: Direction::Increase,

            is_normal_enabled: false,
            is_tangent_enabled: false,
            lines_size: Centimeter(30.0),
        }
    }
}

impl CurveWalker {
    pub fn step(&mut self, ui: &egui::Ui, lines: &mut [Line2D<Point2D>]) {
        if !self.is_enabled || !self.is_visible {
            return;
        }

        if self.current_array_length != lines.len() {
            if lines.is_empty() {
                return;
            }

            self.current_point = lines[0].start;
            self.current_array_index = 0;
            self.current_array_length = lines.len();
            return;
        }

        let delta = (self.direction.factor() as isize) * (self.step as isize);
        let index = (self.current_array_index as isize + delta)
            .rem_euclid(self.current_array_length as isize) as usize;

        self.current_array_index = index;
        self.current_point = lines[index].start;

        ui.ctx().request_repaint();
    }

    pub fn set_increasing(&mut self) {
        if self.is_visible {
            self.is_enabled = !self.is_enabled;
            self.direction = Direction::Increase;
        }
    }

    pub fn set_decreasing(&mut self) {
        if self.is_visible {
            self.is_enabled = !self.is_enabled;
            self.direction = Direction::Decrease;
        }
    }

    pub fn show_toggle(&mut self) {
        self.is_visible = !self.is_visible;
        if !self.is_visible {
            self.is_enabled = false;
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn point(&self) -> Point2D {
        self.current_point
    }

    pub fn dot(&self, viewport: &Viewport) -> Option<Shape> {
        if self.is_visible {
            return Some(self.current_point.to_pixels(viewport).to_dot(&DotMetadata {
                radius: 5.0,
                fill: Color32::GREEN,
                stroke: Stroke::new(0.5, Color32::BLACK),
            }));
        }

        None
    }

    fn unit_derivative(&self, epicycloid: &Epicycloid) -> Option<(f64, f64)> {
        let t = self.current_array_index as f64 * epicycloid.step;

        let sum_r = epicycloid.fixed_radius + epicycloid.rolling_radius;
        let k = sum_r / epicycloid.rolling_radius.value();

        // Calculate derivatives (velocity vector)
        // x' = -S * sin(t) + d * k * sin(k*t)
        // y' =  S * cos(t) - d * k * cos(k*t)
        let (sin_t, cos_t) = t.sin_cos();
        let (sin_kt, cos_kt) = (k * t).sin_cos();

        let dx =
            -sum_r.value() * sin_t + epicycloid.pen_offset.value() * k.value() * sin_kt;
        let dy =
            sum_r.value() * cos_t - epicycloid.pen_offset.value() * k.value() * cos_kt;

        // Vector normalization (for fixed length lines, e.g., 5 cm)
        let len = (dx * dx + dy * dy).sqrt();

        // Handle zero-length vector
        if len < 1e-6 {
            return None;
        }

        // Length of the tangent and normal lines
        let unit_dx = (dx / len) * self.lines_size.value();
        let unit_dy = (dy / len) * self.lines_size.value();

        Some((unit_dx, unit_dy))
    }

    pub fn tangent(&self, epicycloid: &Epicycloid) -> Option<Line2D<Point2D>> {
        if !self.is_visible || !self.is_tangent_enabled {
            return None;
        }

        let (unit_dx, unit_dy) = self.unit_derivative(epicycloid)?;

        // Forming lines
        // Tangent: from (P - v) to (P + v)
        let point = self.point();
        let tangent = Line2D::new(
            Point2D::new(point.x.value() - unit_dx, point.y.value() - unit_dy),
            Point2D::new(point.x.value() + unit_dx, point.y.value() + unit_dy),
            Stroke::new(1.5, Color32::BLUE),
        );

        Some(tangent)
    }

    pub fn normal(&self, epicycloid: &Epicycloid) -> Option<Line2D<Point2D>> {
        if !self.is_visible || !self.is_normal_enabled {
            return None;
        }

        let (unit_dx, unit_dy) = self.unit_derivative(epicycloid)?;

        // Normal: Perpendicular (-dy, dx)
        // Rotate the vector by 90 degrees
        let norm_dx = -unit_dy;
        let norm_dy = unit_dx;

        let point = self.point();
        let normal = Line2D::new(
            Point2D::new(point.x.value() - norm_dx, point.y.value() - norm_dy),
            Point2D::new(point.x.value() + norm_dx, point.y.value() + norm_dy),
            Stroke::new(1.5, Color32::RED),
        );

        Some(normal)
    }

    pub fn hide(&mut self) {
        self.is_normal_enabled = false;
        self.is_tangent_enabled = false;
        self.is_visible = false;
        self.is_enabled = false;
    }
}
