use cosmic::widget::{markdown, text_editor};

#[derive(Debug, Clone)]
pub enum Message {
	Edit(text_editor::Action),
	Url(markdown::Url),
}
