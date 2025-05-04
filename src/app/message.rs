use std::path::PathBuf;

use cosmic::{
	Action,
	app::Task,
	iced::keyboard::{Key, Modifiers},
	widget::{markdown, menu::action::MenuAction, segmented_button::Entity, text_editor},
};
use cosmic_files::dialog::{DialogMessage, DialogResult};

use crate::create_rhai_mod;

#[derive(Debug, Clone)]
pub enum Message {
	Edit(text_editor::Action),
	Parsed(Vec<markdown::Item>),
	Url(markdown::Url),
	Save,

	KeyPress(Key, Modifiers),
	Dialog(DialogMessage),

	SaveAsFilePicker,
	SaveAsFileResult(DialogResult),
	SaveAs(PathBuf),

	OpenFilePicker,
	OpenFileResult(DialogResult),

	OpenEditor(Option<PathBuf>),
	OpenHome,
	SwitchToTab(Entity),
	KillTab(Entity),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MenuActions {
	Save,
	SaveAs,
	OpenFile,
	NewFile,
	GoHome,
}

impl From<MenuActions> for Message {
	fn from(action: MenuActions) -> Self {
		match action {
			MenuActions::Save => Self::Save,
			MenuActions::SaveAs => Self::SaveAsFilePicker,
			MenuActions::OpenFile => Self::OpenFilePicker,
			MenuActions::NewFile => Self::OpenEditor(None),
			MenuActions::GoHome => Self::OpenHome,
		}
	}
}

impl MenuAction for MenuActions {
	type Message = Message;

	fn message(&self) -> Self::Message {
		Message::from(*self)
	}
}

pub fn task(message: Message) -> Task<Message> {
	Task::done(Action::App(message))
}

create_rhai_mod! {
	menu_actions(MenuActions) => [
		Save;
		SaveAs;
		OpenFile;
		NewFile;
		GoHome;
	]
}
