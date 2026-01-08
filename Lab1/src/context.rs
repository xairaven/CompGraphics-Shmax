use crate::ui::modals::error::ErrorModal;
use crate::utils::channel::Channel;
use geometry::primitives::point2d::Point2D;
use geometry::space::{SpaceContext, SpaceSettings, SpaceSize};

#[derive(Debug)]
pub struct Context {
    pub space: SpaceContext,

    pub errors_channel: Channel<ErrorModal>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            space: SpaceContext {
                settings: SpaceSettings {
                    zero_point: Point2D::new(500.0, 500.0).with_space_screen(),
                    size: SpaceSize {
                        width: 100.0,
                        height: 100.0,
                    },
                    unit_length: 1.0,
                    is_drag_enabled: true,
                },
                state: Default::default(),
            },
            errors_channel: Channel::default(),
        }
    }
}
