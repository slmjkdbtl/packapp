// wengwengweng

use std::fs::File;
use crate::utils;

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

}

impl Bundle {

	pub fn new(path: String) -> Self {

		utils::mkdir(&path);
		utils::mkdir(&format!("{}/Contents", path));

		let data = Data {

			CFBundleName: String::from(""),
			CFBundleDisplayName: String::from(""),
			CFBundleIdentifier: String::from(""),
			CFBundleVersion: String::from(""),
			CFBundlePackageType: String::from("APPL"),
			CFBundleExecutable: String::from(""),
			CFBundleIconFile: String::from(""),
			NSHighResolutionCapable: true,

		};

		let mut bundle = Self {
			path: String::from(path),
			data: data,
		};

		bundle.write_plist();

		return bundle;

	}

	pub fn write_plist(&self) -> &Self {

		let file = File::create(format!("{}/Contents/Info.plist", self.path)).unwrap();
		plist::serde::serialize_to_xml(&file, &self.data);

		return self;

	}

	pub fn add_bin(&mut self, bin: &str) -> &Self {

		utils::assert_exist(bin);

		let macos_dir = &format!("{}/Contents/MacOS", self.path);

		if !utils::exists(macos_dir) {
			utils::mkdir(macos_dir);
		}

		utils::copy(bin, &format!("{}/{}", macos_dir, bin));
		self.data.CFBundleExecutable = String::from(bin);
		self.write_plist();

		return self;

	}

	pub fn set_name(&mut self, name: &str) -> &Self {

		self.data.CFBundleName = String::from(name);
		self.write_plist();

		return self;

	}

	pub fn set_display_name(&mut self, name: &str) -> &Self {

		self.data.CFBundleDisplayName = String::from(name);
		self.write_plist();

		return self;

	}

	pub fn set_identifier(&mut self, ident: &str) -> &Self {

		self.data.CFBundleIdentifier = String::from(ident);
		self.write_plist();

		return self;

	}

	pub fn set_icon(&mut self, icon: &str) -> &Self {

		let res_dir = &format!("{}/Contents/Resources", self.path);

		if !utils::exists(res_dir) {
			utils::mkdir(res_dir);
		}

		utils::assert_exist(icon);
		utils::assert_ext(icon, "icns");
		utils::copy(icon, &format!("{}/{}", res_dir, icon));
		self.data.CFBundleIconFile = String::from(icon);
		self.write_plist();

		return self;

	}

	pub fn add_res(&self) -> &Self {

		let res_dir = &format!("{}/Contents/Resources", self.path);

		if !utils::exists(res_dir) {
			utils::mkdir(res_dir);
		}

		return self;

	}

	pub fn add_frameworks(&self) -> &Self {

		let frameworks_dir = &format!("{}/Contents/Resources", self.path);

		if !utils::exists(frameworks_dir) {
			utils::mkdir(frameworks_dir);
		}

		return self;

	}

}

