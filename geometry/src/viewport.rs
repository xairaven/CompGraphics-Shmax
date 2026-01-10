use crate::primitives::point2d::Point2DPixel;
use crate::units::{Centimeter, Pixel};
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

    pub fn update_state(&mut self, response: &Response) {
        let bounds = ViewportBounds::from(response);
        // Update zero point
        let zero_point = self.geometry.zero_point_location.get_point(&bounds);
        self.state.zero_point = zero_point;
        // Update viewport location
        self.state.bounds = bounds;
    }

    pub fn viewport_bounds_centimeter(&self) -> ViewportBounds<Centimeter> {
        let bounds = &self.state.bounds;

        ViewportBounds::<Centimeter> {
            minimum_x: bounds.minimum_x.to_centimeters_x(self),
            maximum_x: bounds.maximum_x.to_centimeters_x(self),
            minimum_y: bounds.minimum_y.to_centimeters_y(self),
            maximum_y: bounds.maximum_y.to_centimeters_y(self),
            center_x: bounds.center_x.to_centimeters_x(self),
            center_y: bounds.center_y.to_centimeters_y(self),
        }
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
    pub zero_point_location: ZeroPointLocation,
    pub pixels_per_centimeter: f64,
    pub offset: Point2DPixel,
}

impl Default for ViewportGeometry {
    fn default() -> Self {
        Self {
            zero_point_location: ZeroPointLocation::Center,
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
    pub bounds: ViewportBounds<Pixel>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZeroPointLocation {
    Center,
    BottomLeftWithOffset { offset: Pixel },
    TopLeftWithOffset { offset: Pixel },
    BottomRightWithOffset { offset: Pixel },
    TopRightWithOffset { offset: Pixel },
}

impl ZeroPointLocation {
    pub fn get_point(&self, bounds: &ViewportBounds<Pixel>) -> Point2DPixel {
        match self {
            ZeroPointLocation::Center => Point2DPixel {
                x: bounds.center_x,
                y: bounds.center_y,
            },
            ZeroPointLocation::BottomLeftWithOffset { offset } => Point2DPixel::new(
                bounds.minimum_x.value() + offset.value(),
                bounds.maximum_y.value() - offset.value(),
            ),
            ZeroPointLocation::TopLeftWithOffset { offset } => Point2DPixel::new(
                bounds.minimum_x.value() + offset.value(),
                bounds.minimum_y.value() + offset.value(),
            ),
            ZeroPointLocation::BottomRightWithOffset { offset } => Point2DPixel::new(
                bounds.maximum_x.value() - offset.value(),
                bounds.maximum_y.value() - offset.value(),
            ),
            ZeroPointLocation::TopRightWithOffset { offset } => Point2DPixel::new(
                bounds.maximum_x.value() - offset.value(),
                bounds.minimum_y.value() + offset.value(),
            ),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ViewportBounds<Unit: Default + Clone> {
    pub minimum_x: Unit,
    pub maximum_x: Unit,
    pub minimum_y: Unit,
    pub maximum_y: Unit,
    pub center_x: Unit,
    pub center_y: Unit,
}

impl From<&Response> for ViewportBounds<Pixel> {
    fn from(response: &Response) -> Self {
        let (center_x, center_y) = response.rect.center().into();

        Self {
            minimum_x: Pixel(response.rect.min.x as f64),
            maximum_x: Pixel(response.rect.max.x as f64),
            minimum_y: Pixel(response.rect.min.y as f64),
            maximum_y: Pixel(response.rect.max.y as f64),
            center_x: Pixel(center_x as f64),
            center_y: Pixel(center_y as f64),
        }
    }
}
