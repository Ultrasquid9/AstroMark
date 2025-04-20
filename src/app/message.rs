use cosmic::widget::{markdown, text_editor};
use cosmic_files::dialog::{DialogMessage, DialogResult};

#[derive(Debug, Clone)]
pub enum Message {
	Edit(text_editor::Action),
	Url(markdown::Url),

	OpenFilePicker,
	DialogMessage(DialogMessage),
	OpenFileResult(DialogResult),
}
