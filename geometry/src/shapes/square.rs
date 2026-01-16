use crate::primitives::point2d::Point2DPixel;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SquareMetadata {
    pub radius: f32,
    pub corner_radius: f32,
    pub fill: egui::Color32,
    pub stroke: egui::Stroke,
}

impl SquareMetadata {
    pub fn rect(&self, center: Point2DPixel) -> egui::Rect {
        let size = egui::Vec2::splat(2.0 * self.radius);
        egui::Rect::from_center_size(center.into(), size)
    }
}
