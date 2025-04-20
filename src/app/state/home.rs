use cosmic::{app::Task, iced_widget::row};

use super::Screen;

// TODO: Store recetly accessed files
pub struct Home {}

impl Home {
	pub fn new() -> Self {
		Self {}
	}
}

impl Screen for Home {
	fn view<'flags>(
		&'flags self,
		_flags: &'flags crate::app::flags::Flags,
	) -> cosmic::Element<'flags, crate::app::message::Message> {
		row![].into()
	}

	fn update<'flags>(
		&'flags mut self,
		_flags: &'flags mut crate::app::flags::Flags,
		_message: crate::app::message::Message,
	) -> Task<crate::app::message::Message> {
		//Dialog::n

		Task::none()
	}
}
