// wengwengweng

use std::fs::File;
use crate::utils;

enum AppDir {
	Resources,
	Frameworks,
	MacOS,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {

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
	data: Data,
	bin: Option<String>,
	icon: Option<String>,
	frameworks: Vec<String>,
	resources: Vec<String>,

}

impl Bundle {

	pub fn new(path: String) -> Self {

		let data = Data {
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

	pub fn write_plist(&self) -> &Self {

		let file = File::create(format!("{}/Contents/Info.plist", self.path)).unwrap();
		plist::serde::serialize_to_xml(&file, &self.data);

		return self;

	}

	pub fn add_bin(&mut self, bin: &str) -> &Self {

		utils::assert_exist(bin);
		self.data.CFBundleExecutable = String::from(bin);
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
		self.data.CFBundleIconFile = String::from(icon);
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

	pub fn copy(&self, file: &str, des: AppDir, subdir: &str) -> &Self {

		let mut dir = "";

		match des {
			AppDir::MacOS => dir = "MacOS",
			AppDir::Resources => dir = "Resources",
			AppDir::Frameworks => dir = "Frameworks",
		}

		let path = &format!("{}/Contents/{}", self.path, dir);

		if !utils::exists(path) {
			utils::mkdir(path);
		}

		utils::copy(&file, &format!("{}/{}", path, file));

		return self;

	}

	pub fn write(&self) {

		utils::mkdir(&self.path);
		utils::mkdir(&format!("{}/Contents", self.path));

		let macos_dir = &format!("{}/Contents/MacOS", self.path);
		let res_dir = &format!("{}/Contents/Resources", self.path);
		let frameworks_dir = &format!("{}/Contents/Frameworks", self.path);

		if let Some(bin) = &self.bin {
			self.copy(bin, AppDir::MacOS, "");
		}

		if let Some(icon) = &self.icon {
			self.copy(icon, AppDir::Resources, "");
		}

		self.write_plist();

	}

}

