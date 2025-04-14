use std::error::Error;

use app::{App, flags::Flags};
use cosmic::app::Settings;

mod app;

/* #[derive(Default)]
struct App {
	text: text_editor::Content,
	md: Vec<Item>,
}

#[derive(Clone, Debug)]
enum Message {
	Edit(text_editor::Action),
	Url(markdown::Url),
}

impl App {
	pub fn view(&self) -> Row<Message> {
		row![
			text_editor(&self.text)
				.placeholder("Type here")
				.on_action(Message::Edit),
			markdown::view(
				self.md.iter(),
				markdown::Settings::default(),
				markdown::Style::from_palette(iced::theme::Palette::CATPPUCCIN_FRAPPE)
			)
			.map(Message::Url)
		]
	}

	pub fn update(&mut self, message: Message) {
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
	}
}

fn main() -> iced::Result {
	iced::run("EstroMD", App::update, App::view)
}
 */

fn main() -> Result<(), Box<dyn Error>> {
	let settings = Settings::default();
	let flags = Flags::new();

	cosmic::app::run::<App>(settings, flags)?;
	Ok(())
}
