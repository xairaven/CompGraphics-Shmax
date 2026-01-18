use crate::figures::contour::Contour;
use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use crate::smooth::ferguson::FergusonCurve;
use egui::Stroke;

#[derive(Debug)]
pub struct Texture {
    pub style: TextureStyle,

    lines: Vec<Line2D<Point2D>>,
}

impl Default for Texture {
    fn default() -> Self {
        let contour = Contour {
            curve: FergusonCurve {
                knots: Contour::default_knots(),
                is_closed: true,
                step: 0.01,
                style: Default::default(),
            },
            is_tooltips_mode_enabled: false,
            is_skeleton_mode_enabled: false,
        };
        let lines = contour.lines();

        Self {
            style: Default::default(),
            lines,
        }
    }
}

impl Texture {
    pub fn lines(&self) -> Vec<Line2D<Point2D>> {
        self.lines
            .iter()
            .map(|line| {
                let mut line = *line;
                line.stroke = self.style.stroke;
                line
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct TextureStyle {
    pub stroke: Stroke,
}

impl Default for TextureStyle {
    fn default() -> Self {
        Self {
            // Pink
            stroke: Stroke::new(1.5, egui::Color32::from_rgb(255, 105, 180)),
        }
    }
}
