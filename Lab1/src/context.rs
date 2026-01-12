use crate::ui::modals::error::ErrorModal;
use crate::utils::channel::Channel;
use geometry::figures::detail::Detail;
use geometry::figures::grid::{Grid2D, Grid2DBuilder};
use geometry::units::{Centimeter, Pixel};
use geometry::viewport::{Viewport, ViewportGeometry, ViewportState, ZeroPointLocation};

#[derive(Debug)]
pub struct Context {
    pub figures: FiguresState,
    pub viewport: Viewport,
    pub errors_channel: Channel<ErrorModal>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            figures: FiguresState::default(),

            viewport: Viewport {
                // Default settings like panning, zooming, etc.
                config: Default::default(),
                // Default geometry settings, can be updated by user
                geometry: ViewportGeometry {
                    zero_point_location: ZeroPointLocation::BottomLeftWithOffset {
                        offset: Pixel(50.0),
                    },
                    ..Default::default()
                },
                // Initial viewport state, will be updated when the UI is built
                state: ViewportState::default(),
            },

            errors_channel: Channel::default(),
        }
    }
}

#[derive(Debug)]
pub struct FiguresState {
    pub grid: Grid2D,
    pub detail: Detail,
}

impl Default for FiguresState {
    fn default() -> Self {
        Self {
            grid: Grid2DBuilder::default()
                .with_unit(Centimeter(5.0))
                .with_bounds_x(Some(Centimeter(0.0)), Some(Centimeter(120.0)))
                .with_bounds_y(Some(Centimeter(0.0)), Some(Centimeter(120.0)))
                .build(),
            detail: Default::default(),
        }
    }
}
