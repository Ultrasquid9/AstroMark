use std::{
	error::Error,
	fs,
	path::{Path, PathBuf},
};

use clap::{ArgAction, ArgMatches, Command, value_parser};
use tracing::{Level, error, info};

use crate::{AppResult, app::flags::Flags};

pub fn args() -> ArgMatches {
	Command::new("EstroMD")
		.version("0")
		.about("Super simple markdown editor")
		.arg(
			clap::arg!(-c --config <FILE> "Use a custom config file")
				//.required(false)
				.value_parser(value_parser!(PathBuf)),
		)
		.arg(clap::arg!(-r --reset_config ... "Reset the config file").action(ArgAction::SetTrue))
		.get_matches()
}

pub fn flags(args: &ArgMatches) -> Flags {
	if args.get_flag("reset_config") {
		info!("Resetting config!");
		if let Err(e) = fs::write(get_or_create_cfg_dir(), Flags::default_str()) {
			error!("{e}")
		}
	}

	let dir = match args.get_one::<PathBuf>("config") {
		Some(dir) => dir,
		None => &get_or_create_cfg_dir(),
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

fn get_or_create_cfg_dir() -> PathBuf {
	let Some(mut dir) = dirs::config_dir() else {
		error!("Config dir could not be found");
		panic!()
	};

	dir.push("estro_md");
	dir_exists_or_run(&dir, fs::create_dir_all);

	dir.push("config.ron");
	dir_exists_or_run(&dir, |pat| {
		info!("No config file detected, creating one now...");
		fs::write(pat, Flags::default_str())
	});

	dir
}

/// Checks if a directory exists. If it does not, runs the provided function and handles any errors.
fn dir_exists_or_run<Pat, Fun, Err, Idk>(dir: Pat, fun: Fun)
where
	Pat: AsRef<Path>,
	Fun: Fn(Pat) -> Result<Idk, Err>,
	Err: Error,
{
	// TODO: Replace with let-chain once stabilized
	if let Ok(exists) = fs::exists(&dir) {
		if !exists {
			if let Err(e) = fun(dir) {
				error!("{e}")
			}
		}
	}
}
