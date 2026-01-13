use crate::ui::modals::error::ErrorModal;
use crate::utils::channel::Channel;
use geometry::figures::detail::Detail;
use geometry::figures::grid::{Grid2D, Grid2DBuilder};
use geometry::pipeline::Pipeline;
use geometry::transformations::affine::Affine;
use geometry::transformations::affine::scaling::AffineScaling;
use geometry::transformations::affine::symmetry::AffinePointSymmetry;
use geometry::transformations::euclidean::offset::EuclideanOffset;
use geometry::transformations::euclidean::rotation::EuclideanRotation;
use geometry::transformations::projective::Projective;
use geometry::units::{Centimeter, Pixel};
use geometry::viewport::{Viewport, ViewportGeometry, ViewportState, ZeroPointLocation};

#[derive(Debug)]
pub struct Context {
    pub figures: FiguresState,
    pub transformations: TransformContext,
    pub viewport: Viewport,
    pub errors_channel: Channel<ErrorModal>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            figures: FiguresState::default(),
            transformations: TransformContext::default(),

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

impl Context {
    pub fn reset(&mut self) {
        *self = Default::default();
    }

    pub fn fixating_grid(&mut self) {
        self.viewport.geometry.fixed_grid = self.transformations.affine.is_enabled
            || self.transformations.scale.is_enabled
            || self.transformations.projective.is_enabled;
    }
}

#[derive(Debug)]
pub struct FiguresState {
    pub grid: Grid2D,
    pub grid_pipeline: Pipeline,

    pub detail: Detail,
    pub detail_pipeline: Pipeline,
}

impl Default for FiguresState {
    fn default() -> Self {
        Self {
            grid: Grid2DBuilder::default()
                .with_unit(Centimeter(5.0))
                .with_bounds_x(Some(Centimeter(0.0)), Some(Centimeter(120.0)))
                .with_bounds_y(Some(Centimeter(0.0)), Some(Centimeter(120.0)))
                .build(),
            grid_pipeline: Default::default(),
            detail: Default::default(),
            detail_pipeline: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct TransformContext {
    pub offset: EuclideanOffset,
    pub rotation: EuclideanRotation,
    pub affine: Affine,
    pub scale: AffineScaling,
    pub symmetry: AffinePointSymmetry,
    pub projective: Projective,
}
