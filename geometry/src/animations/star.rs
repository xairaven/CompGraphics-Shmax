use crate::animations::Direction;
use crate::figures::star3d::Star3D;
use crate::math::angle::Angle;
use crate::transformations::euclidean::rotation3d::Rotation3DOperation;
use std::ops::RangeInclusive;

/// Field: Radius
#[derive(Debug)]
pub struct AnimationStar {
    pub is_enabled: bool,

    direction: Direction,
}

impl Default for AnimationStar {
    fn default() -> Self {
        Self {
            is_enabled: false,
            direction: Direction::Increase,
        }
    }
}

impl AnimationStar {
    pub fn run(
        &mut self, ui: &mut egui::Ui, model: &mut Star3D,
        rotation: &mut Rotation3DOperation,
    ) {
        let field = &mut model.radius.0;

        if self.is_enabled {
            self.step_radius(field);
            self.step_rotation(rotation);
            ui.ctx().request_repaint();
        }
    }

    fn step_radius(&mut self, field: &mut f64) {
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

    fn step_rotation(&mut self, rotation: &mut Rotation3DOperation) {
        rotation.angle_y = Angle::from_degree(rotation.angle_y.degree() + 1.0);
    }

    pub fn toggle(&mut self) {
        self.is_enabled = !self.is_enabled;
    }
}

const STEP_SIZE: f64 = 0.5;
const RANGE: RangeInclusive<f64> = 1.0..=30.0;
