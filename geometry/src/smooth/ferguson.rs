use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::{MoveablePoint, Point2D};
use crate::shapes::dot::DotMetadata;
use crate::shapes::square::SquareMetadata;
use crate::units::Centimeter;
use crate::viewport::Viewport;
use egui::{Color32, Shape, Stroke};

#[derive(Debug)]
pub struct FergusonCurve {
    pub knots: Vec<Knot>,

    pub is_closed: bool,
    pub step: f64,
    pub style: CurveStyle,
}

impl FergusonCurve {
    pub fn contour(&self, viewport: &Viewport) -> Vec<Shape> {
        if self.knots.is_empty() {
            return vec![];
        }

        let mut buffer: Vec<Line2D<Point2D>> = vec![];

        for knot_pair in self.knots.windows(2) {
            let start = &knot_pair[0];
            let end = &knot_pair[1];

            self.segment(start, end, &mut buffer);
        }

        if self.is_closed && self.knots.len() > 2 {
            let first = &self.knots[0];
            let last = &self.knots[self.knots.len() - 1];
            self.segment(last, first, &mut buffer);
        }

        buffer
            .iter()
            .map(|line| line.to_pixels(viewport).to_shape())
            .collect()
    }

    fn segment(&self, start: &Knot, end: &Knot, buffer: &mut Vec<Line2D<Point2D>>) {
        let start_point = start.control.point.coordinates;
        let end_point = end.control.point.coordinates;

        let vector_t_start = Point2D {
            x: start.tangent.point.coordinates.x - start_point.x,
            y: start.tangent.point.coordinates.y - start_point.y,
        };
        let vector_t_end = Point2D {
            x: end.tangent.point.coordinates.x - end_point.x,
            y: end.tangent.point.coordinates.y - end_point.y,
        };

        let mut t = 0.0;
        let mut previous_point = start_point;

        while t < 1.0 {
            t += self.step;

            if t > 1.0 {
                t = 1.0;
            }

            // Basis functions (formulas)
            let t2 = t * t;
            let t3 = t2 * t;
            let h1 = 2.0 * t3 - 3.0 * t2 + 1.0;
            let h2 = -2.0 * t3 + 3.0 * t2;
            let h3 = t3 - 2.0 * t2 + t;
            let h4 = t3 - t2;

            let new_point = Point2D {
                x: Centimeter(
                    h1 * start_point.x.value()
                        + h2 * end_point.x.value()
                        + h3 * vector_t_start.x.value()
                        + h4 * vector_t_end.x.value(),
                ),
                y: Centimeter(
                    h1 * start_point.y.value()
                        + h2 * end_point.y.value()
                        + h3 * vector_t_start.y.value()
                        + h4 * vector_t_end.y.value(),
                ),
            };

            let line = Line2D::new(previous_point, new_point, self.style.contour);
            buffer.push(line);

            previous_point = new_point;
        }
    }

    pub fn skeleton(&self, viewport: &Viewport) -> Vec<Shape> {
        let mut shapes = vec![];

        for knot_pair in self.knots.windows(2) {
            let start = &knot_pair[0];
            let end = &knot_pair[1];

            let line = Line2D::new(
                start.control.point.coordinates,
                end.control.point.coordinates,
                self.style.skeleton,
            )
            .to_pixels(viewport)
            .to_shape();
            shapes.push(line);
        }

        for knot in &self.knots {
            let control_point = knot.control.point.coordinates;
            let tangent_point = knot.tangent.point.coordinates;

            let control_to_tangent =
                Line2D::new(control_point, tangent_point, self.style.control_to_tangent)
                    .to_pixels(viewport)
                    .to_shape();
            shapes.push(control_to_tangent);

            shapes.push(
                control_point
                    .to_pixels(viewport)
                    .to_dot(&self.style.control),
            );
            shapes.push(
                tangent_point
                    .to_pixels(viewport)
                    .to_square(&self.style.tangent),
            );
        }

        shapes
    }
}

#[derive(Debug, Clone)]
pub struct Knot {
    pub control: FergusonPoint,
    pub tangent: FergusonPoint,
}

#[derive(Debug, Clone)]
pub struct FergusonPoint {
    pub point: MoveablePoint,
    pub kind: FergusonPointKind,
}

impl FergusonPoint {
    pub fn control(x: Centimeter, y: Centimeter) -> Self {
        let point = Point2D { x, y };
        Self {
            point: MoveablePoint::new(point),
            kind: FergusonPointKind::Control,
        }
    }

    pub fn tangent(x: Centimeter, y: Centimeter) -> Self {
        let point = Point2D { x, y };
        Self {
            point: MoveablePoint::new(point),
            kind: FergusonPointKind::Tangent,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FergusonPointKind {
    Control,
    Tangent,
}

#[derive(Debug)]
pub struct CurveStyle {
    pub contour: Stroke,
    pub skeleton: Stroke,
    pub control_to_tangent: Stroke,
    pub control: DotMetadata,
    pub tangent: SquareMetadata,
}

impl Default for CurveStyle {
    fn default() -> Self {
        Self {
            contour: Stroke::new(2.0, Color32::BLACK),
            skeleton: Stroke::new(1.6, Color32::DARK_GRAY),
            control_to_tangent: Stroke::new(1.0, Color32::DARK_GREEN),
            control: DotMetadata {
                radius: 5.0,
                fill: Color32::RED,
                stroke: Stroke::new(0.5, Color32::BLACK),
            },
            tangent: SquareMetadata {
                radius: 5.0,
                corner_radius: 0.0,
                fill: Color32::GREEN,
                stroke: Stroke::new(0.5, Color32::BLACK),
            },
        }
    }
}
