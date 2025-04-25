use std::{path::PathBuf, sync::Arc};

use cosmic::{
	Element,
	app::Task,
	iced::{
		Font, Length, highlighter,
		keyboard::{self, key::Named},
		theme::Palette,
	},
	iced_widget::{
		column,
		markdown::{self, Item},
		row, scrollable,
	},
	widget::{
		self, container, horizontal_space,
		text_editor::{self, Action, Binding, Edit},
		vertical_space,
	},
};
use tracing::{error, info, warn};

use crate::{
	app::message::{Message, task},
	trans,
	utils::cfg::script::ScriptCfg,
};

use super::Screen;

const TAB: char = '\t';

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

		let mut editor = Self {
			path,
			default_text: trans!("default_text"),
			text,
			md: vec![],
		};

		editor.parse_md();

		editor
	}

	fn parse_md(&mut self) {
		self.md = markdown::parse(&self.text.text()).collect()
	}

	/// Slightly hacky way to insert hard tabs
	fn hard_tab_hack(&mut self) {
		let action1 = Action::Edit(Edit::Paste(Arc::new("\ta".into())));
		let action2 = Action::Edit(Edit::Backspace);
		self.text.perform(action1);
		self.text.perform(action2);
	}
}

impl Screen for Editor {
	fn view<'flags>(&'flags self, flags: &'flags ScriptCfg) -> Element<'flags, Message> {
		let editor = widget::text_editor(&self.text)
			.key_binding(|kp| key_bindings(kp, flags))
			.placeholder(&self.default_text)
			.size(flags.flags.text_size)
			.font(Font::MONOSPACE)
			// TODO: Configurable Theme
			.highlight("markdown", highlighter::Theme::Base16Eighties)
			.height(Length::Fill)
			.padding(10)
			.on_action(Message::Edit);

		let markdown = markdown::view(
			self.md.iter(),
			markdown::Settings::with_text_size(flags.flags.text_size),
			// TODO: Configurable Theme
			markdown::Style::from_palette(Palette::CATPPUCCIN_FRAPPE),
		)
		.map(Message::Url);

		row![
			container(editor).padding(10),
			horizontal_space().width(flags.flags.text_size),
			scrollable(column![
				markdown,
				vertical_space().height(flags.flags.text_size * 10.)
			])
			.spacing(flags.flags.text_size)
		]
		.into()
	}

	fn update<'flags>(
		&'flags mut self,
		_: &'flags mut ScriptCfg,
		message: Message,
	) -> Task<Message> {
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

			Message::Edit(action) => {
				let is_edit = action.is_edit();

				if let Action::Edit(Edit::Insert(TAB)) = action {
					self.hard_tab_hack();
				} else {
					self.text.perform(action);
				}

				if is_edit {
					self.parse_md();
				}
			}

			Message::Url(url) => {
				info!("Opening {}", url.as_str());

				if let Err(e) = open::that(url.as_str()) {
					warn!("{e}")
				}
			}

			_ => (),
		}

		Task::none()
	}
}

fn key_bindings(kp: text_editor::KeyPress, flags: &ScriptCfg) -> Option<Binding<Message>> {
	// TODO: Custom bindings; Vim/Helix motions

	if let keyboard::Key::Named(Named::Tab) = kp.key {
		// Tabs

		let binding = if flags.flags.expand_tabs {
			Binding::Sequence(vec![Binding::Insert(' '); flags.flags.tab_len])
		} else {
			Binding::Insert(TAB)
		};

		Some(binding)
	} else {
		// Default bindings
		Binding::from_key_press(kp)
	}
}
