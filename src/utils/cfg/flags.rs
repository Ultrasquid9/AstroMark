use std::collections::HashMap;

use cosmic::{iced::highlighter::Theme, widget::menu};
use rhai::{Array, CustomType, FnPtr, Map, TypeBuilder};
use tracing::warn;

use crate::{
	app::message::MenuActions,
	utils::{cfg::script::keybinds::Keybind, ok_or_default},
};

#[derive(Clone, CustomType)]
pub struct Flags {
	/// The size of the smallest text
	pub text_size: f32,
	/// How many spaces a tab takes
	/// Only used if "expand_tabs" is enabled
	pub tab_len: i64,
	/// How many recently accessed files should be shown on the home screen
	pub max_recents: i64,
	/// Converts tabs into spaces
	/// Strongly discouraged
	pub expand_tabs: bool,
	/// The highlighting theme used by the editor
	pub highlight: String,
	/// Keybinds used throughout the app
	pub general_keybinds: Array,
	/// Function ran when starting the app
	/// Mostly useful for debugging purposes
	pub callback: FnPtr,
}

impl Flags {
	pub fn space(&self) -> f32 {
		self.text_size * 2.
	}

	pub fn highlight(&self) -> Theme {
		match self.highlight.to_lowercase().trim() {
			"base16eighties" => Theme::Base16Eighties,
			"base16mocha" => Theme::Base16Mocha,
			"base16ocean" => Theme::Base16Ocean,
			"inspiredgithub" => Theme::InspiredGitHub,
			"solarizeddark" => Theme::SolarizedDark,

			unknown => {
				warn!("Highlight {unknown} not found");
				Theme::Base16Eighties
			}
		}
	}

	pub fn tab_len(&self) -> usize {
		ok_or_default(usize::try_from(self.tab_len))
	}

	pub fn max_recents(&self) -> usize {
		ok_or_default(usize::try_from(self.max_recents))
	}

	pub fn general_keybinds(&self) -> HashMap<menu::KeyBind, MenuActions> {
		let mut keybinds = HashMap::new();

		let map_array = self
			.general_keybinds
			.iter()
			.filter_map(|item| match item.as_map_ref() {
				Ok(map) => Some(map.clone()),
				Err(e) => {
					warn!("Type {e} is not a map!");
					None
				}
			})
			.collect::<Vec<Map>>();

		for map in map_array {
			macro_rules! maybe {
				($in:expr ; $err:literal) => {
					match $in {
						Some(out) => out,
						None => {
							warn!($err);
							continue;
						}
					}
				};
			}

			let dyn_action = maybe!(map.get("action"); "No \"action\" field found");
			let dyn_keybind = maybe!(map.get("keybind"); "No \"keybind\" field found");

			let action = maybe!(dyn_action.clone().try_cast::<MenuActions>(); "\"action\" could not be cast to MenuAction");
			let keybind = maybe!(dyn_keybind.clone().try_cast::<Keybind>(); "\"keybind\" could not be cast to Keybind");

			keybinds.insert(keybind.into(), action);
		}

		keybinds
	}
}

impl Default for Flags {
	fn default() -> Self {
		Self {
			text_size: 14.,
			tab_len: 4,
			expand_tabs: false,
			max_recents: 8,
			highlight: "base16eighties".into(),
			callback: FnPtr::new("callback").unwrap(),
			general_keybinds: vec![],
		}
	}
}
