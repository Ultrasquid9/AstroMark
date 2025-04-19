use cosmic::{Application, ApplicationExt, Core, Element, app::Task, executor};
use flags::Flags;
use message::Message;
use state::State;

pub mod flags;
pub mod message;
pub mod state;

// TODO - Translations
const APP_TITLE: &str = "AstroMark";

pub struct AstroMark {
	core: Core,
	flags: Flags,

	state: State,
}

impl Application for AstroMark {
	type Executor = executor::Default;
	type Message = Message;
	type Flags = Flags;

	const APP_ID: &'static str = "uwu.juni.astromark";

	fn core(&self) -> &Core {
		&self.core
	}

	fn core_mut(&mut self) -> &mut Core {
		&mut self.core
	}

	fn init(core: Core, flags: Self::Flags) -> (Self, Task<Self::Message>) {
		let mut app = Self {
			core,
			flags,

			state: State::new(),
		};

		let tasks = [app.set_window_title(APP_TITLE.into())];

		app.update_header_title();
		(app, Task::batch(tasks))
	}

	fn view(&self) -> Element<Self::Message> {
		self.state.view(&self.flags)
	}

	fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
		self.update_header_title();
		self.state.update(message)
	}
}

impl AstroMark {
	fn update_header_title(&mut self) {
		self.set_header_title(format!("{APP_TITLE} - {}", self.state));
	}
}
