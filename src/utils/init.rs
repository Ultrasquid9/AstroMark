use std::{fs, path::PathBuf};

use clap::{ArgAction, ArgMatches, Command, arg, value_parser};
use tracing::{Level, error, info};

use crate::app::flags::Flags;

use super::{AppResult, cfg::get_or_create_cfg_file};

pub fn args() -> ArgMatches {
	Command::new("AstroMark")
		.version("0")
		.about("Super simple graphical markdown editor")
		.arg(
			arg!(-c --config <FILE> "Use a custom config file")
				.value_parser(value_parser!(PathBuf)),
		)
		.arg(arg!(-r --reset_config ... "Reset the config file").action(ArgAction::SetTrue))
		.get_matches()
}

pub fn flags(args: &ArgMatches) -> Flags {
	if args.get_flag("reset_config") {
		info!("Resetting config!");
		if let Err(e) = fs::write(get_or_create_cfg_file(), Flags::default_str()) {
			error!("{e}")
		}
	}

	let dir = match args.get_one::<PathBuf>("config") {
		Some(dir) => dir,
		None => &get_or_create_cfg_file(),
	};

	Flags::read(dir)
}

pub fn log() -> AppResult<()> {
	let subscriber = tracing_subscriber::FmtSubscriber::builder()
		.with_max_level(Level::INFO)
		.finish();

	tracing::subscriber::set_global_default(subscriber)?;
	Ok(())
}
