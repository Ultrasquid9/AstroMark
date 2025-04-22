use ron::ser::PrettyConfig;
use serde::{Serialize, de::DeserializeOwned};
use std::{
	fs,
	path::{Path, PathBuf},
};
use tracing::{error, info};

use flags::Flags;

use super::dir_exists_or_run;

pub mod flags;
pub mod recent;

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
		fs::write(pat, default_str::<Flags>())
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
				error!("Error deserializing config: {e}");
				Cfg::default()
			}
		},
		Err(e) => {
			error!("Error reading config: {e}");
			Cfg::default()
		}
	}
}

pub fn default_str<Cfg>() -> String
where
	Cfg: Serialize + Default,
{
	let cfg = PrettyConfig::default().indentor("	");
	ron::ser::to_string_pretty(&Cfg::default(), cfg).expect("Default should be serializable")
}
