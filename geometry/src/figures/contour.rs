use crate::smooth::ferguson::{FergusonCurve, Knot};

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
    pub fn default_knots() -> Vec<Knot> {
        vec![]
    }
}
