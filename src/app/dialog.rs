use cosmic::{Element, app::Task, widget};
use cosmic_files::dialog::{Dialog, DialogResult};
use tracing::info;

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
		match message {
			Message::OpenFilePicker if self.0.is_none() => {
				let (dialog, task) = Dialog::new(
					cosmic_files::dialog::DialogKind::OpenFile,
					None,
					Message::DialogMessage,
					Message::OpenFileResult,
				);

				self.0 = Some(dialog);
				return Some(task);
			}

			Message::DialogMessage(dialog_message) => {
				if let Some(dialog) = &mut self.0 {
					return Some(dialog.update(dialog_message.clone()));
				}

				return Some(Task::none());
			}

			Message::OpenFileResult(result) => {
				self.0 = None;

				if let DialogResult::Open(paths) = result {
					for p in paths {
						info!("{:?}", p)
					}
				}

				return Some(Task::none());
			}

			_ => None,
		}
	}
}
