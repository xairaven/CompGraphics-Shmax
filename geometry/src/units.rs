use derive_more::{
    Add, AddAssign, Display, Div, From, Into, Mul, MulAssign, Neg, Rem, Sub,
};
use std::ops::{Deref, DerefMut};

#[derive(
    Add,
    AddAssign,
    MulAssign,
    Sub,
    Mul,
    Div,
    Neg,
    Rem,
    From,
    Into,
    Display,
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
)]
pub struct Centimeter(pub f64);

impl Centimeter {
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Deref for Centimeter {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Centimeter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(
    Add,
    AddAssign,
    Sub,
    Mul,
    Div,
    Neg,
    Rem,
    From,
    Into,
    Display,
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
)]
pub struct Pixel(pub f64);

impl Pixel {
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Deref for Pixel {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pixel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(
    Add,
    AddAssign,
    MulAssign,
    Sub,
    Mul,
    Div,
    Neg,
    Rem,
    From,
    Into,
    Display,
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
)]
pub struct Percent(pub f64);

impl Percent {
    pub const RANGE: std::ops::RangeInclusive<f64> = 0.0..=1.0;

    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}
