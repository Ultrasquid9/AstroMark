use rhai::{CustomType, TypeBuilder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, CustomType)]
pub struct Flags {
	pub text_size: f32,
	pub tab_len: usize,
	pub expand_tabs: bool,
}

impl Flags {
	pub fn space(&self) -> f32 {
		self.text_size * 2.
	}
}

impl Default for Flags {
	fn default() -> Self {
		Self {
			text_size: 14.,
			tab_len: 4,
			expand_tabs: false,
		}
	}
}
