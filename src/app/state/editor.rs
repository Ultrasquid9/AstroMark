use cosmic::{
	Element, Task,
	iced::theme::Palette,
	iced_widget::row,
	widget::{
		horizontal_space,
		markdown::{self, Item},
		text_editor,
	},
};
use tracing::{info, warn};

use crate::app::{flags::Flags, message::Message};

pub struct Editor {
	text: text_editor::Content,
	md: Vec<Item>,
}

impl Editor {
	pub fn new() -> Self {
		Self {
			text: text_editor::Content::default(),
			md: vec![],
		}
	}

	pub fn view(&self, flags: &Flags) -> Element<Message> {
		row![
			text_editor(&self.text)
				.placeholder("Type here")
				.height(u16::MAX) // I doubt monitors will get that large anytime soon
				.size(flags.text_size)
				.on_action(Message::Edit),
			horizontal_space().width(flags.text_size * 2.),
			markdown::view(
				self.md.iter(),
				markdown::Settings::with_text_size(flags.text_size),
				markdown::Style::from_palette(Palette::CATPPUCCIN_FRAPPE)
			)
			.map(Message::Url)
		]
		.into()
	}

	pub fn update(&mut self, message: Message) -> cosmic::app::Task<Message> {
		match message {
			Message::Edit(action) => self.text.perform(action),
			Message::Url(url) => {
				info!("Opening {}", url.as_str());

				if let Err(e) = open::that(url.as_str()) {
					warn!("{e}")
				}
			}

			#[allow(unreachable_patterns)]
			_ => (),
		}

		self.md = markdown::parse(&self.text.text()).collect();

		Task::none()
	}
}
