use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::utils::cfg::deserialize_or_default;

#[derive(Serialize, Deserialize)]
pub struct Flags {
	pub text_size: f32,
	pub tab_len: usize,
}

impl Flags {
	pub fn read(path: &PathBuf) -> Self {
		deserialize_or_default(path)
	}
}

impl Default for Flags {
	fn default() -> Self {
		Self {
			text_size: 14.,
			tab_len: 4,
		}
	}
}
