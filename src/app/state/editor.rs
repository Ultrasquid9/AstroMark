use std::{path::PathBuf, sync::Arc};

use cosmic::{
	Element,
	app::Task,
	iced::{
		Font, Length,
		keyboard::{self, key::Named},
	},
	iced_widget::{column, row, scrollable},
	widget::{
		self, container, horizontal_space,
		markdown::{self, Item},
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

use super::{Screen, format_path};

const TAB: char = '\t';

pub struct Editor {
	path: Option<PathBuf>,
	dirty: bool,
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

		let md = markdown::parse(&text.text()).collect();

		Self {
			path,
			dirty: false,
			default_text: trans!("default_text"),
			text,
			md,
		}
	}

	pub fn name(&self) -> String {
		if let Some(path) = &self.path {
			format_path(path)
		} else {
			trans!("new_file")
		}
	}

	pub fn can_close(&self) -> bool {
		!self.dirty
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
	fn view<'cfg>(&'cfg self, cfg: &'cfg ScriptCfg) -> Element<'cfg, Message> {
		let space = horizontal_space().width(cfg.flags.text_size);

		let editor = widget::text_editor(&self.text)
			.key_binding(|kp| key_bindings(kp, cfg))
			.placeholder(&self.default_text)
			.size(cfg.flags.text_size - 1.5)
			.font(Font::MONOSPACE)
			.highlight("markdown", cfg.flags.highlight())
			.height(Length::Fill)
			.padding(10)
			.on_action(Message::Edit);

		let markdown = markdown::view(
			self.md.iter(),
			markdown::Settings::with_text_size(cfg.flags.text_size),
			markdown::Style::from_palette(cfg.flags.palette),
		)
		.map(Message::Url);

		row![
			container(editor).padding(10),
			row![
				space,
				scrollable(column![
					markdown,
					vertical_space().height(cfg.flags.text_size * 10.)
				])
				.width(Length::Fill)
				.height(Length::Fill)
				.spacing(cfg.flags.text_size)
			]
		]
		.into()
	}

	fn update<'cfg>(&'cfg mut self, _: &'cfg mut ScriptCfg, message: Message) -> Task<Message> {
		match message {
			Message::Save => {
				let Some(path) = self.path.clone() else {
					return task(Message::SaveAsFilePicker);
				};

				if let Err(e) = std::fs::write(&path, self.text.text()) {
					error!("Error when saving: {e}");
				} else {
					self.dirty = false;
					info!("File {:?} saved successfully!", path);
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
					self.dirty = true;
					return Task::future(parse_md(self.text.text()));
				}
			}

			Message::Parsed(md) => {
				self.md = md;
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
			Binding::Sequence(vec![Binding::Insert(' '); flags.flags.tab_len()])
		} else {
			Binding::Insert(TAB)
		};

		Some(binding)
	} else {
		// Default bindings
		Binding::from_key_press(kp)
	}
}

async fn parse_md(text: String) -> cosmic::Action<Message> {
	cosmic::Action::App(Message::Parsed(markdown::parse(&text).collect()))
}
