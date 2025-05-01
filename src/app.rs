use ahash::HashMap;

use cosmic::{
	Application, ApplicationExt, Core, Element,
	app::Task,
	executor,
	iced::{
		Subscription,
		event::{Event, Status, listen_with},
		window::Id,
	},
	iced_core::keyboard,
	iced_widget::column,
	widget::{
		menu::{self, Item, ItemHeight},
		segmented_button::{Entity, Model, SingleSelect},
		tab_bar,
	},
};
use dialog::DialogManager;
use message::{MenuActions, Message, task};
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

		app.set_header_title(trans!("astromark"));
		app.add_tab(State::new());

		let Some(id) = app.core.main_window_id() else {
			error!("App window ID not found!");
			panic!()
		};

		let tasks = [app.set_window_title(trans!("astromark"), id)];

		(app, Task::batch(tasks))
	}

	fn header_start(&self) -> Vec<Element<Self::Message>> {
		vec![self.menu_bar()]
	}

	fn view_window(&self, id: Id) -> Element<Self::Message> {
		self.dialog.view_window(id)
	}

	fn subscription(&self) -> Subscription<Self::Message> {
		let subscriptions = vec![listen_with(|event, status, _id| match event {
			Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })
				if status != Status::Captured =>
			{
				Some(Message::KeyPress(key, modifiers))
			}
			_ => None,
		})];

		Subscription::batch(subscriptions)
	}

	fn view(&self) -> Element<Self::Message> {
		let Some(state) = self.tabs.get(&self.model.active()) else {
			return column![].into();
		};

		let mut children = vec![state.view(&self.flags)];

		if self.tabs.len() > 1 {
			children.insert(
				0,
				tab_bar::horizontal(&self.model)
					.on_activate(Message::SwitchToTab)
					.on_close(Message::KillTab)
					.on_middle_press(Message::KillTab)
					.into(),
			);
		}

		cosmic::widget::column::with_children(children).into()
	}

	fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
		self.update_tabs(&message);

		macro_rules! return_if_some {
			( $( $opt:expr; )+ ) => { $(
				if let Some(task) = $opt {
					return task;
				}
			)+ };
		}

		return_if_some![
			self.dialog.update(&message);
			self.keybinds(&message);
		];

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
		match message {
			Message::SwitchToTab(id) => {
				self.model.activate(*id);
				return;
			}
			Message::KillTab(id) => {
				// If the currently active tab is the one being closed, switch to a different one
				if self.model.active() == *id {
					// Ensuring that you are being sent to a tab that isn't being closed
					let to_activate = match self.model.position(*id) {
						Some(0) => 1,
						_ => 0,
					};
					self.model.activate_position(to_activate);
				}

				self.tabs.remove(id);
				self.model.remove(*id);
				return;
			}
			_ => (),
		}

		if let Some(new) = State::from_message(&self.flags.flags, message) {
			if matches!(self.tabs.get(&self.model.active()), Some(state) if state.can_overwrite()) {
				let id = self.model.active();

				self.tabs.remove(&id);
				self.model.remove(id);
			}
			self.add_tab(new);
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

	fn menu_bar<'thing>(&self) -> Element<'thing, Message> {
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
			menu::items(&self.flags.flags.general_keybinds(), file_menu),
		)])
		.item_height(ItemHeight::Dynamic(40))
		.into()
	}

	fn keybinds(&self, message: &Message) -> Option<Task<Message>> {
		if let Message::KeyPress(key, modifiers) = message {
			for (keybind, action) in self.flags.flags.general_keybinds() {
				if keybind.matches(*modifiers, key) {
					return Some(task(action.into()));
				}
			}
		}
		None
	}
}
