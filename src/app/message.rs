use std::path::PathBuf;

use cosmic::widget::{markdown, menu::action::MenuAction, text_editor};
use cosmic_files::dialog::{DialogMessage, DialogResult};

#[derive(Debug, Clone)]
pub enum Message {
	Edit(text_editor::Action),
	Url(markdown::Url),
	Save,

	OpenFilePicker,
	DialogMessage(DialogMessage),
	OpenFileResult(DialogResult),

	OpenEditor(Option<PathBuf>),
	OpenHome,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MenuActions {
	Save,
	OpenFile,
	NewFile,
	GoHome,
}

impl MenuAction for MenuActions {
	type Message = Message;

	fn message(&self) -> Self::Message {
		match self {
			Self::Save => Message::Save,
			Self::OpenFile => Message::OpenFilePicker,
			Self::NewFile => Message::OpenEditor(None),
			Self::GoHome => Message::OpenHome,
		}
	}
}
