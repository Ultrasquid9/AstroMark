use ahash::HashMap;

use cosmic::{
	Application, ApplicationExt, Core, Element,
	app::Task,
	executor,
	iced::window::Id,
	iced_widget::column,
	widget::{
		menu::{self, Item, ItemHeight},
		segmented_button::{Entity, Model, SingleSelect},
		tab_bar,
	},
};
use dialog::DialogManager;
use message::{MenuActions, Message};
use state::State;
use tracing::error;

use crate::{trans, utils::cfg::script::ScriptCfg};

pub mod dialog;
pub mod message;
pub mod state;

pub trait Screen {
	fn view<'flags>(&'flags self, flags: &'flags ScriptCfg) -> Element<'flags, Message>;

	fn update<'flags>(
		&'flags mut self,
		flags: &'flags mut ScriptCfg,
		message: Message,
	) -> Task<Message>;
}

pub struct AstroMark {
	core: Core,
	flags: ScriptCfg,

	model: Model<SingleSelect>,
	tabs: HashMap<Entity, State>,
	dialog: DialogManager,
}

impl Application for AstroMark {
	type Executor = executor::Default;
	type Message = Message;
	type Flags = ScriptCfg;

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

			model: Model::builder().build(),
			tabs: HashMap::default(),
			dialog: DialogManager::new(),
		};

		app.add_tab(State::new());

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
		let Some(state) = self.tabs.get(&self.model.active()) else {
			return column![].into();
		};

		let mut children = vec![state.view(&self.flags)];

		if self.tabs.len() > 1 {
			children.insert(0, tab_bar::horizontal(&self.model).into());
		}

		cosmic::widget::column::with_children(children).into()
	}

	fn view_window(&self, id: Id) -> Element<Self::Message> {
		self.dialog.view_window(id)
	}

	fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
		self.update_tabs(&message);
		self.update_header_title();

		if let Some(task) = self.dialog.update(&message) {
			return task;
		}

		if let Some(state) = self.tabs.get_mut(&self.model.active()) {
			state.update(&mut self.flags, message.clone())
		} else {
			Task::none()
		}
	}
}

impl AstroMark {
	fn add_tab(&mut self, state: State) {
		let tab = self.model.insert().text(state.to_string()).closable().id();

		self.model.activate(tab);
		self.tabs.insert(tab, state);
	}

	fn update_tabs(&mut self, message: &Message) {
		let Some(state) = self.tabs.get_mut(&self.model.active()) else {
			return;
		};

		if let Some(new) = State::from_message(&message) {
			if state.can_overwrite() {
				*state = new;
			} else {
				self.add_tab(new);
			}
		}
	}

	fn current_state(&self) -> &State {
		match self.tabs.get(&self.model.active()) {
			Some(state) => state,
			None => {
				error!("No state found!");
				todo!()
			}
		}
	}

	fn update_header_title(&mut self) {
		self.set_header_title(format!(
			"{} - {}",
			trans!("astromark"),
			self.current_state()
		));
	}

	fn menu_bar<'thing>(&self) -> Element<'thing, Message> {
		use std::collections::HashMap;

		let keybinds: HashMap<menu::KeyBind, MenuActions> = HashMap::new();

		let mut file_menu = vec![];
		if let State::Editor(_) = self.current_state() {
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
