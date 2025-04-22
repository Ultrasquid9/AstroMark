use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::deserialize_or_default;

#[derive(Serialize, Deserialize, Default)]
pub struct Recent(Vec<PathBuf>);

impl Recent {
	pub fn read(path: PathBuf) -> Self {
		deserialize_or_default(path)
	}

	pub fn add(&mut self, path: PathBuf) {
		self.0.retain(|p| *p != path);
		self.0.push(path);

		while self.0.len() > 10 {
			_ = self.0.remove(0)
		}
	}
}
