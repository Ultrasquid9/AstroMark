use std::u16;

use cosmic::{
	Application, Core, Element,
	app::Task,
	executor,
	iced::theme::Palette,
	iced_widget::row,
	widget::{
		horizontal_space,
		markdown::{self, Item},
		text_editor,
	},
};
use flags::Flags;
use message::Message;
use tracing::{info, warn};

pub mod flags;
pub mod message;

pub struct App {
	core: Core,
	flags: Flags,

	text: text_editor::Content,
	md: Vec<Item>,
}

impl Application for App {
	type Executor = executor::Default;
	type Message = Message;
	type Flags = Flags;

	const APP_ID: &'static str = "uwu.juni.estromd";

	fn core(&self) -> &Core {
		&self.core
	}

	fn core_mut(&mut self) -> &mut Core {
		&mut self.core
	}

	fn init(core: Core, flags: Self::Flags) -> (Self, Task<Self::Message>) {
		let app = Self {
			core,
			flags,

			text: text_editor::Content::default(),
			md: vec![],
		};

		(app, Task::none())
	}

	fn view(&self) -> Element<Self::Message> {
		row![
			text_editor(&self.text)
				.placeholder("Type here")
				.height(u16::MAX) // I doubt monitors will get that large anytime soon
				.size(self.flags.text_size)
				.on_action(Message::Edit),
			horizontal_space().width(self.flags.text_size * 2.),
			markdown::view(
				self.md.iter(),
				markdown::Settings::with_text_size(self.flags.text_size),
				markdown::Style::from_palette(Palette::CATPPUCCIN_FRAPPE)
			)
			.map(Message::Url)
		]
		.into()
	}

	fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
		match message {
			Message::Edit(action) => self.text.perform(action),
			Message::Url(url) => {
				info!("Opening {}", url.as_str());

				if let Err(e) = open::that(url.as_str()) {
					warn!("{e}")
				}
			}
		}

		self.md = markdown::parse(&self.text.text()).collect();

		Task::none()
	}
}
