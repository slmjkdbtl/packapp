// wengwengweng

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::Result;
use crate::Error;

pub fn exists(path: impl AsRef<Path>) -> bool {
	return path.as_ref().exists();
}

pub fn assert_exists(path: impl AsRef<Path>) -> Result<()> {
	let path = path.as_ref();
	if !exists(path) {
		return Err(Error::IO(format!("{} not found", path.display())))
	} else {
		return Ok(());
	}
}

pub fn is_dir(path: impl AsRef<Path>) -> bool {
	return path.as_ref().is_dir();
}

pub fn is_file(path: impl AsRef<Path>) -> bool {
	return path.as_ref().is_file();
}

pub fn mkdir(path: impl AsRef<Path>) -> Result<()> {

	let path = path.as_ref();

	return fs::create_dir_all(path)
		.map_err(|_| Error::IO(format!("failed to create directory {}", path.display())));

}

pub fn copy(p1: impl AsRef<Path>, p2: impl AsRef<Path>) -> Result<u64> {

	let p1 = p1.as_ref();
	let p2 = p2.as_ref();

	return fs::copy(p1, p2)
		.map_err(|_| Error::IO(format!("failed to copy {} to {}", p1.display(), p2.display())));

}

pub fn copy_dir(p1: impl AsRef<Path>, p2: impl AsRef<Path>) -> Result<u64> {

	let p1 = p1.as_ref();
	let p2 = p2.as_ref();
	let mut options = fs_extra::dir::CopyOptions::new();

	options.overwrite = true;
	options.copy_inside = true;

	return fs_extra::dir::copy(p1, p2, &options)
		.map_err(|_| Error::IO(format!("failed to copy {} to {}", p1.display(), p2.display())));

}

pub fn write(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> Result<()> {

	let path = path.as_ref();

	return fs::write(path, content)
		.map_err(|_| Error::IO(format!("failed to write file {}", path.display())));

}

pub fn read(path: impl AsRef<Path>) -> Result<Vec<u8>> {

	let path = path.as_ref();

	return fs::read(&path)
		.map_err(|_| Error::IO(format!("failed to read file {}", path.display())));

}

pub fn basename(path: impl AsRef<Path>) -> Result<String> {

	let path = path
		.as_ref();

	return Ok(
		path
			.file_stem()
			.ok_or(Error::IO(format!("failed to get basename: {}", path.display())))?
			.to_str()
			.ok_or(Error::IO(format!("failed to get basename: {}", path.display())))?
			.to_owned()
	);

}

pub fn base(path: impl AsRef<Path>) -> Result<PathBuf> {

	let path = path.as_ref();

	return path
		.iter()
		.last()
		.map(PathBuf::from)
		.ok_or(Error::IO(format!("failed to get base: {}", path.display())))
		;

}

pub fn extname(path: impl AsRef<Path>) -> Result<String> {

	let path = path.as_ref();

	return Ok(path
		.extension()
		.ok_or(Error::IO(format!("failed to get extname: {}", path.display())))?
		.to_os_string()
		.into_string().map_err(|_| Error::IO(format!("failed to get extname: {}", path.display())))?
	);

}

pub fn assert_ext(path: impl AsRef<Path>, ext: &str) -> Result<()> {
	let path = path.as_ref();
	if extname(path)? != String::from(ext) {
		return Err(Error::IO(format!("invalid file {}, expected a .{}", path.display(), ext)))
	} else {
		return Ok(());
	}
}

