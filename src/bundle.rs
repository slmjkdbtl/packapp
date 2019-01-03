// wengwengweng

use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use std::fs::File;
use clap::ErrorKind;
use crate::utils;

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

	path: String,
	data: PlistData,
	bin: Option<String>,
	icon: Option<String>,
	frameworks: Vec<String>,
	resources: Vec<String>,

}

impl Bundle {

	pub fn new(path: &str) -> Self {

		let data = PlistData {
			CFBundleName: "".to_owned(),
			CFBundleDisplayName: "".to_owned(),
			CFBundleIdentifier: "".to_owned(),
			CFBundleVersion: "".to_owned(),
			CFBundlePackageType: "APPL".to_owned(),
			CFBundleExecutable: "".to_owned(),
			CFBundleIconFile: "".to_owned(),
			NSHighResolutionCapable: true,
		};

		let bundle = Self {
			path: path.to_owned(),
			data: data,
			bin: None,
			icon: None,
			frameworks: vec![],
			resources: vec![],
		};

		return bundle;

	}

	pub fn set_bin(&mut self, bin: &str) -> &Self {

		utils::assert_exist(bin);
		self.data.CFBundleExecutable = utils::basename(bin);
		self.bin = Some(bin.to_owned());

		return self;

	}

	pub fn set_name(&mut self, name: &str) -> &Self {

		self.data.CFBundleName = String::from(name);

		return self;

	}

	pub fn set_display_name(&mut self, name: &str) -> &Self {

		self.data.CFBundleDisplayName = String::from(name);

		return self;

	}

	pub fn set_identifier(&mut self, ident: &str) -> &Self {

		self.data.CFBundleIdentifier = String::from(ident);

		return self;

	}

	pub fn set_version(&mut self, version: &str) -> &Self {

		self.data.CFBundleVersion = String::from(version);

		return self;

	}

	pub fn set_icon(&mut self, icon: &str) -> &Self {

		utils::assert_exist(icon);
		utils::assert_ext(icon, "icns");
		self.data.CFBundleIconFile = utils::basename(icon);
		self.icon = Some(icon.to_owned());

		return self;

	}

	pub fn add_res(&mut self, file: &str) -> &Self {

		utils::assert_exist(file);
		self.resources.push(file.to_owned());

		return self;

	}

	pub fn add_frameworks(&mut self, file: &str) -> &Self {

		utils::assert_exist(file);
		self.frameworks.push(file.to_owned());

		return self;

	}

	pub fn write(&self) {

		utils::mkdir(&self.path);
		utils::mkdir(&format!("{}/Contents", self.path));

		if let Some(bin) = &self.bin {
			self.copy(bin, AppDir::MacOS, "");
		}

		if let Some(icon) = &self.icon {
			self.copy(icon, AppDir::Resources, "");
		}

		self.write_plist();

	}

	fn copy(&self, file: &str, des: AppDir, subdir: &str) -> &Self {

		let dir;

		match des {
			AppDir::MacOS => dir = "MacOS",
			AppDir::Resources => dir = "Resources",
			AppDir::Frameworks => dir = "Frameworks",
		}

		let path = &format!("{}/Contents/{}", self.path, dir);

		if !utils::exists(path) {
			utils::mkdir(path);
		}

		utils::copy(&file, &format!("{}/{}", path, utils::basename(file)));

		return self;

	}

	fn write_plist(&self) -> &Self {

		let file = File::create(format!("{}/Contents/Info.plist", self.path)).unwrap();

		if plist::serde::serialize_to_xml(&file, &self.data).is_err() {
			utils::fail("failed to write plist", ErrorKind::Io);
		}

		return self;

	}

}

