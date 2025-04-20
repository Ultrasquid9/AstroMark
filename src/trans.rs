use std::sync::OnceLock;

use i18n_embed::{
	DesktopLanguageRequester, LanguageLoader,
	fluent::{FluentLanguageLoader, fluent_language_loader},
};

use crate::AppResult;

pub static LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

#[derive(rust_embed::RustEmbed)]
#[folder = "translations/"]
struct Translations;

pub fn init() -> AppResult<()> {
	let loader = fluent_language_loader!();
	let requested = DesktopLanguageRequester::requested_languages();
	let ids = i18n_embed::select(&loader, &Translations, &requested)?;

	loader.load_languages(&Translations, &ids)?;
	LOADER.get_or_init(|| loader);

	Ok(())
}

#[macro_export]
macro_rules! trans {
	($message_id:literal) => {{
		i18n_embed_fl::fl!($crate::trans::LOADER.get().unwrap(), $message_id)
	}};

	($message_id:literal, $($args:expr),*) => {{
		i18n_embed_fl::fl!($crate::trans::LOADER.get().unwrap(), $message_id, $($args), *)
	}};
}
