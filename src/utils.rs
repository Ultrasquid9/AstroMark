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
