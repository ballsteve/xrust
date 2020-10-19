//! # xdm::error
//!
//! XDM, XPath, XQuery and XSLT errors.

use core::{fmt, str};

pub enum ErrorKind {
    TypeError,
    Unknown,
}

impl ErrorKind {
    pub fn to_string(&self) -> &'static str {
        match *self {
            ErrorKind::TypeError => "type error",
	    ErrorKind::Unknown => "unknown",
	}
    }
}

pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.message)
    }
}

