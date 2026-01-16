use crate::smooth::ferguson::{FergusonCurve, FergusonPoint, Knot};
use crate::units::Centimeter;
use crate::viewport::Viewport;
use egui::{Response, Shape};

#[derive(Debug)]
pub struct Contour {
    pub curve: FergusonCurve,
    pub is_tooltips_mode_enabled: bool,
    pub is_skeleton_mode_enabled: bool,
}

impl Default for Contour {
    fn default() -> Self {
        Self {
            curve: FergusonCurve {
                knots: Self::default_knots(),
                is_closed: true,
                step: 0.01,
                style: Default::default(),
            },
            is_tooltips_mode_enabled: false,
            is_skeleton_mode_enabled: false,
        }
    }
}

impl Contour {
    pub fn lines(&self, viewport: &Viewport) -> Vec<Shape> {
        self.curve.contour(viewport)
    }

    pub fn skeleton(&self, viewport: &Viewport) -> Vec<Shape> {
        if !self.is_skeleton_mode_enabled {
            return vec![];
        }
        self.curve.skeleton(viewport)
    }

    pub fn update_curve(
        &mut self, ui: &egui::Ui, response: &Response, viewport: &Viewport,
    ) {
        self.curve
            .knots
            .iter_mut()
            .enumerate()
            .for_each(|(index, knot)| {
                let index = index + 1;

                knot.control.point.update_on_pan(ui, response, viewport);
                knot.tangent.point.update_on_pan(ui, response, viewport);

                if self.is_tooltips_mode_enabled {
                    knot.control
                        .point
                        .show_tooltip(index, ui, response, viewport);
                    knot.tangent
                        .point
                        .show_tooltip(index, ui, response, viewport);
                }
            });
    }

    pub fn default_knots() -> Vec<Knot> {
        vec![
            Knot {
                control: FergusonPoint::control(Centimeter(1.0), Centimeter(5.0)),
                tangent: FergusonPoint::tangent(Centimeter(2.0), Centimeter(7.0)),
            },
            Knot {
                control: FergusonPoint::control(Centimeter(3.0), Centimeter(5.0)),
                tangent: FergusonPoint::tangent(Centimeter(4.0), Centimeter(7.0)),
            },
            Knot {
                control: FergusonPoint::control(Centimeter(5.0), Centimeter(5.0)),
                tangent: FergusonPoint::tangent(Centimeter(6.0), Centimeter(7.0)),
            },
            Knot {
                control: FergusonPoint::control(Centimeter(7.0), Centimeter(5.0)),
                tangent: FergusonPoint::tangent(Centimeter(8.0), Centimeter(7.0)),
            },
            Knot {
                control: FergusonPoint::control(Centimeter(9.0), Centimeter(5.0)),
                tangent: FergusonPoint::tangent(Centimeter(10.0), Centimeter(7.0)),
            },
            Knot {
                control: FergusonPoint::control(Centimeter(11.0), Centimeter(5.0)),
                tangent: FergusonPoint::tangent(Centimeter(12.0), Centimeter(7.0)),
            },
        ]
    }
}
