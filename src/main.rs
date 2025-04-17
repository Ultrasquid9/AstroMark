use std::error::Error;

use app::App;
use cosmic::app::Settings;

mod app;
mod init;

pub type AppResult<Ok> = Result<Ok, Box<dyn Error + Send + Sync>>;

fn main() -> AppResult<()> {
	init::log()?;
	let args = init::args();

	let settings = Settings::default();
	let flags = init::flags(&args);

	cosmic::app::run::<App>(settings, flags)?;
	Ok(())
}
