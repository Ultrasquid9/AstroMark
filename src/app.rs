use cosmic::{
	Application, Core, 
	app::Task, 
	executor,
	iced::theme::Palette,
	iced_widget::row,
	widget::{
		markdown::{self, Item},
		text_editor,
	},
};
use flags::Flags;
use message::Message;

pub mod flags;
pub mod message;

pub struct App {
	core: Core,

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

			text: text_editor::Content::default(),
			md: vec![],
		};

		#[allow(unused)]
		let flags = flags;

		(app, Task::none())
	}

	fn view(&self) -> cosmic::Element<Self::Message> {
		row![
			text_editor(&self.text)
				.placeholder("Type here")
				.on_action(Message::Edit),
			markdown::view(
				self.md.iter(),
				markdown::Settings::default(),
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
				println!("opening {}", url.as_str());

				if let Err(e) = open::that(url.as_str()) {
					println!("{e}")
				}
			}
		}

		self.md = markdown::parse(&self.text.text()).collect();

		Task::none()
	}
}
