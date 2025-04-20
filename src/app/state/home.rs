use cosmic::app::Task;

use crate::app::{flags::Flags, message::Message};

use super::Screen;

// TODO: Store recetly accessed files
pub struct Home {}

impl Home {
	pub fn new() -> Self {
		Self {}
	}
}

impl Screen for Home {
	fn view<'flags>(&'flags self, _flags: &'flags Flags) -> cosmic::Element<'flags, Message> {
		cosmic::widget::button::text("label")
			.on_press(Message::OpenFilePicker)
			.into()
	}

	fn update<'flags>(
		&'flags mut self,
		_flags: &'flags mut Flags,
		_message: Message,
	) -> Task<Message> {
		Task::none()
	}
}
