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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlistData {
	#[serde(rename = "CFBundleName")]
	pub name: Option<String>,
	#[serde(rename = "CFBundleDisplayName")]
	pub display_name: Option<String>,
	#[serde(rename = "CFBundleIdentifier")]
	pub identifier: Option<String>,
	#[serde(rename = "CFBundleVersion")]
	pub version: Option<String>,
	#[serde(rename = "CFBundlePackageType")]
	pub package_type: Option<String>,
	#[serde(rename = "CFBundleExecutable")]
	pub executable: String,
	#[serde(rename = "CFBundleIconFile")]
	pub icon_file: Option<String>,
	#[serde(rename = "CFBundleSignature")]
	pub signature: Option<String>,
	#[serde(rename = "NSHighResolutionCapable")]
	pub high_res_capable: bool,
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
		let oname = PathBuf::from(format!("{}.app", name));

		let data = PlistData {
			name: None,
			display_name: None,
			identifier: None,
			version: None,
			package_type: None,
			executable: name,
			icon_file: None,
			signature: None,
			high_res_capable: true,
		};

		let bundle = Self {
			bin: path.to_owned(),
			data: data,
			output: oname,
			icon: None,
			frameworks: vec![],
			resources: vec![],
		};

		return Ok(bundle);

	}

	pub fn set_output(&mut self, path: impl AsRef<Path>) {
		self.output = path.as_ref().to_owned();
	}

	pub fn set_plist_data(&mut self, data: PlistData) {
		self.data = data;
	}

	pub fn set_name(&mut self, name: &str) {
		self.data.name = Some(String::from(name));
	}

	pub fn set_display_name(&mut self, name: &str) {
		self.data.display_name = Some(String::from(name));
	}

	pub fn set_identifier(&mut self, ident: &str) {
		self.data.identifier = Some(String::from(ident));
	}

	pub fn set_version(&mut self, version: &str) {
		self.data.version = Some(String::from(version));
	}

	pub fn set_icon(&mut self, path: impl AsRef<Path>) -> Result<()> {

		let path = path.as_ref();

		utils::assert_exists(path)?;
// 		utils::assert_ext(path, "icns")?;
		self.data.icon_file = Some(format!("{}", utils::base(path)?.display()));
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

