pub mod epicycloid;

#[derive(Debug)]
pub enum Direction {
    Increase,
    Decrease,
}

impl Direction {
    pub fn factor(&self) -> f64 {
        match self {
            Self::Increase => 1.0,
            Self::Decrease => -1.0,
        }
    }

    pub fn toggle(&mut self) {
        *self = match self {
            Self::Increase => Self::Decrease,
            Self::Decrease => Self::Increase,
        };
    }
}
