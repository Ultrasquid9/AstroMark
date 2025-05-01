use cosmic::{
	iced_core::keyboard,
	widget::menu::{KeyBind, key_bind::Modifier},
};
use rhai::{Array, CustomType, Module, TypeBuilder, export_module};
use smol_str::SmolStr;
use tracing::warn;

#[derive(PartialEq, Clone, CustomType)]
pub struct Key(pub SmolStr);

#[derive(Clone, CustomType)]
pub struct Keybind {
	pub key: Key,
	pub modifiers: Vec<Key>,
}

impl Key {
	pub fn new(str: String) -> Self {
		Self(str.into())
	}
}

impl Keybind {
	pub fn new(key: Key, modifiers: Array) -> Self {
		let modifiers = modifiers
			.iter()
			.filter_map(|item| item.clone().try_cast())
			.collect();

		Self { key, modifiers }
	}
}

impl From<Key> for keyboard::Key {
	fn from(key: Key) -> Self {
		// TODO: handle some "Named" variants
		keyboard::Key::Character(key.0)
	}
}

impl From<Key> for Option<Modifier> {
	fn from(key: Key) -> Self {
		if key == modifiers::SUPER {
			Some(Modifier::Super)
		} else if key == modifiers::CTRL {
			Some(Modifier::Ctrl)
		} else if key == modifiers::ALT {
			Some(Modifier::Alt)
		} else if key == modifiers::SHIFT {
			Some(Modifier::Shift)
		} else {
			warn!("{} is not a modifier!", key.0);
			None
		}
	}
}

impl From<Keybind> for KeyBind {
	fn from(bind: Keybind) -> Self {
		Self {
			key: bind.key.into(),
			modifiers: bind
				.modifiers
				.iter()
				.filter_map(|item| item.clone().into())
				.collect(),
		}
	}
}

#[export_module]
pub mod modifiers {
	use super::Key;

	const fn key(str: &'static str) -> Key {
		Key(smol_str::SmolStr::new_static(str))
	}

	pub const SUPER: Key = key("Super");
	pub const CTRL: Key = key("Ctrl");
	pub const ALT: Key = key("Alt");
	pub const SHIFT: Key = key("Shift");
}
