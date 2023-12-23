/*! Defines interfaces for closures and functions that are used to communicate with external processes.
*/

use crate::xdmerror::Error;

/// Resolves a URL, given as a base URI and a relative URL, and returns the content of the resource as a string.
pub(crate) type URLResolver = fn(Option<String>, String) -> Result<String, Error>;

