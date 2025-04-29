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
	let flags = init::cfg(&args);

	// Testing the Rhai callback system
	match flags.call_rhai_fn::<()>(flags.flags.callback.clone(), ()) {
		Err(e) => tracing::error!("{e}"),
		_ => (),
	}

	run::<app::AstroMark>(settings, flags)?;
	Ok(())
}
