use std::{error::Error, fs, path::Path};

use tracing::error;

pub mod cfg;
pub mod init;

pub type AppResult<Ok> = Result<Ok, Box<dyn Error + Send + Sync>>;

/// Checks if a directory exists. If it does not, runs the provided function and handles any errors.
pub fn dir_exists_or_run<Dir, Fun, Err, Idk>(dir: Dir, fun: Fun)
where
	Dir: AsRef<Path>,
	Fun: Fn(Dir) -> Result<Idk, Err>,
	Err: Error,
{
	// TODO: Replace with let-chain once stabilized
	if matches!(fs::exists(&dir), Ok(exists) if !exists) {
		if let Err(e) = fun(dir) {
			error!("{e}")
		}
	}
}

/// Returns the Ok value if present. Otherwise, returns the default.
pub fn ok_or_default<Ok, Err>(opt: Result<Ok, Err>) -> Ok
where
	Ok: Default,
	Err: Error,
{
	match opt {
		Ok(opt) => opt,
		Err(e) => {
			error!("{e}");
			Ok::default()
		}
	}
}

/// Creates a module containing constants for each type variant provided
#[macro_export]
macro_rules! create_rhai_mod {
	( $mod:ident ( $type:ty ) => [ $( $variant:ident; )+ ] ) => {
		use rhai::{export_module, Module};

		#[export_module]
		pub mod $mod {
			$(
				#[allow(non_upper_case_globals)]
				pub const $variant: $type = <$type>::$variant;
			)*
		}
	};
}

/// Pipes the first expression into any others provided
#[macro_export]
macro_rules! pipe {
	( $var:ident : $in1:expr ; $( |> $in2:expr ; )* ) => {{
		let $var = $in1;
		$( let $var = $in2; )*
		$var
	}};
}
