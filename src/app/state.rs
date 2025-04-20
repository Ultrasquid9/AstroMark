use std::fmt::Display;

use cosmic::{Element, app::Task};

use crate::trans;

use super::{flags::Flags, message::Message};

pub mod editor;
pub mod home;

pub trait Screen {
	fn view<'flags>(&'flags self, flags: &'flags Flags) -> Element<'flags, Message>;

	fn update<'flags>(
		&'flags mut self,
		flags: &'flags mut Flags,
		message: Message,
	) -> Task<Message>;
}

pub enum State {
	Editor(editor::Editor),
	Home(home::Home),
}

impl State {
	pub fn new() -> Self {
		Self::Home(home::Home::new())
	}
}

impl Screen for State {
	fn view<'flags>(&'flags self, flags: &'flags Flags) -> Element<'flags, Message> {
		let screen: &dyn Screen = match self {
			Self::Editor(editor) => editor,
			Self::Home(home) => home,
		};
		screen.view(flags)
	}

	fn update<'flags>(
		&'flags mut self,
		flags: &'flags mut Flags,
		message: Message,
	) -> Task<Message> {
		match &message {
			Message::OpenEditor(path) => *self = Self::Editor(editor::Editor::new(path.clone())),

			_ => (),
		}

		let screen: &mut dyn Screen = match self {
			Self::Editor(editor) => editor,
			Self::Home(home) => home,
		};
		screen.update(flags, message)
	}
}

impl Display for State {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			State::Editor(_) => f.write_str(&trans!("editor")),
			State::Home(_) => f.write_str(&trans!("home")),
		}
	}
}
