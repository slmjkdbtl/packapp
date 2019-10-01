// wengwengweng

use std::fmt;

#[derive(Debug)]
pub enum Error {
	IO(String),
	PList,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return match self {
			Error::IO(s) => write!(f, "{}", s),
			Error::PList => write!(f, "failed to write plist"),
		};
	}

}

impl From<plist::Error> for Error {
	fn from(_: plist::Error) -> Self {
		return Error::PList;
	}
}

