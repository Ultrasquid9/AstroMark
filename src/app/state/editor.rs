use std::path::PathBuf;

use cosmic::{
	Element,
	app::Task,
	iced::{
		keyboard::{self, key::Named},
		theme::Palette,
	},
	iced_widget::row,
	widget::{
		horizontal_space,
		markdown::{self, Item},
		text_editor,
	},
};
use tracing::{error, info, warn};

use crate::{
	app::message::{Message, task},
	trans,
	utils::cfg::flags::Flags,
};

use super::Screen;

pub struct Editor {
	path: Option<PathBuf>,
	default_text: String,
	text: text_editor::Content,
	md: Vec<Item>,
}

impl Editor {
	pub fn new(path: Option<PathBuf>) -> Self {
		let text = if let Some(path) = &path {
			let str = match std::fs::read_to_string(path) {
				Ok(str) => str,
				Err(e) => {
					error!("File could not be read: {e}");
					"".into()
				}
			};
			text_editor::Content::with_text(&str)
		} else {
			text_editor::Content::new()
		};

		Self {
			path,
			default_text: trans!("default_text"),
			text,
			md: vec![],
		}
	}
}

impl Screen for Editor {
	fn view<'flags>(&'flags self, flags: &'flags Flags) -> Element<'flags, Message> {
		row![
			text_editor(&self.text)
				.key_binding(|kp| key_bindings(kp, flags))
				.placeholder(&self.default_text)
				.height(u16::MAX) // I doubt monitors will get that large anytime soon
				.size(flags.text_size)
				.on_action(Message::Edit),
			horizontal_space().width(flags.text_size * 2.),
			markdown::view(
				self.md.iter(),
				markdown::Settings::with_text_size(flags.text_size),
				markdown::Style::from_palette(Palette::CATPPUCCIN_FRAPPE)
			)
			.map(Message::Url)
		]
		.into()
	}

	fn update<'flags>(&'flags mut self, _: &'flags mut Flags, message: Message) -> Task<Message> {
		match message {
			Message::Save => {
				let Some(path) = self.path.clone() else {
					return task(Message::SaveAsFilePicker);
				};

				if let Err(e) = std::fs::write(&path, self.text.text()) {
					error!("Error when saving: {e}");
				} else {
					info!("File {:?} saved successfully!", path)
				}
			}

			Message::SaveAs(path) => {
				self.path = Some(path);
				return task(Message::Save);
			}

			Message::Edit(action) => self.text.perform(action),
			Message::Url(url) => {
				info!("Opening {}", url.as_str());

				if let Err(e) = open::that(url.as_str()) {
					warn!("{e}")
				}
			}

			_ => (),
		}

		// TODO: Make async
		self.md = markdown::parse(&self.text.text()).collect();

		Task::none()
	}
}

fn key_bindings(kp: text_editor::KeyPress, flags: &Flags) -> Option<text_editor::Binding<Message>> {
	// TODO: Custom bindings; Vim/Helix motions maybe?
	// Lua/Rhai config would be epic

	if let keyboard::Key::Named(Named::Tab) = kp.key {
		// Tabs
		// TODO: Find some way to use hard tabs instead of spaces
		Some(text_editor::Binding::Sequence(
			vec![text_editor::Binding::Insert(' '); flags.tab_len],
		))
	} else {
		// Default bindings
		text_editor::Binding::from_key_press(kp)
	}
}
