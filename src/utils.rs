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

/// Creates a module and automatically exports it to Rhai. Includes 
/// a higher-level API for creating constants for Enum variants, and 
/// a lower-level API for creating modules manually. 
/// 
/// # Examples
/// Using the Enum API:
/// ```
/// pub enum MyEnum {
/// 	Variant1,
/// 	Variant2,
/// 	Variant3,
/// }
/// 
/// create_rhai_mod! {
/// 	my_enum(MyEnum) => [
/// 		Variant1,
/// 		Variant2,
/// 		Variant3,
/// 	]
/// }
/// ```
/// Using the lower-level API: 
/// ``` 
/// create_rhai_mod! {
/// 	my_mod {
/// 		// Functions/Constants go here
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! create_rhai_mod {
	( $mod:ident ( $type:ty ) => [ $( $variant:ident; )+ ] ) => {
		create_rhai_mod! { $mod { $(
			#[allow(non_upper_case_globals)]
			pub const $variant: $type = <$type>::$variant;
		)* } }
	};
	( $mod:ident $in:tt ) => {
		#[allow(unused_imports)]
		use rhai::plugin::*;

		#[export_module]
		pub mod $mod $in
	}
}

/// Pipes the first expression into any others provided.
/// 
/// # Examples
/// ```
/// # fn add_one(i: i32) -> i32 { i + 1 }
/// # fn divide_by_two(i: i32) -> i32 { i / 2 }
/// let string = pipe! {
/// 	my_var: "123";
/// 	|> my_var.parse::<i32>();
/// 	|> add_one(my_var);
/// 	|> divide_by_two(my_var);
/// 	|> my_var.to_string();
/// }
/// 
/// assert_eq!("62", string);
/// ```
#[macro_export]
macro_rules! pipe {
	( $var:ident : $in1:expr ; $( |> $in2:expr ; )* ) => {{
		let $var = $in1;
		$( let $var = $in2; )*
		$var
	}};
}
