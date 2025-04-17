use std::{
	error::Error,
	fs,
	path::{Path, PathBuf},
};

use clap::{ArgMatches, Command, value_parser};
use tracing::{Level, error};

use crate::{AppResult, app::flags::Flags};

pub fn args() -> ArgMatches {
	Command::new("EstroMD")
		.version("0")
		.about("Super simple markdown editor")
		.arg(
			clap::arg!(-c --config <FILE> "Use a custom config file")
				.required(false)
				.value_parser(value_parser!(PathBuf)),
		)
		.get_matches()
}

pub fn flags(args: &ArgMatches) -> Flags {
	let dir = match args.get_one::<PathBuf>("config") {
		Some(dir) => dir,
		None => &default_cfg_dir(),
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

fn default_cfg_dir() -> PathBuf {
	let mut dir = cfg_dir();
	dir.push("config.ron");

	dir_exists_or_run(&dir, || fs::write(&dir, Flags::default_str()));

	dir
}

fn cfg_dir() -> PathBuf {
	match dirs::config_dir() {
		Some(mut dir) => {
			dir.push("estro_md");
			dir_exists_or_run(&dir, || fs::create_dir_all(&dir));

			dir
		}
		None => {
			error!("Config dir could not be found");
			panic!()
		}
	}
}

/// Checks if a directory exists. If it does not, runs the provided function and handles any errors.
fn dir_exists_or_run<Pat, Fun, Err, Idk>(dir: Pat, fun: Fun)
where
	Pat: AsRef<Path>,
	Fun: Fn() -> Result<Idk, Err>,
	Err: Error,
{
	// TODO: Replace with let-chain once stabilized
	if let Ok(exists) = fs::exists(&dir) {
		if !exists {
			if let Err(e) = fun() {
				error!("{e}")
			}
		}
	}
}
