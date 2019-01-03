// wengwengweng

use clap::{App, AppSettings, Arg};

mod bundle;
mod utils;

use crate::bundle::*;

fn main() {

	let matches = App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.setting(AppSettings::ColoredHelp)

		.arg(Arg::with_name("BIN")
			.takes_value(true)
			.required(true)
			.help("the binary to pack"))

		.arg(Arg::with_name("NAME")
			.long("name")
			.takes_value(true)
			.requires("BIN"))

		.arg(Arg::with_name("DNAME")
			.long("display-name")
			.takes_value(true)
			.requires("BIN"))

		.arg(Arg::with_name("IDENT")
			.long("identifier")
			.takes_value(true)
			.requires("BIN"))

		.arg(Arg::with_name("VERSION")
			.long("version")
			.takes_value(true)
			.requires("BIN"))

		.arg(Arg::with_name("ICON")
			.long("icon")
			.takes_value(true)
			.requires("BIN"))

		.get_matches();

	let bin = matches.value_of("BIN").unwrap().to_owned();
	let mut bundle = Bundle::new(&format!("{}.app", &utils::basename(&bin)));

	bundle.set_bin(&bin);
	bundle.set_name(matches.value_of("NAME").unwrap_or(""));
	bundle.set_display_name(matches.value_of("DNAME").unwrap_or(""));
	bundle.set_identifier(matches.value_of("IDENT").unwrap_or(""));
	bundle.set_version(matches.value_of("VERSION").unwrap_or(""));

	if let Some(icon) = matches.value_of("ICON") {
		bundle.set_icon(icon);
	}

	bundle.write();

}

