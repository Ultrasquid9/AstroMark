use serde::de::DeserializeOwned;
use std::{
	fs,
	path::{Path, PathBuf},
};
use tracing::{error, info, warn};

use super::dir_exists_or_run;

pub mod flags;
pub mod recent;
pub mod script;

pub trait DefaultBytes {
	fn default_bytes() -> impl AsRef<[u8]>;
}

pub fn get_or_create_cfg_dir() -> PathBuf {
	let Some(mut dir) = dirs::config_dir() else {
		error!("Config dir could not be found");
		panic!()
	};

	dir.push("astromark");
	dir_exists_or_run(&dir, fs::create_dir_all);

	dir
}

pub fn get_or_create_cfg_file<Dir, Cfg>(name: Dir) -> PathBuf
where
	Dir: AsRef<Path>,
	Cfg: DefaultBytes
{
	let mut dir = get_or_create_cfg_dir();

	dir.push(name);
	dir_exists_or_run(&dir, |pat| {
		info!("File {:?} not found, creating it now...", pat);
		fs::write(pat, Cfg::default_bytes())
	});

	dir
}

pub fn deserialize_or_default<Dir, Cfg>(path: Dir) -> Cfg
where
	Dir: AsRef<Path>,
	Cfg: DeserializeOwned + Default,
{
	match fs::read(path) {
		Ok(bytes) => match bincode::deserialize(&bytes) {
			Ok(flags) => flags,
			Err(e) => {
				warn!("Error deserializing file: {e}");
				Cfg::default()
			}
		},
		Err(e) => {
			error!("Error reading file: {e}");
			Cfg::default()
		}
	}
}
