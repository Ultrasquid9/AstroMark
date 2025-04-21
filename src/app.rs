use std::collections::HashMap;

use cosmic::{
	Application, ApplicationExt, Core, Element,
	app::Task,
	executor,
	iced::window::Id,
	widget::menu::{self, Item, ItemHeight},
};
use dialog::DialogManager;
use flags::Flags;
use message::{MenuActions, Message};
use state::{Screen, State};
use tracing::error;

use crate::trans;

pub mod dialog;
pub mod flags;
pub mod message;
pub mod state;

pub struct AstroMark {
	core: Core,
	flags: Flags,

	dialog: DialogManager,
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

			dialog: DialogManager::new(),
			state: State::new(),
		};

		let Some(id) = app.core.main_window_id() else {
			error!("App window ID not found!");
			panic!()
		};

		let tasks = [app.set_window_title(trans!("astromark"), id)];

		app.update_header_title();
		(app, Task::batch(tasks))
	}

	fn header_start(&self) -> Vec<Element<Self::Message>> {
		vec![self.menu_bar()]
	}

	fn view(&self) -> Element<Self::Message> {
		self.state.view(&self.flags)
	}

	fn view_window(&self, id: Id) -> Element<Self::Message> {
		self.dialog.view_window(id)
	}

	fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
		self.update_header_title();

		if let Some(task) = self.dialog.update(&message) {
			return task;
		}

		self.state.update(&mut self.flags, message)
	}
}

impl AstroMark {
	fn update_header_title(&mut self) {
		self.set_header_title(format!("{} - {}", trans!("astromark"), self.state));
	}
}

impl AstroMark {
	fn menu_bar<'thing>(&self) -> Element<'thing, Message> {
		let keybinds: HashMap<menu::KeyBind, MenuActions> = HashMap::new();

		let mut file_menu = vec![];
		if let State::Editor(_) = self.state {
			file_menu.append(&mut vec![
				Item::Button(trans!("save"), None, MenuActions::Save),
				Item::Button(trans!("save_as"), None, MenuActions::SaveAs),
				Item::Divider,
			]);
		}
		file_menu.append(&mut vec![
			Item::Button(trans!("open_file"), None, MenuActions::OpenFile),
			Item::Button(trans!("new_file"), None, MenuActions::NewFile),
			Item::Divider,
			Item::Button(trans!("go_home"), None, MenuActions::GoHome),
		]);

		menu::bar(vec![menu::Tree::with_children(
			menu::root(trans!("file")),
			menu::items(&keybinds, file_menu),
		)])
		.item_height(ItemHeight::Dynamic(40))
		.into()
	}
}
