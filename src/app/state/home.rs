use cosmic::{
	Element,
	app::Task,
	iced_widget::{Column, center, column, row, text},
	widget::{button, horizontal_space, vertical_space},
};

use crate::{
	app::message::Message,
	trans,
	utils::cfg::{
		flags::Flags,
		get_or_create_cfg_file,
		recent::{DIR, Recent},
		script::ScriptCfg,
	},
};

use super::{Screen, format_path};

pub struct Home {
	pub recent: Recent,
}

impl Home {
	pub fn new() -> Self {
		Self {
			recent: Recent::read(get_or_create_cfg_file::<_, Recent>(DIR)),
		}
	}

	fn recent_buttons<'flags>(&self, flags: &'flags Flags) -> Vec<Element<'flags, Message>> {
		let mut buttons = self
			.recent
			.get_inner()
			.iter()
			.map(|path| {
				button::text(format_path(path))
					.font_size(flags.text_size as u16)
					.on_press(Message::OpenEditor(Some(path.clone())))
					.into()
			})
			.collect::<Vec<Element<'flags, Message>>>();

		buttons.push(vertical_space().height(flags.space()).into());
		buttons.push(text(trans!("recents")).size(flags.space()).into());
		buttons.reverse();

		buttons
	}
}

impl Screen for Home {
	fn view<'flags>(&'flags self, flags: &'flags ScriptCfg) -> Element<'flags, Message> {
		row![
			center(column![
				text(trans!("get_started")).size(flags.flags.space()),
				vertical_space().height(flags.flags.space()),
				button::text(trans!("open_file"))
					.font_size(flags.flags.text_size as u16)
					.on_press(Message::OpenFilePicker),
				button::text(trans!("new_file"))
					.font_size(flags.flags.text_size as u16)
					.on_press(Message::OpenEditor(None)),
			]),
			horizontal_space().width(flags.flags.text_size),
			center(Column::with_children(self.recent_buttons(&flags.flags))),
		]
		.into()
	}

	fn update<'flags>(
		&'flags mut self,
		_flags: &'flags mut ScriptCfg,
		_message: Message,
	) -> Task<Message> {
		Task::none()
	}
}
