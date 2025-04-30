use cosmic::iced::highlighter::Theme;
use rhai::{CustomType, FnPtr, TypeBuilder};
use tracing::warn;

use crate::utils::ok_or_default;

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
	/// Function ran when starting the app  
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
		}
	}
}
