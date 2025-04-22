use std::{fs, path::PathBuf};
use tracing::{error, info};

use crate::app::flags::Flags;

use super::dir_exists_or_run;

pub fn get_or_create_cfg_dir() -> PathBuf {
	let Some(mut dir) = dirs::config_dir() else {
		error!("Config dir could not be found");
		panic!()
	};

	dir.push("astromark");
	dir_exists_or_run(&dir, fs::create_dir_all);

	dir
}

pub fn get_or_create_cfg_file() -> PathBuf {
	let mut dir = get_or_create_cfg_dir();

	dir.push("config.ron");
	dir_exists_or_run(&dir, |pat| {
		info!("No config file detected, creating one now...");
		fs::write(pat, Flags::default_str())
	});

	dir
}
