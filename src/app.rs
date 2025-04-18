use cosmic::{Application, ApplicationExt, Core, Element, app::Task, executor};
use flags::Flags;
use message::Message;
use state::State;

pub mod flags;
pub mod message;
pub mod state;

pub struct App {
	core: Core,
	flags: Flags,

	state: State,
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
		let mut app = Self {
			core,
			flags,

			state: State::new(),
		};

		let tasks = [app.set_window_title("EstroMD".into())];

		(app, Task::batch(tasks))
	}

	fn view(&self) -> Element<Self::Message> {
		self.state.view(&self.flags)
	}

	fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
		self.state.update(message)
	}
}
