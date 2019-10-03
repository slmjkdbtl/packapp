// wengwengweng

use std::path::PathBuf;
use structopt::StructOpt;
use structopt::clap::AppSettings::*;

use packapp::*;

#[derive(StructOpt, Debug)]
#[structopt(
	no_version,
	global_settings(&[
		ColoredHelp,
		VersionlessSubcommands,
		DisableHelpFlags,
		DisableHelpSubcommand,
		DisableVersion,
	])
)]
struct Opt {

	#[structopt(short = "r", long, parse(from_os_str))]
	resources: Vec<PathBuf>,

	#[structopt(short = "f", long, parse(from_os_str))]
	frameworks: Vec<PathBuf>,

	#[structopt(short = "c", long, parse(from_os_str))]
	icon: Option<PathBuf>,

	#[structopt(short = "i", long)]
	identifier: Option<String>,

	#[structopt(short = "n", long)]
	name: Option<String>,

	#[structopt(short = "d", long)]
	display_name: Option<String>,

	#[structopt(short = "v", long)]
	version: Option<String>,

	#[structopt(short = "p", long)]
	plain: bool,

	#[structopt(name = "BIN", parse(from_os_str))]
	bin: PathBuf,

	#[structopt(short = "o", long, parse(from_os_str))]
	out: Option<PathBuf>,

	#[structopt(long)]
	verbose: bool,

}

fn pack() -> Result<(), Error> {

    let opt = Opt::from_args();
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

