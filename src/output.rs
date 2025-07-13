/*! How to serialise a tree structure.
*/

use crate::qname::UriQualifiedName;
use core::fmt;

/// An output definition. See XSLT v3.0 26 Serialization
#[derive(Clone, Debug)]
pub struct OutputDefinition {
    name: Option<UriQualifiedName>, // Don't use QualifiedName to avoid dependency on Interner
    indent: bool,
    // TODO: all the other myriad output parameters
}

impl Default for OutputDefinition {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputDefinition {
    pub fn new() -> OutputDefinition {
        OutputDefinition {
            name: None,
            indent: false,
        }
    }
    pub fn get_name(&self) -> Option<UriQualifiedName> {
        self.name.clone()
    }
    pub fn set_name(&mut self, name: Option<UriQualifiedName>) {
        match name {
            Some(n) => {
                self.name.replace(n);
            }
            None => {
                self.name = None;
            }
        }
    }
    pub fn get_indent(&self) -> bool {
        self.indent
    }
    pub fn set_indent(&mut self, ind: bool) {
        self.indent = ind;
    }
}
impl fmt::Display for OutputDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.indent {
            f.write_str("indent output")
        } else {
            f.write_str("do not indent output")
        }
    }
}
