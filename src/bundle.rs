// wengwengweng

use std::path::PathBuf;
use std::path::Path;
use std::fs::File;

use serde::Deserialize;
use serde::Serialize;

use crate::utils;
use crate::Result;
use crate::Error;

enum AppDir {
	Resources,
	Frameworks,
	MacOS,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlistData {

	CFBundleName: String,
	CFBundleDisplayName: String,
	CFBundleIdentifier: String,
	CFBundleVersion: String,
	CFBundlePackageType: String,
	CFBundleExecutable: String,
	CFBundleIconFile: String,
	NSHighResolutionCapable: bool,

}

pub struct Bundle {

	bin: PathBuf,
	data: PlistData,
	path: PathBuf,
	icon: Option<PathBuf>,
	frameworks: Vec<PathBuf>,
	resources: Vec<PathBuf>,

}

impl Bundle {

	pub fn new(path: impl AsRef<Path>) -> Result<Self> {

		let path = path.as_ref();

		utils::assert_exists(path)?;

		let name = utils::basename(path)?;

		let data = PlistData {
			CFBundleName: "".to_owned(),
			CFBundleDisplayName: "".to_owned(),
			CFBundleIdentifier: "".to_owned(),
			CFBundleVersion: "".to_owned(),
			CFBundlePackageType: "APPL".to_owned(),
			CFBundleExecutable: name.clone(),
			CFBundleIconFile: "".to_owned(),
			NSHighResolutionCapable: true,
		};

		let bundle = Self {
			bin: path.to_owned(),
			data: data,
			path: PathBuf::from(format!("{}.app", name)),
			icon: None,
			frameworks: vec![],
			resources: vec![],
		};

		return Ok(bundle);

	}

	pub fn set_name(&mut self, name: &str) {
		self.data.CFBundleName = String::from(name);
	}

	pub fn set_display_name(&mut self, name: &str) {
		self.data.CFBundleDisplayName = String::from(name);
	}

	pub fn set_identifier(&mut self, ident: &str) {
		self.data.CFBundleIdentifier = String::from(ident);
	}

	pub fn set_version(&mut self, version: &str) {
		self.data.CFBundleVersion = String::from(version);
	}

	pub fn set_icon(&mut self, path: impl AsRef<Path>) -> Result<()> {

		let path = path.as_ref();

		utils::assert_exists(path)?;
		utils::assert_ext(path, "icns")?;
		self.data.CFBundleIconFile = format!("{}", utils::base(path)?.display());
		self.icon = Some(path.to_owned());

		return Ok(());

	}

	pub fn add_resource(&mut self, path: impl AsRef<Path>) -> Result<()> {

		let path = path.as_ref();

		utils::assert_exists(path)?;
		self.resources.push(path.to_owned());

		return Ok(());

	}

	pub fn add_framework(&mut self, path: impl AsRef<Path>) -> Result<()> {

		let path = path.as_ref();

		utils::assert_exists(path)?;
		self.frameworks.push(path.to_owned());

		return Ok(());

	}

	pub fn write(&self) -> Result<()> {

		utils::mkdir(&self.path)?;
		utils::mkdir(self.path.join("Contents"))?;

		self.copy(&self.bin, AppDir::MacOS)?;

		if let Some(icon) = &self.icon {
			self.copy(icon, AppDir::Resources)?;
		}

		for r in &self.resources {
			self.copy(&r, AppDir::Resources)?;
		}

		for f in &self.frameworks {
			self.copy(&f, AppDir::Frameworks)?;
		}

		self.write_plist()?;

		return Ok(());

	}

	fn copy(&self, path: impl AsRef<Path>, des: AppDir) -> Result<()> {

		let path = path.as_ref();

		let dir = match des {
			AppDir::MacOS => "MacOS",
			AppDir::Resources => "Resources",
			AppDir::Frameworks => "Frameworks",
		};

		let dir = self.path.join("Contents").join(dir);

		if !utils::exists(&dir) {
			utils::mkdir(&dir)?;
		}

		if utils::is_file(path) {
			utils::copy(path, dir.join(utils::base(path)?))?;
		} else if utils::is_dir(path) {
			utils::copy_dir(path, dir.join(utils::base(path)?))?;
		}

		return Ok(());

	}

	fn write_plist(&self) -> Result<()> {

		let path = self.path.join("Contents").join("Info.plist");

		plist::to_file_xml(&path, &self.data)?;

		return Ok(());

	}

}

