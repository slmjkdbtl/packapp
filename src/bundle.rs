// wengwengweng

// https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFBundles/BundleTypes/BundleTypes.html
// https://developer.apple.com/library/archive/documentation/General/Reference/InfoPlistKeyReference/Introduction/Introduction.html

use std::path::PathBuf;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use crate::*;

#[derive(Clone, Copy, Debug)]
enum AppDir {
	MacOS,
	Resources,
	Frameworks,
	Plugins,
	SharedSupport,
}

impl AppDir {
	pub fn as_str(&self) -> &'static str {
		return match self {
			AppDir::MacOS => "MacOS",
			AppDir::Resources => "Resources",
			AppDir::Frameworks => "Frameworks",
			AppDir::Plugins => "PlugIns",
			AppDir::SharedSupport => "SharedSupport",
		};
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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
	pub package_type: Option<PackageType>,
	#[serde(rename = "CFBundleExecutable")]
	pub executable: Option<String>,
	#[serde(rename = "CFBundleIconFile")]
	pub icon_file: Option<PathBuf>,
	#[serde(rename = "CFBundleSignature")]
	pub signature: Option<String>,
	#[serde(rename = "NSHighResolutionCapable")]
	pub high_res: Option<bool>,
	#[serde(rename = "CFBundleDocumentTypes")]
	pub document_types: Option<Vec<DocumentType>>,
	#[serde(rename = "LSUIElement")]
	pub is_agent: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocumentType {
	#[serde(rename = "CFBundleTypeName")]
	pub name: Option<String>,
	#[serde(rename = "CFBundleTypeExtensions")]
	pub extensions: Option<Vec<String>>,
	#[serde(rename = "CFBundleTypeRole")]
	pub role: Option<Role>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
	#[serde(rename = "Editor")]
	Editor,
	#[serde(rename = "Viewer")]
	Viewer,
	#[serde(rename = "Shell")]
	Shell,
	#[serde(rename = "None")]
	None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PackageType {
	#[serde(rename = "APPL")]
	Application,
	#[serde(rename = "FMWK")]
	Framework,
	#[serde(rename = "BNDL")]
	Bundle,
}

pub struct BundleBuilder {
	bin: Option<PathBuf>,
	plist: PlistData,
	frameworks: Vec<PathBuf>,
	resources: Vec<PathBuf>,
}

impl BundleBuilder {

	pub fn new() -> Self {
		return Self {
			bin: None,
			plist: PlistData::default(),
			frameworks: vec![],
			resources: vec![],
		};
	}

	pub fn bin(&mut self, path: impl AsRef<Path>) -> &mut Self {
		self.bin = Some(path.as_ref().to_path_buf());
		return self;
	}

	pub fn plist(&mut self, plist: PlistData) -> &mut Self {
		self.plist = plist;
		return self;
	}

	pub fn resource(&mut self, path: impl AsRef<Path>) -> &mut Self {
		self.resources.push(path.as_ref().to_path_buf());
		return self;
	}

	pub fn framework(&mut self, path: impl AsRef<Path>) -> &mut Self {
		self.frameworks.push(path.as_ref().to_path_buf());
		return self;
	}

	pub fn build(self, out: impl AsRef<Path>) -> Result<()> {

		let out = out.as_ref();

		utils::mkdir(&out)?;
		utils::mkdir(out.join("Contents"))?;

		if let Some(bin) = &self.bin {
			self.copy(bin, out, AppDir::MacOS)?;
		}

		for r in &self.resources {
			self.copy(r, out, AppDir::Resources)?;
		}

		for f in &self.frameworks {
			self.copy(f, out, AppDir::Frameworks)?;
		}

		// write plist
		let plist_path = out.join("Contents").join("Info.plist");

		plist::to_file_xml(&plist_path, &self.plist)?;

		return Ok(());

	}

	fn copy(&self, path: impl AsRef<Path>, out: impl AsRef<Path>, dest: AppDir) -> Result<()> {

		let out = out.as_ref();
		let path = path.as_ref();
		let dir = out.join("Contents").join(dest.as_str());

		if !path.exists() {
			return Err(Error::IO(format!("path '{}' not found", path.display())));
		}

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

}

