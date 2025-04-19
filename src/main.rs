use std::error::Error;

use cosmic::app::{Settings, run};

mod app;
mod init;

pub type AppResult<Ok> = Result<Ok, Box<dyn Error + Send + Sync>>;

fn main() -> AppResult<()> {
	init::log()?;
	let args = init::args();

	let settings = Settings::default();
	let flags = init::flags(&args);

	run::<app::AstroMark>(settings, flags)?;
	Ok(())
}
