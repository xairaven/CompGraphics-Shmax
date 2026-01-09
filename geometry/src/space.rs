use crate::convertible::Convertible;
use crate::primitives::line2d::Line2D;
use crate::primitives::point2d::Point2D;
use egui::{InputState, Response};
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Space {
    Local,
    Screen,
}

pub const PX_PER_CM_RANGE: RangeInclusive<i64> = 10..=100;

#[derive(Debug)]
pub struct SpaceContext {
    pub settings: SpaceSettings,
    pub state: SpaceState,
}

impl SpaceContext {
    pub fn handle_drag(&mut self, ui: &mut egui::Ui, response: Response) {
        if self.settings.is_drag_enabled && response.dragged() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);

            let delta = response.drag_delta();
            let dragging_coefficient = 1.0;

            self.state.offset.0 += delta.x as f64 * dragging_coefficient;
            self.state.offset.1 += delta.y as f64 * dragging_coefficient;

            ui.ctx().request_repaint();
        }
    }
}

// Stored in pixels, unit length in centimeters
#[derive(Debug)]
pub struct SpaceSettings {
    pub zero_point: Point2D,
    pub size: SpaceSize,
    pub unit_length: f64,
    pub is_drag_enabled: bool,
}

// Stored in pixels
#[derive(Debug)]
pub struct SpaceState {
    pub pixels_per_centimeter: f64,
    pub offset: (f64, f64),
}

impl Default for SpaceState {
    fn default() -> Self {
        Self {
            pixels_per_centimeter: 20.0,
            offset: (0.0, 0.0),
        }
    }
}

impl SpaceState {
    pub fn handle_scroll(&mut self, input_state: &InputState) {
        let delta = input_state.smooth_scroll_delta.y;

        self.pixels_per_centimeter += (delta as f64) * 0.1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpaceSize {
    pub width: f64,
    pub height: f64,
}

pub trait Shapeable {
    fn screen_shape(&self, space_context: &SpaceContext) -> egui::Shape;
}

impl Shapeable for Line2D {
    fn screen_shape(&self, space_context: &SpaceContext) -> egui::Shape {
        self.centimeters_to_pixels(space_context).to_shape()
    }
}
