use std::error::Error;

use app::{App, flags::Flags};
use cosmic::app::Settings;
use tracing::{Level, info};

mod app;

pub type AppResult<Ok> = Result<Ok, Box<dyn Error + Send + Sync>>;

fn main() -> AppResult<()> {
	init_log();

	let settings = Settings::default();
	let flags = Flags::new();

	cosmic::app::run::<App>(settings, flags)?;
	Ok(())
}

fn init_log() {
	let subscriber = tracing_subscriber::FmtSubscriber::builder()
		.with_max_level(Level::INFO)
		.finish();

	tracing::subscriber::set_global_default(subscriber).expect("Setting subscriber failed");
	info!("Logger initialized");
}
