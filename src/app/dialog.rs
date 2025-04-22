use std::path::PathBuf;

use cosmic::{Element, app::Task, widget};
use cosmic_files::dialog::{Dialog, DialogKind, DialogResult};
use tracing::{info, warn};

use crate::app::message::task;

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
			Message::Dialog(dialog_message) => {
				if let Some(dialog) = &mut self.0 {
					Some(dialog.update(dialog_message.clone()))
				} else {
					task_none
				}
			}

			Message::OpenFilePicker if self.0.is_none() => {
				self.picker(DialogKind::OpenFile, Message::OpenFileResult)
			}
			Message::OpenFileResult(DialogResult::Open(paths)) => {
				result(paths, |pth| Message::OpenEditor(Some(pth)))
			}

			Message::SaveAsFilePicker if self.0.is_none() => self.picker(
				DialogKind::SaveFile {
					filename: "unnamed.md".into(),
				},
				Message::SaveAsFileResult,
			),
			Message::SaveAsFileResult(DialogResult::Open(paths)) => result(paths, Message::SaveAs),

			_ => {
				self.0 = None;
				None
			}
		}
	}

	fn picker(
		&mut self,
		kind: DialogKind,
		fun: impl Fn(DialogResult) -> Message + 'static,
	) -> Option<Task<Message>> {
		let (dialog, task) = Dialog::new(kind, None, Message::Dialog, fun);

		self.0 = Some(dialog);
		Some(task)
	}
}

fn result(paths: &[PathBuf], fun: impl Fn(PathBuf) -> Message + 'static) -> Option<Task<Message>> {
	let Some(path) = paths.first() else {
		warn!("No paths selected!");
		return Some(Task::none());
	};

	info!("File {:?} selected", path);
	Some(task(fun(path.clone())))
}
