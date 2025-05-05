use cosmic::app::{Settings, run};
use tikv_jemallocator::Jemalloc;
use utils::init;

mod app;
mod trans;
mod utils;

/// Using Jemalloc instead of the default (glibc) allocator
#[global_allocator]
static ALLOC: Jemalloc = Jemalloc;

fn main() -> utils::AppResult<()> {
	trans::init()?;
	init::log()?;
	let args = init::args();

	let settings = Settings::default();
	let flags = init::cfg(&args);

	// Testing the Rhai callback system
	if let Err(e) = flags.call_rhai_fn::<()>(flags.flags.callback.clone(), ()) {
		tracing::error!("{e}");
	}

	run::<app::AstroMark>(settings, flags)?;
	Ok(())
}
