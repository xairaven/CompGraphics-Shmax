use crate::primitives::point2d::Point2D;
use crate::shapes::dot::DotMetadata;
use crate::units::Centimeter;
use crate::viewport::Viewport;
use egui::{Color32, Shape, Stroke};
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;

pub mod zigzag;

pub trait FractalIFS {
    fn iterations(&self) -> u32;
    fn radius(&self) -> f64;
    fn systems(&self) -> &[EquationSystem];

    fn points(&self) -> Vec<(Point2D, Color32)> {
        let mut points: Vec<(Point2D, Color32)> = Vec::new();

        let start = Point2D::zero();
        points.push((start, Color32::TRANSPARENT));

        let probabilities: Vec<f64> = self
            .systems()
            .iter()
            .map(|equation| equation.probability())
            .collect();
        let mut rng = rand::rng();

        let dist =
            WeightedIndex::new(&probabilities).expect("Failed to create WeightedIndex");

        for current_index in 0..self.iterations() {
            let equation = &self.systems()[dist.sample(&mut rng)];
            let new_point = equation.next_point(&points[current_index as usize].0);

            points.push(new_point);
        }

        points
    }

    fn shapes(&self, viewport: &Viewport) -> Vec<Shape> {
        let points = self.points();

        points
            .iter()
            .map(|(point, color)| {
                point.to_pixels(viewport).to_dot(&DotMetadata {
                    radius: self.radius() as f32,
                    fill: *color,
                    stroke: Stroke::new(0.0, Color32::TRANSPARENT),
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EquationSystem {
    a: f64,
    b: f64,
    d: f64,
    e: f64,
    c: f64,
    f: f64,
    p: f64,

    color: Color32,
}

impl EquationSystem {
    pub fn new(coefficients: [f64; 7]) -> Self {
        Self {
            a: coefficients[0],
            b: coefficients[1],
            d: coefficients[2],
            e: coefficients[3],
            c: coefficients[4],
            f: coefficients[5],
            p: coefficients[6],

            color: Color32::BLACK,
        }
    }

    pub fn new_colored(coefficients: [f64; 7], color: Color32) -> Self {
        let mut system = Self::new(coefficients);
        system.color = color;

        system
    }

    pub fn probability(&self) -> f64 {
        self.p
    }

    pub fn next_point(&self, point: &Point2D) -> (Point2D, Color32) {
        let x = self.a * point.x.value() + self.b * point.y.value() + self.c;
        let y = self.d * point.x.value() + self.e * point.y.value() + self.f;

        let point = Point2D {
            x: Centimeter(x),
            y: Centimeter(y),
        };

        (point, self.color)
    }
}
