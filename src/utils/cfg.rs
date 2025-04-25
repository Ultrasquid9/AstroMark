use script::ScriptCfg;
use serde::de::DeserializeOwned;
use std::{
	fs,
	path::{Path, PathBuf},
};
use tracing::{error, info};

use super::dir_exists_or_run;

pub mod flags;
pub mod recent;
pub mod script;

pub trait DefaultStr {
	fn default_str() -> String;
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

pub fn get_or_create_cfg_file<Dir>(name: Dir) -> PathBuf
where
	Dir: AsRef<Path>,
{
	let mut dir = get_or_create_cfg_dir();

	dir.push(name);
	dir_exists_or_run(&dir, |pat| {
		info!("No config file detected, creating one now...");
		fs::write(pat, ScriptCfg::default_str())
	});

	dir
}

pub fn deserialize_or_default<Dir, Cfg>(path: Dir) -> Cfg
where
	Dir: AsRef<Path>,
	Cfg: DeserializeOwned + Default,
{
	match fs::read_to_string(path) {
		Ok(str) => match ron::from_str(&str) {
			Ok(flags) => flags,
			Err(e) => {
				error!("Error deserializing file: {e}");
				Cfg::default()
			}
		},
		Err(e) => {
			error!("Error reading file: {e}");
			Cfg::default()
		}
	}
}
