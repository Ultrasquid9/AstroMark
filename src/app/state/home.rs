use cosmic::{
	Element,
	app::Task,
	iced_widget::{Column, center, column, row},
	widget::{button, horizontal_space},
};
use tracing::error;

use crate::{
	app::message::Message,
	trans,
	utils::cfg::{flags::Flags, get_or_create_cfg_file, recent::Recent},
};

use super::Screen;

const RECENTS: &str = "recent.ron";

pub struct Home {
	pub recent: Recent,
}

impl Home {
	pub fn new() -> Self {
		Self {
			recent: Recent::read(get_or_create_cfg_file(RECENTS)),
		}
	}
}

impl Screen for Home {
	fn view<'flags>(&'flags self, flags: &'flags Flags) -> Element<'flags, Message> {
		let buttons = self
			.recent
			.get_inner()
			.iter()
			.map(|path| {
				button::text(path.to_str().unwrap())
					.on_press(Message::OpenEditor(Some(path.clone())))
					.into()
			})
			.collect::<Vec<Element<'flags, Message>>>();

		row![
			center(column![
				button::text(trans!("open_file")).on_press(Message::OpenFilePicker),
				button::text(trans!("new_file")).on_press(Message::OpenEditor(None)),
			]),
			horizontal_space().width(flags.text_size * 2.),
			center(Column::with_children(buttons)),
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

impl Drop for Home {
	fn drop(&mut self) {
		let dir = get_or_create_cfg_file(RECENTS);

		let str = match ron::to_string(&self.recent) {
			Ok(ok) => ok,
			Err(e) => {
				error!("{e}");
				return;
			}
		};

		if let Err(e) = std::fs::write(dir, str) {
			error!("{e}");
		}
	}
}
