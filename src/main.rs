// wengwengweng

use std::path::PathBuf;
use structopt::StructOpt;

use packapp::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "packapp")]
struct Opt {

	#[structopt(long, parse(from_os_str))]
	resources: Vec<PathBuf>,

	#[structopt(long, parse(from_os_str))]
	frameworks: Vec<PathBuf>,

	#[structopt(long, parse(from_os_str))]
	icon: Option<PathBuf>,

	#[structopt(long)]
	identifier: Option<String>,

	#[structopt(long)]
	name: Option<String>,

	#[structopt(long)]
	display_name: Option<String>,

	#[structopt(long)]
	version: Option<String>,

	#[structopt(name = "BIN", parse(from_os_str))]
	bin: PathBuf,

}

fn pack() -> Result<(), Error> {

    let opt = Opt::from_args();
	let mut bundle = Bundle::new(opt.bin)?;

	bundle.set_name(&opt.name.unwrap_or(String::new()));
	bundle.set_display_name(&opt.display_name.unwrap_or(String::new()));
	bundle.set_identifier(&opt.identifier.unwrap_or(String::new()));
	bundle.set_version(&opt.version.unwrap_or(String::new()));

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

	let result = pack();

	if let Err(e) = result {
		println!("{}", e);
	}

}

