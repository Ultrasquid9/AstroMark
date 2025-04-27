use cosmic::iced::highlighter::Theme;
use rhai::{CustomType, TypeBuilder};

#[derive(Clone, CustomType)]
pub struct Flags {
	pub text_size: f32,
	pub tab_len: usize,
	pub expand_tabs: bool,
	pub highlight: String,
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
			_ => Theme::Base16Eighties,
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
		}
	}
}
