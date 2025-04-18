use cosmic::Element;

use super::{flags::Flags, message::Message};

pub mod editor;

pub enum State {
	Editor(editor::Editor),
}

impl State {
	pub fn new() -> Self {
		Self::Editor(editor::Editor::new())
	}

	pub fn view(&self, flags: &Flags) -> Element<Message> {
		match self {
			Self::Editor(editor) => editor.view(flags),
		}
	}

	pub fn update(&mut self, message: Message) -> cosmic::app::Task<Message> {
		match self {
			Self::Editor(editor) => editor.update(message),
		}
	}
}
