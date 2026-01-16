use crate::ui::modals::error::ErrorModal;
use crate::utils::channel::Channel;
use geometry::figures::contour::Contour;
use geometry::figures::grid::{Grid2D, Grid2DBuilder};
use geometry::pipeline::Pipeline;
use geometry::transformations::euclidean::offset::EuclideanOffset;
use geometry::transformations::euclidean::rotation::EuclideanRotation;
use geometry::units::Centimeter;
use geometry::viewport::{Viewport, ViewportGeometry, ViewportState, ZeroPointLocation};

#[derive(Debug)]
pub struct Context {
    pub figures: FiguresState,
    pub transformations: TransformContext,
    pub animations: AnimationsContext,
    pub viewport: Viewport,
    pub errors_channel: Channel<ErrorModal>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            figures: FiguresState::default(),
            transformations: TransformContext::default(),
            animations: AnimationsContext,

            viewport: Viewport {
                // Default settings like panning, zooming, etc.
                config: Default::default(),
                // Default geometry settings, can be updated by user
                geometry: ViewportGeometry {
                    zero_point_location: ZeroPointLocation::Center,
                    ..Default::default()
                },
                // Initial viewport state, will be updated when the UI is built
                state: ViewportState::default(),
            },

            errors_channel: Channel::default(),
        }
    }
}

impl Context {
    pub fn reset(&mut self) {
        *self = Default::default();
    }
}

#[derive(Debug)]
pub struct FiguresState {
    pub grid: Grid2D,
    pub grid_pipeline: Pipeline,

    pub contour: Contour,
    pub contour_pipeline: Pipeline,
}

impl Default for FiguresState {
    fn default() -> Self {
        Self {
            grid: Grid2DBuilder::default().with_unit(Centimeter(5.0)).build(),
            grid_pipeline: Default::default(),

            contour: Contour::default(),
            contour_pipeline: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct TransformContext {
    pub offset: EuclideanOffset,
    pub rotation: EuclideanRotation,
}

#[derive(Debug, Default)]
pub struct AnimationsContext;
