use rhai::{Engine, exported_module, module_resolvers::FileModuleResolver};

use crate::{
	app::message::{MenuActions, menu_actions},
	utils::cfg::{flags::Flags, get_or_create_cfg_dir},
};

use super::keybinds::{Key, Keybind, modifiers};

pub fn engine() -> Engine {
	let mut engine = Engine::new();

	remove_features(&mut engine);
	register_types(&mut engine);
	misc(&mut engine);

	engine
}

fn remove_features(engine: &mut Engine) {
	engine
		// Eval
		// Recommended by the Rhai docs
		.disable_symbol("eval")
		// Exceptions
		// I don't like them
		.disable_symbol("throw")
		.disable_symbol("try")
		.disable_symbol("catch");
}

fn register_types(engine: &mut Engine) {
	engine
		// Flags
		// Store the configuration
		.build_type::<Flags>()
		.register_fn("flags", Flags::default)
		// Key
		// Used for keybinds
		.build_type::<Key>()
		.register_fn("key", Key::new)
		// Keybinds
		.build_type::<Keybind>()
		.register_fn("keybind", Keybind::new)
		.register_static_module("Modifier", exported_module!(modifiers).into())
		// Menu Actions
		// Used for "general" keybinds
		.register_type_with_name::<MenuActions>("Action")
		.register_static_module("Action", exported_module!(menu_actions).into());
}

fn misc(engine: &mut Engine) {
	engine
		// Require variables to be defined during compilation
		.set_strict_variables(true)
		// Use the config directory for modules
		.set_module_resolver(FileModuleResolver::new_with_path(get_or_create_cfg_dir()));
}
