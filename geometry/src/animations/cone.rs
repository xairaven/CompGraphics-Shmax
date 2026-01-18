use crate::animations::Direction;
use crate::figures::surface::Surface;
use std::ops::RangeInclusive;

/// Field: Radius of the cone's base.
#[derive(Debug)]
pub struct AnimationCone {
    pub is_enabled: bool,

    direction: Direction,
}

impl Default for AnimationCone {
    fn default() -> Self {
        Self {
            is_enabled: false,
            direction: Direction::Increase,
        }
    }
}

impl AnimationCone {
    pub fn run(&mut self, ui: &mut egui::Ui, model: &mut Surface) {
        let field = &mut model.radius_base.0;

        if self.is_enabled {
            self.step(field);
            ui.ctx().request_repaint();
        }
    }

    fn step(&mut self, field: &mut f64) {
        *field += STEP_SIZE * self.direction.factor();

        // Clamp the value within the defined range
        if *field < *RANGE.start() {
            *field = *RANGE.start();
            self.direction.toggle();
        } else if *field > *RANGE.end() {
            *field = *RANGE.end();
            self.direction.toggle();
        }
    }

    pub fn toggle(&mut self) {
        self.is_enabled = !self.is_enabled;
    }
}

const STEP_SIZE: f64 = 0.5;
const RANGE: RangeInclusive<f64> = 1.0..=20.0;
