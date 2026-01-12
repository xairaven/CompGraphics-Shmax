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

    pub fn update_chain(&mut self, segment_id: SegmentId) {
        let points = segment_id.points(&mut self.points);
        let length = segment_id.length(&mut self.lengths);
        Self::resize_line(points.0, points.1, length);

        let neighbours = segment_id.neighbours();
        for neighbour in neighbours {
            let points = neighbour.points(&mut self.points);
            let length = neighbour.length(&mut self.lengths);
            Self::update_line_length(points.0, points.1, length);
        }
    }

    pub fn update_line_length(
        p1: &mut Point2D, p2: &mut Point2D, length: &mut Centimeter,
    ) {
        *length = Centimeter(Line2D::with_transparent(*p1, *p2).length());
    }

    pub fn resize_line(start: &mut Point2D, end: &mut Point2D, length: &mut Centimeter) {
        let initial_length = Line2D::with_transparent(*start, *end).length();

        if initial_length < 1e-6 {
            return;
        }

        let unit_vector = Point2D {
            x: end.x - start.x,
            y: end.y - start.y,
        };
        let magnitude = Point2D {
            x: unit_vector.x / initial_length,
            y: unit_vector.y / initial_length,
        };
        let midpoint = {
            let x = (start.x + end.x) / 2.0;
            let y = (start.y + end.y) / 2.0;
            Point2D { x, y }
        };
        let new_start = Point2D {
            x: Centimeter(
                midpoint.x.value() - (length.value() / 2.0) * magnitude.x.value(),
            ),
            y: Centimeter(
                midpoint.y.value() - (length.value() / 2.0) * magnitude.y.value(),
            ),
        };
        let new_end = Point2D {
            x: Centimeter(
                midpoint.x.value() + (length.value() / 2.0) * magnitude.x.value(),
            ),
            y: Centimeter(
                midpoint.y.value() + (length.value() / 2.0) * magnitude.y.value(),
            ),
        };

        *start = new_start;
        *end = new_end;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SegmentId {
    AB,
    BC,
    CD,
    DE,
    EF,
    FG,
    GH,
    HI,
    IJ,
    JK,
    KL,
}

impl SegmentId {
    pub fn points<'a>(
        &self, points: &'a mut DetailPoints,
    ) -> (&'a mut Point2D, &'a mut Point2D) {
        match self {
            SegmentId::AB => (&mut points.a, &mut points.b),
            SegmentId::BC => (&mut points.b, &mut points.c),
            SegmentId::CD => (&mut points.c, &mut points.d),
            SegmentId::DE => (&mut points.d, &mut points.e),
            SegmentId::EF => (&mut points.e, &mut points.f),
            SegmentId::FG => (&mut points.f, &mut points.g),
            SegmentId::GH => (&mut points.g, &mut points.h),
            SegmentId::HI => (&mut points.h, &mut points.i),
            SegmentId::IJ => (&mut points.i, &mut points.j),
            SegmentId::JK => (&mut points.j, &mut points.k),
            SegmentId::KL => (&mut points.k, &mut points.l),
        }
    }

    pub fn length<'a>(&self, lengths: &'a mut DetailSideLengths) -> &'a mut Centimeter {
        match self {
            SegmentId::AB => &mut lengths.ab,
            SegmentId::BC => &mut lengths.bc,
            SegmentId::CD => &mut lengths.cd,
            SegmentId::DE => &mut lengths.de,
            SegmentId::EF => &mut lengths.ef,
            SegmentId::FG => &mut lengths.fg,
            SegmentId::GH => &mut lengths.gh,
            SegmentId::HI => &mut lengths.hi,
            SegmentId::IJ => &mut lengths.ij,
            SegmentId::JK => &mut lengths.jk,
            SegmentId::KL => &mut lengths.kl,
        }
    }

    pub fn neighbours(&self) -> &'static [SegmentId] {
        match self {
            SegmentId::AB => &[SegmentId::BC],
            SegmentId::BC => &[SegmentId::AB, SegmentId::CD],
            SegmentId::CD => &[SegmentId::BC, SegmentId::DE],
            SegmentId::DE => &[SegmentId::CD, SegmentId::EF],
            SegmentId::EF => &[SegmentId::DE, SegmentId::FG],
            SegmentId::FG => &[SegmentId::EF, SegmentId::GH],
            SegmentId::GH => &[SegmentId::FG, SegmentId::HI],
            SegmentId::HI => &[SegmentId::GH, SegmentId::IJ],
            SegmentId::IJ => &[SegmentId::HI, SegmentId::JK],
            SegmentId::JK => &[SegmentId::IJ, SegmentId::KL],
            SegmentId::KL => &[SegmentId::JK],
        }
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
