use crate::fractals::{EquationSystem, FractalIFS};
use egui::Color32;

#[derive(Debug, Clone)]
pub struct FractalZigZag {
    pub iterations: u32,
    pub radius: f64,

    pub systems: Vec<EquationSystem>,
}

impl Default for FractalZigZag {
    fn default() -> Self {
        Self {
            iterations: 10000,
            radius: 1.5,
            systems: vec![
                EquationSystem::new_colored(
                    [
                        -0.632407, -0.614815, -0.545370, 0.659259, 3.840822, 1.282321,
                        0.888128,
                    ],
                    Color32::BLUE,
                ),
                EquationSystem::new_colored(
                    [
                        -0.036111, 0.444444, 0.210185, 0.037037, 2.071081, 8.330552,
                        0.111872,
                    ],
                    Color32::YELLOW,
                ),
            ],
        }
    }
}

impl FractalIFS for FractalZigZag {
    fn iterations(&self) -> u32 {
        self.iterations
    }

    fn radius(&self) -> f64 {
        self.radius
    }

    fn systems(&self) -> &[EquationSystem] {
        &self.systems
    }
}
