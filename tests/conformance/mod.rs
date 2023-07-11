use std::fs;
use xrust::{Error, ErrorKind};

mod xml;


fn dtdfileresolve() -> fn(Option<String>, String) -> Result<String, Error> {
    move |locdir, uri| {
        let u = match locdir {
            None => uri,
            Some(ld) => ld + uri.as_str()
        };
        match fs::read_to_string(u) {
            Err(e) => Err(xrust::Error {
                kind: ErrorKind::Unknown,
                message: "Unable to read external DTD".to_string(),
            }),
            Ok(s) => Ok(s),
        }
    }
}