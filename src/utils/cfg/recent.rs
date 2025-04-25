use std::{path::PathBuf, vec::IntoIter};

use serde::{Deserialize, Serialize};
use tracing::error;

use super::{deserialize_or_default, get_or_create_cfg_file};

pub const DIR: &str = "recent.ron";
const COMMENT: &str = "\
// This stores recently opened files
// Please avoid editing it manually, or putting it in your dotfiles
";

#[derive(Serialize, Deserialize, Default)]
pub struct Recent(Vec<PathBuf>);

impl Recent {
	pub fn read(path: PathBuf) -> Self {
		deserialize_or_default(path)
	}

	pub fn add(&mut self, path: PathBuf) {
		self.0.retain(|p| *p != path);
		self.0.push(path);

		// TODO: Use flags to control amount
		while self.0.len() > 8 {
			_ = self.0.remove(0)
		}
	}

	pub fn write(&self) {
		let dir = get_or_create_cfg_file(DIR);

		let str = match ron::to_string(&self) {
			Ok(ok) => ok,
			Err(e) => {
				error!("{e}");
				return;
			}
		};

		if let Err(e) = std::fs::write(dir, COMMENT.to_string() + str.as_str()) {
			error!("{e}");
		}
	}

	pub fn get_inner(&self) -> &[PathBuf] {
		&self.0
	}
}

impl IntoIterator for Recent {
	type Item = PathBuf;
	type IntoIter = IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}
