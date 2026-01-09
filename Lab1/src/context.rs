use crate::ui::modals::error::ErrorModal;
use crate::utils::channel::Channel;
use geometry::viewport::{Viewport, ViewportState};

#[derive(Debug)]
pub struct Context {
    pub viewport: Viewport,
    pub errors_channel: Channel<ErrorModal>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            viewport: Viewport {
                // Default settings like panning, zooming, etc.
                config: Default::default(),
                // Default geometry settings, can be updated by user
                geometry: Default::default(),
                // Initial viewport state, will be updated when the UI is built
                state: ViewportState::default(),
            },

            errors_channel: Channel::default(),
        }
    }
}
