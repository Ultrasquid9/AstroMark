use cosmic::{app::Task, iced_widget::column, Element};

use crate::{app::{flags::Flags, message::Message}, trans};

use super::Screen;

// TODO: Store recetly accessed files
pub struct Home {}

impl Home {
	pub fn new() -> Self {
		Self {}
	}
}

impl Screen for Home {
	fn view<'flags>(&'flags self, _flags: &'flags Flags) -> Element<'flags, Message> {
		column![
			cosmic::widget::button::text(trans!("open_file"))
				.on_press(Message::OpenFilePicker),
			cosmic::widget::button::text(trans!("new_file"))
				.on_press(Message::OpenEditor(None)),
		]
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
