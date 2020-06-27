// wengwengweng

mod utils;

use std::path::PathBuf;
use argh::FromArgs;

use packapp::*;

#[derive(FromArgs, Debug)]
/// pack a binary to a MacOS .app bundle
struct Opt {
	/// binary path
	#[argh(positional)]
	bin: PathBuf,
	#[argh(option)]
	/// stuff to copy into the "Resources" folder
	res: Vec<PathBuf>,
	#[argh(option)]
	/// stuff to copy into the "Frameworks" folder
	framework: Vec<PathBuf>,
	#[argh(option)]
	/// icon file
	icon: Option<PathBuf>,
	#[argh(option)]
	/// app identifier
	ident: Option<String>,
	#[argh(option)]
	/// app name
	name: Option<String>,
	#[argh(option)]
	/// app display name
	display_name: Option<String>,
	#[argh(option)]
	/// app version
	version: Option<String>,
	#[argh(switch)]
	/// if app should render in high resolution
	high_res: bool,
	#[argh(option)]
	/// output path
	output: Option<PathBuf>,
}

fn pack() -> Result<(), Error> {

    let opt = argh::from_env::<Opt>();
	let name = utils::basename(&opt.bin)?;
	let out = opt.output.unwrap_or(PathBuf::from(format!("{}.app", name)));
	let mut bb = BundleBuilder::new();

	if let Some(icon) = &opt.icon {
		if icon.exists() {
			bb.resource(icon);
		}
	}

	let mut plist = PlistData {
		name: opt.name,
		display_name: opt.display_name,
		identifier: opt.ident,
		version: opt.version,
		package_type: None,
		executable: Some(name),
		icon_file: opt.icon,
		signature: None,
		high_res: None,
		is_agent: None,
		document_types: None,
	};

	if opt.high_res {
		plist.high_res = Some(true);
	} else {
		plist.high_res = None;
	}

	bb.bin(&opt.bin);
	bb.plist(plist);

	for r in opt.res {
		bb.resource(r);
	}

	for f in opt.framework {
		bb.framework(f);
	}

	bb.build(out)?;

	return Ok(());

}

fn main() {
	if let Err(e) = pack() {
		eprintln!("{}", e);
	}
}

