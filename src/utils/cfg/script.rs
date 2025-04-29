use std::path::PathBuf;

use rhai::Engine;
use tracing::error;

use super::{DefaultBytes, flags::Flags};

pub mod engine;

#[allow(unused)]
pub struct ScriptCfg {
	engine: Engine,
	pub flags: Flags,
}

impl ScriptCfg {
	pub fn read(path: &PathBuf) -> Self {
		let engine = engine::engine();

		let flags = match engine.eval_file::<Flags>(path.into()) {
			Ok(ok) => ok,
			Err(e) => {
				error!("{e}");
				Flags::default()
			}
		};

		Self { engine, flags }
	}
}

impl DefaultBytes for ScriptCfg {
	fn default_bytes() -> impl AsRef<[u8]> {
		// TODO: "Default Config" file
		"flags()"
	}
}
