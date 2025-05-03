use std::num::TryFromIntError;

use cosmic::iced::{Color, theme::Palette};
use tracing::warn;

use crate::create_rhai_mod;

pub fn str_to_color(string: String) -> Color {
	Color::parse(&string).unwrap_or_else(|| {
		warn!("\"{string}\" could not be parsed into a color!");
		Color::BLACK
	})
}

pub fn i64_to_color(r: i64, g: i64, b: i64) -> Color {
	fn maybe(maybe: Result<u8, TryFromIntError>) -> u8 {
		maybe.unwrap_or_else(|_| {
			warn!(
				"Invalid int provided: must be between {} and {}",
				u8::MIN,
				u8::MAX
			);
			0
		})
	}

	let r8 = maybe(u8::try_from(r));
	let g8 = maybe(u8::try_from(g));
	let b8 = maybe(u8::try_from(b));

	Color::from_rgb8(r8, g8, b8)
}

pub fn palette(
	background: Color,
	text: Color,
	primary: Color,
	success: Color,
	danger: Color,
) -> Palette {
	Palette {
		background,
		text,
		primary,
		success,
		danger,
	}
}

create_rhai_mod! {
	palettes(Palette) => [
		DRACULA;
		NORD;
		SOLARIZED_LIGHT;
		SOLARIZED_DARK;
		GRUVBOX_LIGHT;
		GRUVBOX_DARK;
		CATPPUCCIN_LATTE;
		CATPPUCCIN_FRAPPE;
		CATPPUCCIN_MACCHIATO;
		CATPPUCCIN_MOCHA;
		TOKYO_NIGHT;
		TOKYO_NIGHT_STORM;
		TOKYO_NIGHT_LIGHT;
		KANAGAWA_WAVE;
		KANAGAWA_DRAGON;
		KANAGAWA_LOTUS;
		MOONFLY;
		NIGHTFLY;
		OXOCARBON;
		FERRA;
	]
}
