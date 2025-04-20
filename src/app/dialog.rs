use cosmic::{Action, Element, app::Task, widget};
use cosmic_files::dialog::{Dialog, DialogResult};
use tracing::{info, warn};

use super::message::Message;

pub struct DialogManager(Option<Dialog<Message>>);

impl DialogManager {
	pub fn new() -> Self {
		Self(None)
	}

	pub fn view_window(&self, id: cosmic::iced::window::Id) -> Element<Message> {
		let Self(inner) = self;

		match inner {
			Some(dialog) => dialog.view(id),
			None => widget::text("Unknown Window ID").into(),
		}
	}

	pub fn update(&mut self, message: &Message) -> Option<Task<Message>> {
		let task_none = Some(Task::none());

		match message {
			Message::OpenFilePicker if self.0.is_none() => {
				let (dialog, task) = Dialog::new(
					cosmic_files::dialog::DialogKind::OpenFile,
					None,
					Message::DialogMessage,
					Message::OpenFileResult,
				);

				self.0 = Some(dialog);
				Some(task)
			}

			Message::DialogMessage(dialog_message) => {
				if let Some(dialog) = &mut self.0 {
					Some(dialog.update(dialog_message.clone()))
				} else {
					task_none
				}
			}

			Message::OpenFileResult(DialogResult::Open(paths)) => {
				let Some(path) = paths.first() else {
					warn!("No paths selected!");
					return task_none;
				};

				info!("File {:?} selected", path);

				let message = Message::OpenEditor(Some(path.clone()));
				Some(Task::done(Action::App(message)))
			}

			_ => {
				self.0 = None;
				None
			}
		}
	}
}
