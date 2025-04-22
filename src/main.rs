use cosmic::app::{Settings, run};

use utils::init;

mod app;
mod trans;
mod utils;

fn main() -> utils::AppResult<()> {
	trans::init()?;
	init::log()?;
	let args = init::args();

	let settings = Settings::default();
	let flags = init::flags(&args);

	run::<app::AstroMark>(settings, flags)?;
	Ok(())
}
