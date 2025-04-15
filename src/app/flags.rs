use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Flags {
	pub text_size: f32,
}

impl Flags {
	pub fn new() -> Self {
		Self::default()
	}
}

impl Default for Flags {
	fn default() -> Self {
		Self { text_size: 14. }
	}
}
