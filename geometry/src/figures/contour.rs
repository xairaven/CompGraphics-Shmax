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

                knot.control
                    .update_on_change_smoothness(ui, response, viewport);
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
            // 1
            Knot {
                control: FergusonPoint::control(Centimeter(1.02011), Centimeter(-8.9539)),
                tangent: FergusonPoint::tangent(
                    Centimeter(-2.9082),
                    Centimeter(-8.45039),
                ),
            },
            // 2
            Knot {
                control: FergusonPoint::control(
                    Centimeter(-2.57812),
                    Centimeter(-4.25937),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(-2.0705),
                    Centimeter(-3.69433),
                ),
            },
            // 3
            Knot {
                control: FergusonPoint::control(
                    Centimeter(-11.21499940368627),
                    Centimeter(-2.9941134058440477),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(-14.831832156238951),
                    Centimeter(-0.80444295212432),
                ),
            },
            // 4
            Knot {
                control: FergusonPoint::control(
                    Centimeter(-14.228291375486565),
                    Centimeter(-0.12198798368550326),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(-12.328674990838119),
                    Centimeter(2.2451162890802476),
                ),
            },
            // 5
            Knot {
                control: FergusonPoint::control(
                    Centimeter(-9.97698281234206),
                    Centimeter(2.8346457797572246),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(-5.8401819112369475),
                    Centimeter(3.990045716729621),
                ),
            },
            // 6
            Knot {
                control: FergusonPoint::control(
                    Centimeter(-6.3958775171549185),
                    Centimeter(3.8785035308882505),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(-4.434602310064234),
                    Centimeter(3.317793015151888),
                ),
            },
            // 7
            Knot {
                control: FergusonPoint::control(
                    Centimeter(-7.112860463766285),
                    Centimeter(10.035839898050087),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(-2.7874835520080725),
                    Centimeter(8.572557808866835),
                ),
            },
            // 8
            Knot {
                control: FergusonPoint::control(
                    Centimeter(-2.676282745918255),
                    Centimeter(4.789741734933403),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(-2.602211030698747),
                    Centimeter(3.0901043114669156),
                ),
            },
            // 9
            Knot {
                control: FergusonPoint::control(
                    Centimeter(3.556262044747978),
                    Centimeter(10.101223588434513),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(6.625739419182469),
                    Centimeter(11.14758134548919),
                ),
            },
            // 10
            Knot {
                control: FergusonPoint::control(
                    Centimeter(5.390144612009004),
                    Centimeter(9.154267148166994),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(0.4290323743080803),
                    Centimeter(5.386406090061552),
                ),
            },
            // 11
            Knot {
                control: FergusonPoint::control(
                    Centimeter(1.8854850913662917),
                    Centimeter(2.5035405438065554),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(1.219927374118533),
                    Centimeter(-1.0327243857573374),
                ),
            },
            // 12
            Knot {
                control: FergusonPoint::control(
                    Centimeter(3.8334775576725946),
                    Centimeter(1.6024243115932943),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(4.076671268023657),
                    Centimeter(0.4524572075772364),
                ),
            },
            // 13
            Knot {
                control: FergusonPoint::control(
                    Centimeter(7.571347256075643),
                    Centimeter(6.0936956959692425),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(10.485816861775529),
                    Centimeter(6.671258930993505),
                ),
            },
            // 14
            Knot {
                control: FergusonPoint::control(
                    Centimeter(8.716791539342216),
                    Centimeter(5.042138394495609),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(6.2405913030773075),
                    Centimeter(3.6880759019755005),
                ),
            },
            // 15
            Knot {
                control: FergusonPoint::control(
                    Centimeter(7.572503423298151),
                    Centimeter(1.082574552103942),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(8.65645263032965),
                    Centimeter(-1.3779622250390882),
                ),
            },
            // 16
            Knot {
                control: FergusonPoint::control(
                    Centimeter(9.465559525994093),
                    Centimeter(0.6877470235008322),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(10.44226160081697),
                    Centimeter(0.7300056323602206),
                ),
            },
            // 17
            Knot {
                control: FergusonPoint::control(
                    Centimeter(14.759256269148503),
                    Centimeter(8.500284622434885),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(15.065184833168402),
                    Centimeter(12.074115639660603),
                ),
            },
            // 18
            Knot {
                control: FergusonPoint::control(
                    Centimeter(15.483009096548447),
                    Centimeter(8.653382532751975),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(16.275236938110325),
                    Centimeter(3.6458556649725584),
                ),
            },
            // 19
            Knot {
                control: FergusonPoint::control(
                    Centimeter(13.52670352093569),
                    Centimeter(0.9832184338498043),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(7.658246820047673),
                    Centimeter(-5.471083938872315),
                ),
            },
            // 20
            Knot {
                control: FergusonPoint::control(
                    Centimeter(11.624266957917081),
                    Centimeter(-3.525219347892877),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(10.145560495887011),
                    Centimeter(-7.826989464818565),
                ),
            },
            // 21
            Knot {
                control: FergusonPoint::control(
                    Centimeter(13.241039622993377),
                    Centimeter(-6.78097926523506),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(17.0872924578732),
                    Centimeter(-11.06824656573134),
                ),
            },
            // 22
            Knot {
                control: FergusonPoint::control(
                    Centimeter(15.635647915902004),
                    Centimeter(-7.604476379990596),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(16.253620505580958),
                    Centimeter(-9.944169360259641),
                ),
            },
            // 23
            Knot {
                control: FergusonPoint::control(
                    Centimeter(15.004187980209167),
                    Centimeter(-8.61564340053936),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(13.267700209337248),
                    Centimeter(-8.637344281072494),
                ),
            },
            // 24
            Knot {
                control: FergusonPoint::control(
                    Centimeter(11.498449118768706),
                    Centimeter(-7.209828107048276),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(10.118710543558842),
                    Centimeter(-5.442599841625735),
                ),
            },
            // 25
            Knot {
                control: FergusonPoint::control(
                    Centimeter(8.757031249999997),
                    Centimeter(-3.5929687500000007),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(7.332031250000002),
                    Centimeter(-0.7474609375000006),
                ),
            },
            // 26
            Knot {
                control: FergusonPoint::control(
                    Centimeter(6.344335937500004),
                    Centimeter(-3.4855468750000003),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(4.606249999999999),
                    Centimeter(-3.50703125),
                ),
            },
            // 27
            Knot {
                control: FergusonPoint::control(
                    Centimeter(6.268164062499997),
                    Centimeter(-5.420898437499998),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(3.3839843750000016),
                    Centimeter(-6.825781250000002),
                ),
            },
            // 28
            Knot {
                control: FergusonPoint::control(
                    Centimeter(3.815820312500003),
                    Centimeter(-3.939648437499996),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(-3.3201171875000006),
                    Centimeter(-1.1154296875000018),
                ),
            },
            // 29
            Knot {
                control: FergusonPoint::control(
                    Centimeter(0.9818359374999983),
                    Centimeter(-3.939453124999999),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(-0.24082031250000108),
                    Centimeter(-3.5933593749999986),
                ),
            },
            // 30
            Knot {
                control: FergusonPoint::control(
                    Centimeter(2.1382812499999995),
                    Centimeter(-6.560546874999997),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(2.5269531249999964),
                    Centimeter(-7.505468749999999),
                ),
            },
            // 31
            Knot {
                control: FergusonPoint::control(
                    Centimeter(1.9167968749999984),
                    Centimeter(-8.800585937500001),
                ),
                tangent: FergusonPoint::tangent(
                    Centimeter(0.03710937500000225),
                    Centimeter(-9.857421874999996),
                ),
            },
        ]
    }
}
