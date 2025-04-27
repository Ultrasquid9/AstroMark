use std::{fmt::Display, path::PathBuf};

use cosmic::{Element, app::Task};
use tracing::error;

use crate::{
	trans,
	utils::cfg::{
		get_or_create_cfg_file,
		recent::{self, Recent},
		script::ScriptCfg,
	},
};

use super::{Screen, message::Message};

pub mod editor;
pub mod home;

pub enum State {
	Editor(editor::Editor),
	Home(home::Home),
}

impl State {
	pub fn new() -> Self {
		Self::Home(home::Home::new())
	}

	pub fn can_overwrite(&self) -> bool {
		matches!(self, Self::Home(_))
	}

	pub fn from_message(message: &Message) -> Option<Self> {
		match message {
			Message::OpenEditor(path) => Some(Self::editor(path)),
			Message::OpenHome => Some(Self::home()),

			_ => None,
		}
	}

	fn home() -> Self {
		Self::Home(home::Home::new())
	}

	fn editor(path: &Option<PathBuf>) -> Self {
		if let Some(path) = path {
			let mut recent = Recent::read(get_or_create_cfg_file::<_, Recent>(recent::DIR));
			recent.add(path.clone());
			recent.write();
		}

		Self::Editor(editor::Editor::new(path.clone()))
	}
}

impl Screen for State {
	fn view<'flags>(&'flags self, flags: &'flags ScriptCfg) -> Element<'flags, Message> {
		let screen: &dyn Screen = match self {
			Self::Editor(editor) => editor,
			Self::Home(home) => home,
		};
		screen.view(flags)
	}

	fn update<'flags>(
		&'flags mut self,
		flags: &'flags mut ScriptCfg,
		message: Message,
	) -> Task<Message> {
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
			State::Editor(editor) => f.write_str(&editor.name()),
			State::Home(_) => f.write_str(&trans!("home")),
		}
	}
}

pub fn format_path(path: &PathBuf) -> String {
	let file_name = match path.file_name() {
		Some(file_name) => file_name,
		None => {
			error!("File {:?} has no name", path);
			return "Unnamed".into();
		}
	};

	let name = match file_name.to_str() {
		Some(name) => name,
		None => {
			error!("File {:?} has an invalid name", path);
			return "Invalid Name".into();
		}
	};

	name.into()
}
