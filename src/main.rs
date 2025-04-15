use std::error::Error;

use app::{App, flags::Flags};
use cosmic::app::Settings;

mod app;

pub type AppResult<Ok> = Result<Ok, Box<dyn Error + Send + Sync>>;

fn main() -> AppResult<()> {
	let settings = Settings::default();
	let flags = Flags::new();

	cosmic::app::run::<App>(settings, flags)?;
	Ok(())
}
