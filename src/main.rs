// wengwengweng

use std::path::PathBuf;
use argh::FromArgs;

use packapp::*;

#[derive(FromArgs)]
/// pack a binary to a MacOS .app bundle
struct Opt {

	#[argh(positional)]
	bin: PathBuf,

	#[argh(option, short = 'r')]
	/// stuff to copy into the "Resources" folder
	resources: Vec<PathBuf>,

	#[argh(option, short = 'f')]
	/// stuff to copy into the "Frameworks" folder
	frameworks: Vec<PathBuf>,

	#[argh(option, short = 'c')]
	/// icon file
	icon: Option<PathBuf>,

	#[argh(option, short = 'i')]
	/// app identifier
	identifier: Option<String>,

	#[argh(option, short = 'n')]
	/// app name
	name: Option<String>,

	#[argh(option, short = 'd')]
	/// app display name
	display_name: Option<String>,

	#[argh(option, short = 'v')]
	/// app version
	version: Option<String>,

	#[argh(option, short = 't')]
	/// file types that can be opened with this bundle
	types: Vec<String>,

	#[argh(option, short = 'o')]
	/// output path
	out: Option<PathBuf>,

	#[argh(switch)]
	/// verbose output
	verbose: bool,

}

fn pack() -> Result<(), Error> {

    let opt = argh::from_env::<Opt>();
	let mut bundle = Bundle::new(opt.bin)?;

	if let Some(out) = &opt.out {
		bundle.set_output(out);
	}

	if let Some(name) = &opt.name {
		bundle.set_name(name);
	}

	if let Some(display_name) = &opt.display_name {
		bundle.set_display_name(display_name);
	}

	if let Some(identifier) = &opt.identifier {
		bundle.set_identifier(identifier);
	}

	if let Some(version) = &opt.version {
		bundle.set_version(version);
	}

	if let Some(icon) = opt.icon {
		bundle.set_icon(icon)?;
	}

	for r in opt.resources {
		bundle.add_resource(r)?;
	}

	for f in opt.frameworks {
		bundle.add_framework(f)?;
	}

	bundle.write()?;

	return Ok(());

}

fn main() {
	if let Err(e) = pack() {
		println!("{}", e);
	}
}

