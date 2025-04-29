use cosmic::iced::highlighter::Theme;
use rhai::{CustomType, FnPtr, TypeBuilder};
use tracing::warn;

#[derive(Clone, CustomType)]
pub struct Flags {
	pub text_size: f32,
	pub tab_len: usize,
	pub expand_tabs: bool,
	pub highlight: String,
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
}

impl Default for Flags {
	fn default() -> Self {
		Self {
			text_size: 14.,
			tab_len: 4,
			expand_tabs: false,
			highlight: "base16eighties".into(),
			callback: FnPtr::new("callback").unwrap(),
		}
	}
}
