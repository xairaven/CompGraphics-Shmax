pub mod ferguson;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SmoothnessType {
    Break,
    Smooth,
}

impl SmoothnessType {
    pub fn toggle(&mut self) {
        *self = match self {
            Self::Break => Self::Smooth,
            Self::Smooth => Self::Break,
        }
    }
}
