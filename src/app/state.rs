use std::fmt::Display;

use cosmic::{Element, app::Task};

use super::{flags::Flags, message::Message};

pub mod editor;

pub enum State {
	Editor(editor::Editor),
}

impl State {
	pub fn new() -> Self {
		Self::Editor(editor::Editor::new())
	}

	pub fn view<'flags>(&'flags self, flags: &'flags Flags) -> Element<'flags, Message> {
		match self {
			Self::Editor(editor) => editor.view(flags),
		}
	}

	pub fn update(&mut self, message: Message) -> Task<Message> {
		match self {
			Self::Editor(editor) => editor.update(message),
		}
	}
}

impl Display for State {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			State::Editor(_) => f.write_str("Editor"),
		}
	}
}
