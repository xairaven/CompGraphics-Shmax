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
    pub unit_length: f64,
    pub is_drag_enabled: bool,
}

#[derive(Debug)]
pub struct SpaceState {
    pub pixels_per_centimeter: f64,
    pub offset: (f64, f64),
}

impl SpaceState {
    pub fn update_offset(
        &mut self, ui: &egui::Ui, response: &egui::Response, settings: &SpaceSettings,
    ) {
        if settings.is_drag_enabled && response.dragged() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);

            let delta = response.drag_delta();
            let dragging_coefficient = 1.0;

            self.offset.0 += delta.x as f64 * dragging_coefficient;
            self.offset.1 += delta.y as f64 * dragging_coefficient;

            ui.ctx().request_repaint();
        }
    }
}
