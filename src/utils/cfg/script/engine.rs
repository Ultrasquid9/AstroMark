use rhai::{Engine, module_resolvers::FileModuleResolver};

use crate::utils::cfg::{flags::Flags, get_or_create_cfg_dir};

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
		.build_type::<Flags>()
		.register_fn("flags", Flags::default);
}

fn misc(engine: &mut Engine) {
	engine
		// Require variables to be defined during compilation
		.set_strict_variables(true)
		// Use the config directory for modules
		.set_module_resolver(FileModuleResolver::new_with_path(get_or_create_cfg_dir()));
}
