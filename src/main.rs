// wengwengweng

use std::fs;
use clap::App;

fn main() {

	App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.get_matches();

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

	fs::create_dir(bundle_dir).expect("failed to create dir");
	fs::create_dir(contents_dir).expect("failed to create dir");
	fs::create_dir(macos_dir).expect("failed to create dir");
	fs::create_dir(resources_dir).expect("failed to create dir");
	fs::copy(name, bin_path).expect("failed to copy bin");
	fs::write(plist_path, plist).expect("unable to write plist");

}

