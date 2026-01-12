use crate::primitives::circle::{CircularShape, ShapeType};
use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::units::Centimeter;
use egui::Stroke;

pub mod defaults {
    use egui::{Color32, Stroke};

    pub const DETAIL_BLACK: Stroke = Stroke {
        width: 3.0,
        color: Color32::BLACK,
    };
}

#[derive(Debug)]
pub struct Detail {
    pub points: DetailPoints,
    pub lengths: DetailSideLengths,
    pub radiuses: DetailRadiuses,

    pub stroke: Stroke,
}

impl Default for Detail {
    fn default() -> Self {
        Self {
            points: DetailPoints::default(),
            lengths: DetailSideLengths::default(),
            radiuses: DetailRadiuses::default(),
            stroke: defaults::DETAIL_BLACK,
        }
    }
}

impl Detail {
    pub fn lines(&self) -> Vec<Line2D<Point2D>> {
        let mut lines = vec![];

        let points = vec![
            &self.points.a,
            &self.points.b,
            &self.points.c,
            &self.points.d,
            &self.points.e,
            &self.points.f,
            &self.points.g,
            &self.points.h,
            &self.points.i,
            &self.points.j,
            &self.points.k,
            &self.points.l,
        ];
        for window in points.windows(2) {
            let line = Line2D {
                start: *window[0],
                end: *window[1],
                stroke: self.stroke,
            };
            lines.push(line);
        }

        let outer_circle = CircularShape::from_points_and_radius(
            self.points.l,
            self.points.a,
            self.radiuses.outer,
            self.stroke,
        )
        .lines(128);
        lines.extend(outer_circle);

        let inner_circle = CircularShape {
            center: self.points.m,
            radius: self.radiuses.inner,
            shape_type: ShapeType::Full,
            stroke: defaults::DETAIL_BLACK,
        }
        .lines(128);
        lines.extend(inner_circle);

        lines
    }
}

#[derive(Debug)]
pub struct DetailPoints {
    pub a: Point2D,
    pub b: Point2D,
    pub c: Point2D,
    pub d: Point2D,
    pub e: Point2D,
    pub f: Point2D,
    pub g: Point2D,
    pub h: Point2D,
    pub i: Point2D,
    pub j: Point2D,
    pub k: Point2D,
    pub l: Point2D,
    pub m: Point2D,
}

impl Default for DetailPoints {
    fn default() -> Self {
        Self {
            a: Point2D::new(30.0, 30.0),
            b: Point2D::new(50.0, 30.0),
            c: Point2D::new(50.0, 10.0),
            d: Point2D::new(70.0, 10.0),
            e: Point2D::new(70.0, 34.0),
            f: Point2D::new(57.0, 34.0),
            g: Point2D::new(57.0, 66.0),
            h: Point2D::new(70.0, 66.0),
            i: Point2D::new(70.0, 116.0),
            j: Point2D::new(50.0, 116.0),
            k: Point2D::new(50.0, 70.0),
            l: Point2D::new(30.0, 70.0),
            m: Point2D::new(30.0, 50.0),
        }
    }
}

#[derive(Debug)]
pub struct DetailSideLengths {
    pub ab: Centimeter,
    pub bc: Centimeter,
    pub cd: Centimeter,
    pub de: Centimeter,
    pub ef: Centimeter,
    pub fg: Centimeter,
    pub gh: Centimeter,
    pub hi: Centimeter,
    pub ij: Centimeter,
    pub jk: Centimeter,
    pub kl: Centimeter,
}

impl Default for DetailSideLengths {
    fn default() -> Self {
        Self {
            ab: Centimeter(20.0),
            bc: Centimeter(20.0),
            cd: Centimeter(20.0),
            de: Centimeter(24.0),
            ef: Centimeter(13.0),
            fg: Centimeter(32.0),
            gh: Centimeter(13.0),
            hi: Centimeter(50.0),
            ij: Centimeter(20.0),
            jk: Centimeter(46.0),
            kl: Centimeter(20.0),
        }
    }
}

#[derive(Debug)]
pub struct DetailRadiuses {
    pub inner: Centimeter,
    pub outer: Centimeter,
}

impl Default for DetailRadiuses {
    fn default() -> Self {
        Self {
            inner: Centimeter(11.0),
            outer: Centimeter(20.0),
        }
    }
}
