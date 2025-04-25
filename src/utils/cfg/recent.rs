use std::{path::PathBuf, vec::IntoIter};

use serde::{Deserialize, Serialize};
use tracing::error;

use super::{DefaultBytes, deserialize_or_default, get_or_create_cfg_file};

pub const DIR: &str = ".recents";

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
		let dir = get_or_create_cfg_file::<_, Self>(DIR);

		let bytes = match bincode::serialize(&self) {
			Ok(ok) => ok,
			Err(e) => {
				error!("{e}");
				return;
			}
		};

		if let Err(e) = std::fs::write(dir, bytes) {
			error!("{e}");
		}
	}

	pub fn get_inner(&self) -> &[PathBuf] {
		&self.0
	}
}

impl DefaultBytes for Recent {
	fn default_bytes() -> impl AsRef<[u8]> {
		bincode::serialize(&Self::default()).expect("Default should be serializable")
	}
}

impl IntoIterator for Recent {
	type Item = PathBuf;
	type IntoIter = IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}
