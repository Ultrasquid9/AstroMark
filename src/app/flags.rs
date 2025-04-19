use std::{fs, path::PathBuf};

use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Serialize, Deserialize)]
pub struct Flags {
	pub text_size: f32,
	pub tab_len: usize,
}

impl Flags {
	pub fn read(path: &PathBuf) -> Self {
		match fs::read_to_string(path) {
			Ok(str) => match ron::from_str(&str) {
				Ok(flags) => flags,
				Err(e) => {
					error!("Error deserializing config: {e}");
					Self::default()
				}
			},
			Err(e) => {
				error!("Error reading config: {e}");
				Self::default()
			}
		}
	}

	pub fn default_str() -> String {
		let cfg = PrettyConfig::default().indentor("	");
		ron::ser::to_string_pretty(&Self::default(), cfg).expect("Default should be serializable")
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
