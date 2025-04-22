use cosmic::{
	Element,
	app::Task,
	iced_widget::{Column, center, column, row},
	widget::{button, horizontal_space},
};

use crate::{app::message::Message, trans, utils::cfg::flags::Flags};

use super::Screen;

pub struct Home {}

impl Home {
	pub fn new() -> Self {
		Self {}
	}
}

impl Screen for Home {
	fn view<'flags>(&'flags self, flags: &'flags Flags) -> Element<'flags, Message> {
		row![
			center(column![
				button::text(trans!("open_file")).on_press(Message::OpenFilePicker),
				button::text(trans!("new_file")).on_press(Message::OpenEditor(None)),
			]),
			horizontal_space().width(flags.text_size * 2.),
			center(Column::with_children([])),
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
