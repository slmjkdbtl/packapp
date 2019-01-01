// wengwengweng

use clap::{App, AppSettings, Arg, Error, ErrorKind};

mod bundle;
mod utils;

use crate::bundle::Bundle;

fn main() {

	let matches = App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.setting(AppSettings::ColoredHelp)
		.setting(AppSettings::TrailingVarArg)
		.arg(Arg::with_name("BIN")
			.takes_value(true)
			.required(true)
			.help("the binary to pack"))
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

	let name = matches.value_of("BIN").unwrap().to_string();
	let bundle = Bundle::new(format!("{}.app", name));

	bundle.add_bin(&name);

}

