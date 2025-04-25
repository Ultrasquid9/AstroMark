use std::path::PathBuf;

use rhai::{Engine, module_resolvers::FileModuleResolver};
use tracing::error;

use super::{DefaultBytes, flags::Flags, get_or_create_cfg_dir};

#[allow(unused)]
pub struct ScriptCfg {
	engine: Engine,
	pub flags: Flags,
}

impl ScriptCfg {
	pub fn read(path: &PathBuf) -> Self {
		let engine = engine();

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
		"flags()"
	}
}

fn engine() -> Engine {
	let mut engine = Engine::new();

	engine
		.disable_symbol("eval")
		.disable_symbol("throw")
		.disable_symbol("try")
		.disable_symbol("catch")
		.build_type::<Flags>()
		.register_fn("flags", Flags::default)
		.set_strict_variables(true)
		.set_module_resolver(FileModuleResolver::new_with_path(get_or_create_cfg_dir()));

	engine
}
