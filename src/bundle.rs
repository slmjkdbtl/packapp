// wengwengweng

// https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFBundles/BundleTypes/BundleTypes.html

use std::path::PathBuf;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use crate::utils;
use crate::Result;

enum AppDir {
	Resources,
	Frameworks,
	Plugins,
	MacOS,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlistData {
	CFBundleName: Option<String>,
	CFBundleDisplayName: Option<String>,
	CFBundleIdentifier: Option<String>,
	CFBundleVersion: Option<String>,
	CFBundlePackageType: Option<String>,
	CFBundleExecutable: Option<String>,
	CFBundleIconFile: Option<String>,
	CFBundleSignature: Option<String>,
}

pub struct Bundle {

	bin: PathBuf,
	data: PlistData,
	output: PathBuf,
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
			CFBundleName: None,
			CFBundleDisplayName: None,
			CFBundleIdentifier: None,
			CFBundleVersion: None,
			CFBundlePackageType: Some(String::from("APPL")),
			CFBundleExecutable: Some(String::from(name.clone())),
			CFBundleIconFile: None,
			CFBundleSignature: None,
		};

		let bundle = Self {
			bin: path.to_owned(),
			data: data,
			output: PathBuf::from(format!("{}.app", name)),
			icon: None,
			frameworks: vec![],
			resources: vec![],
		};

		return Ok(bundle);

	}

	pub fn set_output(&mut self, path: impl AsRef<Path>) {
		self.output = path.as_ref().to_owned();
	}

	pub fn set_name(&mut self, name: &str) {
		self.data.CFBundleName = Some(String::from(name));
	}

	pub fn set_display_name(&mut self, name: &str) {
		self.data.CFBundleDisplayName = Some(String::from(name));
	}

	pub fn set_identifier(&mut self, ident: &str) {
		self.data.CFBundleIdentifier = Some(String::from(ident));
	}

	pub fn set_version(&mut self, version: &str) {
		self.data.CFBundleVersion = Some(String::from(version));
	}

	pub fn set_icon(&mut self, path: impl AsRef<Path>) -> Result<()> {

		let path = path.as_ref();

		utils::assert_exists(path)?;
// 		utils::assert_ext(path, "icns")?;
		self.data.CFBundleIconFile = Some(format!("{}", utils::base(path)?.display()));
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

		utils::mkdir(&self.output)?;
		utils::mkdir(self.output.join("Contents"))?;

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
			AppDir::Plugins => "Plugins",
		};

		let dir = self.output.join("Contents").join(dir);

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

		let path = self.output.join("Contents").join("Info.plist");

		plist::to_file_xml(&path, &self.data)?;

		return Ok(());

	}

}

