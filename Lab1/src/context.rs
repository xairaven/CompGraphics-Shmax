use crate::ui::modals::error::ErrorModal;
use crate::utils::channel::Channel;

#[derive(Debug, Default)]
pub struct Context {
    pub errors_channel: Channel<ErrorModal>,
}
