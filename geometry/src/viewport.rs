use crate::primitives::point2d::Point2DPixel;
use crate::units::Pixel;
use egui::{InputState, Response};
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Viewport {
    pub config: ViewportConfig,
    pub geometry: ViewportGeometry,
    pub state: ViewportState,
}

impl Viewport {
    pub fn handle_pan(&mut self, ui: &mut egui::Ui, response: Response) {
        if self.config.is_pannable && response.dragged() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);

            let delta = response.drag_delta();
            const DRAGGING_COEFFICIENT: f64 = 1.0;

            self.geometry.offset.x += Pixel(delta.x as f64 * DRAGGING_COEFFICIENT);
            self.geometry.offset.y += Pixel(delta.y as f64 * DRAGGING_COEFFICIENT);

            ui.ctx().request_repaint();
        }
    }

    pub fn handle_scroll(&mut self, input_state: &InputState) {
        let delta = input_state.smooth_scroll_delta.y;

        self.geometry.pixels_per_centimeter += (delta as f64) * 0.1;
    }

    pub fn update_state(
        &mut self, response: &Response, is_grid_with_negative_sectors: bool,
    ) {
        // Update viewport location
        self.state.location = ViewportLocation::from(response);

        // Update zero point
        let point = if is_grid_with_negative_sectors {
            Point2DPixel::from(response.rect.center())
        } else {
            const OFFSET: f64 = 50.0;

            let x = self.state.location.minimum_x.value() + OFFSET;
            let y = self.state.location.maximum_y.value() - OFFSET;

            Point2DPixel::new(x, y)
        };
        self.state.zero_point = point;
    }
}

#[derive(Debug)]
pub struct ViewportConfig {
    pub is_pannable: bool,
    pub is_zoomable: bool,
}

impl Default for ViewportConfig {
    fn default() -> Self {
        Self {
            is_pannable: true,
            is_zoomable: true,
        }
    }
}

pub const PX_PER_CM_RANGE: RangeInclusive<f64> = 10.0..=100.0;

#[derive(Debug)]
pub struct ViewportGeometry {
    pub pixels_per_centimeter: f64,
    pub offset: Point2DPixel,
}

impl Default for ViewportGeometry {
    fn default() -> Self {
        Self {
            pixels_per_centimeter: 20.0,
            offset: Point2DPixel {
                x: Pixel(0.0),
                y: Pixel(0.0),
            },
        }
    }
}

#[derive(Debug, Default)]
pub struct ViewportState {
    pub zero_point: Point2DPixel,
    pub location: ViewportLocation,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ViewportLocation {
    pub minimum_x: Pixel,
    pub maximum_x: Pixel,
    pub minimum_y: Pixel,
    pub maximum_y: Pixel,
}

impl From<&Response> for ViewportLocation {
    fn from(response: &Response) -> Self {
        Self {
            minimum_x: Pixel(response.rect.min.x as f64),
            maximum_x: Pixel(response.rect.max.x as f64),
            minimum_y: Pixel(response.rect.min.y as f64),
            maximum_y: Pixel(response.rect.max.y as f64),
        }
    }
}
