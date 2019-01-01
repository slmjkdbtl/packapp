// wengwengweng

use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use crate::utils;

pub struct Bundle {
	path: String,
}

impl Bundle {

	pub fn new(path: String) -> Self {

		utils::mkdir(&path);
		utils::mkdir(&format!("{}/Contents", path));

		return Self {
			path: String::from(path),
		};

	}

	pub fn update_plist(&self) -> &Self {
		return self;
	}

	pub fn add_bin(&self, bin: &str) -> &Self {

		utils::require_exist(bin);

		let macos_dir = &format!("{}/Contents/MacOS", self.path);

		if !utils::exists(macos_dir) {
			utils::mkdir(macos_dir);
		}

		utils::copy(bin, &format!("{}/{}", macos_dir, bin));

		return self;

	}

	pub fn add_res(&self) -> &Self {

		let res_dir = &format!("{}/Contents/Resources", self.path);

		if !utils::exists(res_dir) {
			utils::mkdir(res_dir);
		}

		return self;

	}

	pub fn add_icon(&self) -> &Self {

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

