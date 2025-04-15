use std::{fs::read_to_string, path::PathBuf};

use clap::{Command, value_parser};
use tracing::{Level, info};

use crate::{AppResult, app::flags::Flags};

pub fn args() -> AppResult<Flags> {
	let args = Command::new("EstroMD")
		.version("0")
		.about("Super simple markdown editor")
		.arg(
			clap::arg!(-c --config <FILE> "Use a custom config file")
				.required(false)
				.value_parser(value_parser!(PathBuf)),
		)
		.get_matches();

	if let Some(path) = args.get_one::<PathBuf>("config") {
		info!("Custom config: {:?}", path);
		Ok(ron::from_str(&read_to_string(path)?)?)
	} else {
		info!("Using default config");
		Ok(Flags::new())
	}
}

pub fn log() -> AppResult<()> {
	let subscriber = tracing_subscriber::FmtSubscriber::builder()
		.with_max_level(Level::INFO)
		.finish();

	tracing::subscriber::set_global_default(subscriber)?;
	Ok(())
}
