// wengwengweng

use std::fs;
use std::path::Path;
use clap::{App, AppSettings, Arg, Error, ErrorKind};

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

	let bin = matches.value_of("BIN").unwrap();

	require(bin);

	let ident = format!("com.company.{}", bin);
	let ident = matches.value_of("IDENT").unwrap_or(&ident);
	let dname = matches.value_of("DNAME").unwrap_or(bin);
	let version = matches.value_of("VERSION").unwrap_or("0.0.0");

	pack(bin, dname, version, ident);
	println!("created {}.app", bin);

}

fn pack(name: &str, dname: &str, version: &str, ident: &str) {

	let plist = include_str!("template.plist");
	let plist = plist.replace("##name##", name);
	let plist = plist.replace("##dname##", dname);
	let plist = plist.replace("##version##", version);
	let plist = plist.replace("##identifier##", ident);

	let bundle_dir = format!("{}.app", name);
	let contents_dir = format!("{}/Contents", bundle_dir);
	let macos_dir = format!("{}/MacOS", contents_dir);
	let resources_dir = format!("{}/Resources", contents_dir);
	let bin_path = format!("{}/{}", macos_dir, name);
	let plist_path = format!("{}/info.plist", contents_dir);

	mkdir(&bundle_dir);
	mkdir(&contents_dir);
	mkdir(&macos_dir);
	mkdir(&resources_dir);
	copy(name, &bin_path);
	write(&plist_path, &plist);

}

fn fail(msg: &str, kind: ErrorKind) {
	Error::with_description(msg, kind).exit();
}

fn require(path: &str) {
	if !Path::new(path).exists() {
		fail(&format!("\"{}\" not found", path), ErrorKind::Io);
	}
}

fn mkdir(dir: &str) {
	fs::create_dir(dir).unwrap_or_else(|s| {
		fail(&format!("failed to create dir {}", dir), ErrorKind::Io);
	});
}

fn copy(f1: &str, f2: &str) {
	fs::copy(f1, f2).unwrap_or_else(|s| {
		fail(&format!("failed to copy {} to {}", f1, f2), ErrorKind::Io);
		return 0;
	});
}

fn write(file: &str, content: &str) {
	fs::write(file, content).unwrap_or_else(|s| {
		fail(&format!("failed to write to {}", file), ErrorKind::Io);
	});
}

