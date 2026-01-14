use crate::figures::epicycloid::Epicycloid;
use std::ops::RangeInclusive;

/// Field: Pen Offset
#[derive(Debug)]
pub struct AnimationEpicycloid {
    pub is_enabled: bool,

    direction: Direction,
}

impl Default for AnimationEpicycloid {
    fn default() -> Self {
        Self {
            is_enabled: false,
            direction: Direction::Increase,
        }
    }
}

impl AnimationEpicycloid {
    pub fn run(&mut self, model: &mut Epicycloid) {
        let field = &mut model.pen_offset.0;

        if self.is_enabled {
            self.step(field);
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
const RANGE: RangeInclusive<f64> = 1.0..=100.0;

#[derive(Debug)]
pub enum Direction {
    Increase,
    Decrease,
}

impl Direction {
    pub fn factor(&self) -> f64 {
        match self {
            Self::Increase => 1.0,
            Self::Decrease => -1.0,
        }
    }

    pub fn toggle(&mut self) {
        *self = match self {
            Self::Increase => Self::Decrease,
            Self::Decrease => Self::Increase,
        };
    }
}
