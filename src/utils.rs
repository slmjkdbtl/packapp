// wengwengweng

use std::fs;
use std::path::Path;
use clap::{Error, ErrorKind};

pub fn fail(msg: &str, kind: ErrorKind) {
	Error::with_description(msg, kind).exit();
}

pub fn exists(path: &str) -> bool {
	return Path::new(path).exists();
}

pub fn assert_exist(path: &str) {

	if !exists(path) {
		fail(&format!("\"{}\" not found", path), ErrorKind::Io);
	}

}

pub fn assert_ext(path: &str, expected_ext: &str) {

	if let Some(ext) = Path::new(path).extension() {
		if ext != expected_ext {
			fail(&format!("invalid file \"{}\", expected a .{}", path, expected_ext), ErrorKind::Io);
		}
	} else {
		fail(&format!("invalid file \"{}\"", path), ErrorKind::Io);
	}

}

pub fn mkdir(dir: &str) {

	println!("+ {}", dir);

	if fs::create_dir(dir).is_err() {
		fail(&format!("failed to create dir {}", dir), ErrorKind::Io);
	}

}

pub fn copy(f1: &str, f2: &str) {

	println!("{} -> {}", f1, f2);

	if fs::copy(f1, f2).is_err() {
		fail(&format!("failed to copy {} to {}", f1, f2), ErrorKind::Io);
	}

}

pub fn write(file: &str, content: &str) {

	println!("> {}", file);

	if fs::write(file, content).is_err() {
		fail(&format!("failed to write {}", file), ErrorKind::Io);
	}

}

pub fn read(file: &str) -> String {

	println!("< {}", file);

	return fs::read_to_string(file).unwrap_or_else(|s| {
		fail(&format!("failed to read {}", file), ErrorKind::Io);
		return String::from("");
	});

}

