use crate::animations::Direction;
use crate::figures::contour::Contour;
use crate::smooth::ferguson::{FergusonPoint, Knot};
use crate::units::Centimeter;

#[derive(Debug)]
pub struct AnimationContour {
    pub is_enabled: bool,
    pub speed: f64,

    t: f64,
    direction: Direction,

    // Start state
    shark_knots: Vec<Knot>,
    // End state
    circle_knots: Vec<Knot>,
}

impl Default for AnimationContour {
    fn default() -> Self {
        let shark = Self::shark_contour();
        let circle = Self::circle_contour(shark.len(), Centimeter(12.0));

        Self {
            is_enabled: false,
            direction: Direction::Increase,
            t: 0.0,
            speed: 0.5,
            shark_knots: shark,
            circle_knots: circle,
        }
    }
}

impl AnimationContour {
    pub fn step(&mut self, ui: &egui::Ui, current_knots: &mut [Knot]) {
        if !self.is_enabled {
            return;
        }

        let dt = ui.input(|i| i.stable_dt).min(0.1) as f64;

        match self.direction {
            Direction::Increase => {
                self.t += self.speed * dt;
                if self.t >= 1.0 {
                    self.t = 1.0;
                    self.direction.toggle();
                }
            },
            Direction::Decrease => {
                self.t -= self.speed * dt;
                if self.t <= 0.0 {
                    self.t = 0.0;
                    self.direction.toggle();
                }
            },
        }

        self.interpolate(current_knots);

        ui.ctx().request_repaint();
    }

    /// Formula: R(u) = R_shark * (1 - t) + R_circle * t
    fn interpolate(&self, target_knots: &mut [Knot]) {
        if target_knots.len() != self.shark_knots.len()
            || target_knots.len() != self.circle_knots.len()
        {
            return;
        }

        let t = self.t;
        let one_minus_t = 1.0 - t;

        for (i, knot) in target_knots.iter_mut().enumerate() {
            let shark = &self.shark_knots[i];
            let circle = &self.circle_knots[i];

            let sx = shark.control.point.coordinates.x.value();
            let sy = shark.control.point.coordinates.y.value();

            let cx = circle.control.point.coordinates.x.value();
            let cy = circle.control.point.coordinates.y.value();

            knot.control.point.coordinates.x = Centimeter(sx * one_minus_t + cx * t);
            knot.control.point.coordinates.y = Centimeter(sy * one_minus_t + cy * t);

            let tx_s = shark.tangent.point.coordinates.x.value();
            let ty_s = shark.tangent.point.coordinates.y.value();

            let tx_c = circle.tangent.point.coordinates.x.value();
            let ty_c = circle.tangent.point.coordinates.y.value();

            knot.tangent.point.coordinates.x = Centimeter(tx_s * one_minus_t + tx_c * t);
            knot.tangent.point.coordinates.y = Centimeter(ty_s * one_minus_t + ty_c * t);
        }
    }

    pub fn play_forward(&mut self) {
        self.direction = Direction::Increase;
        self.is_enabled = true;
    }

    pub fn play_backward(&mut self) {
        self.direction = Direction::Decrease;
        self.is_enabled = true;
    }

    pub fn reset(&mut self, current_knots: &mut [Knot]) {
        self.is_enabled = false;
        self.t = 0.0;
        self.interpolate(current_knots);
    }

    pub fn toggle(&mut self) {
        self.is_enabled = !self.is_enabled;
    }

    pub fn shark_contour() -> Vec<Knot> {
        Contour::default_knots()
    }

    pub fn circle_contour(count: usize, radius: Centimeter) -> Vec<Knot> {
        let mut knots = Vec::with_capacity(count);

        let step_angle = 2.0 * std::f64::consts::PI / count as f64;

        let tangent_magnitude = radius * step_angle;

        let start_angle = std::f64::consts::PI;
        for i in 0..count {
            let theta = start_angle + step_angle * i as f64;

            let cx = radius * theta.cos();
            let cy = radius * theta.sin();

            let vec_x = -theta.sin() * tangent_magnitude.value();
            let vec_y = theta.cos() * tangent_magnitude.value();

            let tx = cx.value() + vec_x;
            let ty = cy.value() + vec_y;

            knots.push(Knot {
                control: FergusonPoint::control(cx, cy),
                tangent: FergusonPoint::tangent(Centimeter(tx), Centimeter(ty)),
            });
        }

        knots
    }
}
