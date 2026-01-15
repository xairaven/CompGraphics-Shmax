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

    pub is_inflection_points_enabled: bool,
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

            is_inflection_points_enabled: false,
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

    fn current_t(&self, epicycloid: &Epicycloid) -> f64 {
        self.current_array_index as f64 * epicycloid.step
    }

    pub fn current_curvature_radius(&self, epicycloid: &Epicycloid) -> f64 {
        epicycloid.curvature_radius_at(self.current_t(epicycloid))
    }

    // Helper for creating lines of fixed length
    fn create_vector_line(&self, dx: f64, dy: f64) -> Option<Line2D<Point2D>> {
        let len = (dx * dx + dy * dy).sqrt();
        if len < 1e-6 {
            return None;
        }

        let scale = self.lines_size.value();
        let u_dx = (dx / len) * scale;
        let u_dy = (dy / len) * scale;

        let p = self.point();
        Some(Line2D::with_transparent(
            Point2D::new(p.x.value() - u_dx, p.y.value() - u_dy),
            Point2D::new(p.x.value() + u_dx, p.y.value() + u_dy),
        ))
    }

    pub fn tangent(&self, epicycloid: &Epicycloid) -> Option<Line2D<Point2D>> {
        if !self.is_visible || !self.is_tangent_enabled {
            return None;
        }

        let t = self.current_t(epicycloid);
        let (_, _, dx, dy, _, _) = epicycloid.get_derivatives(t);

        self.create_vector_line(dx, dy).map(|mut line| {
            line.stroke = Stroke::new(1.5, Color32::BLUE);
            line
        })
    }

    pub fn normal(&self, epicycloid: &Epicycloid) -> Option<Line2D<Point2D>> {
        if !self.is_visible || !self.is_normal_enabled {
            return None;
        }

        let t = self.current_t(epicycloid);
        let (_, _, dx, dy, _, _) = epicycloid.get_derivatives(t);

        // Normal: perpendicular (-dy, dx)
        self.create_vector_line(-dy, dx).map(|mut line| {
            line.stroke = Stroke::new(1.5, Color32::ORANGE);
            line
        })
    }

    pub fn hide(&mut self) {
        self.is_normal_enabled = false;
        self.is_tangent_enabled = false;
        self.is_inflection_points_enabled = false;
        self.is_visible = false;
        self.is_enabled = false;
    }
}
