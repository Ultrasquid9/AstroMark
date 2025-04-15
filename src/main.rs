use std::error::Error;

use app::App;
use cosmic::app::Settings;

mod app;
mod init;

pub type AppResult<Ok> = Result<Ok, Box<dyn Error + Send + Sync>>;

fn main() -> AppResult<()> {
	init::log()?;
	let flags = init::args()?;

	cosmic::app::run::<App>(Settings::default(), flags)?;
	Ok(())
}
