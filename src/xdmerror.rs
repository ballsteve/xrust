//! # xrust::error
//!
//! XDM, XPath, XQuery and XSLT errors.

use core::{fmt, str};

/// Errors defined in XPath
#[derive(Copy, Clone, Debug)]
pub enum ErrorKind {
    StaticAbsent,
    /// XPST0001
    DynamicAbsent,
    /// XPDY0002
    StaticSyntax,
    /// XPST0003
    TypeError,
    /// XPTY0004
    StaticData,
    /// XPST0005
    StaticUndefined,
    /// XPST0008
    StaticNamespace,
    /// XPST0010
    StaticBadFunction,
    /// XPST0017
    MixedTypes,
    /// XPTY0018
    NotNodes,
    /// XPTY0019
    ContextNotNode,
    /// XPTY0020
    NotImplemented,
    Unknown,
}

impl ErrorKind {
    /// String representation of error
    pub fn to_string(&self) -> &'static str {
        match *self {
            ErrorKind::StaticAbsent => "a component of the static context is absent",
            ErrorKind::DynamicAbsent => "a component of the dynamic context is absent",
            ErrorKind::StaticSyntax => "syntax error",
            ErrorKind::TypeError => "type error",
            ErrorKind::StaticData => "wrong static type",
            ErrorKind::StaticUndefined => "undefined name",
            ErrorKind::StaticNamespace => "namespace axis not supported",
            ErrorKind::StaticBadFunction => "function call name and arity do not match",
            ErrorKind::MixedTypes => "result of path operator contains both nodes and non-nodes",
            ErrorKind::NotNodes => "path expression is not a sequence of nodes",
            ErrorKind::ContextNotNode => "context item is not a node for an axis step",
            ErrorKind::NotImplemented => "not implemented",
            ErrorKind::Unknown => "unknown",
        }
    }
}

/// An error returned by an XPath, XQuery or XSLT function/method
#[derive(Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

impl std::error::Error for Error {}

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Self {
        Error { kind, message }
    }
    pub fn to_string(&self) -> String {
        self.message.clone()
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.message)
    }
}
