use crate::primitives::point2d::Point2D;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Space {
    Local,
    Screen,
}

pub const PX_PER_CM_RANGE: RangeInclusive<i64> = 10..=100;

pub struct SpaceContext {
    pub settings: SpaceSettings,
    pub state: SpaceState,
}

#[derive(Debug)]
pub struct SpaceSettings {
    pub zero_point: Point2D,
    pub center: Point2D,
    pub unit_length: f64,
}

#[derive(Debug)]
pub struct SpaceState {
    pub pixels_per_centimeter: f64,
    pub offset: (f64, f64),
}
