use std::path::PathBuf;

use rhai::{AST, Engine, FnPtr, FuncArgs};

use crate::utils::{AppResult, ok_or_default};

use super::{DefaultBytes, flags::Flags};

pub mod color;
pub mod engine;
pub mod keybinds;

const DEFAULT_CFG: &str = "\
let flags = flags();

flags
";

#[allow(unused)]
pub struct ScriptCfg {
	engine: Engine,
	ast: AST,
	pub flags: Flags,
}

impl ScriptCfg {
	pub fn read(path: &PathBuf) -> Self {
		let engine = engine::engine();
		let ast = ok_or_default(engine.compile_file(path.into()));
		let flags = ok_or_default(engine.eval_ast::<Flags>(&ast));

		Self { engine, ast, flags }
	}

	pub fn call_rhai_fn<T>(&self, fnptr: FnPtr, args: impl FuncArgs) -> AppResult<T>
	where
		T: Send + Sync + Clone + 'static,
	{
		Ok(fnptr.call(&self.engine, &self.ast, args)?)
	}
}

impl DefaultBytes for ScriptCfg {
	fn default_bytes() -> impl AsRef<[u8]> {
		DEFAULT_CFG
	}
}
