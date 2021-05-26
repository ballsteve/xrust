//! # An XSLT compiler
//!
//! Compile an XSLT stylesheet into a sequence constructor.

use roxmltree::Document;
//use crate::evaluate::{
//  evaluate,
//  Constructor,
//  DynamicContext,
//  StaticContext,
//  static_analysis,
//}

pub trait XSLT {
  /// Compile from a roxmltree
  fn from_xnode(&self) -> Vec<Constructor>;
}

impl XSLT for Document {
  fn from_xnode(&self) -> Vec<Constructor> {
    // Check that this is a valid XSLT stylesheet
    // Strip/preserve whitespace
    // Iterate over children, looking for templates
    // * compile match pattern
    // * compile content into sequence constructor
    // * register template in static context
    vec![]
  }
}

