use crate::ui::modals::error::ErrorModal;
use crate::utils::channel::Channel;
use geometry::figures::grid3d::Grid3D;
use geometry::projections::twopoint::TwoPointPerspective;
use geometry::transformations::euclidean::offset3d::EuclideanOffset3D;
use geometry::transformations::euclidean::rotation3d::EuclideanRotation3D;
use geometry::viewport::{Viewport, ViewportGeometry, ViewportState, ZeroPointLocation};

#[derive(Debug)]
pub struct Context {
    pub figures: FiguresState,
    pub projections: ProjectionsContext,
    pub transformations: TransformContext,
    pub animations: AnimationsContext,
    pub viewport: Viewport,
    pub errors_channel: Channel<ErrorModal>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            figures: FiguresState::default(),
            projections: ProjectionsContext::default(),
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

#[derive(Debug, Default)]
pub struct FiguresState {
    pub grid: Grid3D,
}

#[derive(Debug, Default)]
pub struct TransformContext {
    pub offset: EuclideanOffset3D,
    pub rotation: EuclideanRotation3D,
}

#[derive(Debug, Default)]
pub struct AnimationsContext;

#[derive(Debug, Default)]
pub struct ProjectionsContext {
    pub twopoint: TwoPointPerspective,
}
