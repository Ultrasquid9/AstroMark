use std::path::PathBuf;

use clap::{ArgMatches, Command, arg, value_parser};
use tracing::Level;

use super::{cfg::{get_or_create_cfg_file, script::ScriptCfg}, AppResult};

pub fn args() -> ArgMatches {
	Command::new("AstroMark")
		.version("0")
		.about("Super simple graphical markdown editor")
		.arg(
			arg!(-c --config <FILE> "Use a custom config file")
				.value_parser(value_parser!(PathBuf)),
		)
		// TODO: --default_config option
		//.arg(arg!(-r --reset_config ... "Reset the config file").action(ArgAction::SetTrue))
		.get_matches()
}

pub fn cfg(args: &ArgMatches) -> ScriptCfg {
	let dir = match args.get_one::<PathBuf>("config") {
		Some(dir) => dir,
		None => &get_or_create_cfg_file("config.rhai"),
	};

	ScriptCfg::read(dir)
}

pub fn log() -> AppResult<()> {
	let subscriber = tracing_subscriber::FmtSubscriber::builder()
		.with_max_level(Level::INFO)
		.finish();

	tracing::subscriber::set_global_default(subscriber)?;
	Ok(())
}
