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

pub fn require_exist(path: &str) {
	if !exists(path) {
		fail(&format!("\"{}\" not found", path), ErrorKind::Io);
	}
}

pub fn require_ext(path: &str, expected_ext: &str) {
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
	fs::create_dir(dir).unwrap_or_else(|s| {
		fail(&format!("failed to create dir {}", dir), ErrorKind::Io);
	});
}

pub fn copy(f1: &str, f2: &str) {
	println!("{} -> {}", f1, f2);
	fs::copy(f1, f2).unwrap_or_else(|s| {
		fail(&format!("failed to copy {} to {}", f1, f2), ErrorKind::Io);
		return 0;
	});
}

pub fn write(file: &str, content: &str) {
	println!("> {}", file);
	fs::write(file, content).unwrap_or_else(|s| {
		fail(&format!("failed to write to {}", file), ErrorKind::Io);
	});
}


