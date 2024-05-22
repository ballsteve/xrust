/*! A generic test suite.

Most of xrust's modules are written to use the generic Node trait. However, to test their functionality a concrete implementation must be used. Rather than writing, and rewriting, the same set of tests for each concrete implementation, all of the tests have been written as macros. An implementation can then be tested by calling the macros using the type that implements Node.
*/

pub mod item_node;
pub mod item_value;
pub mod pattern_tests;
pub mod transform_tests;
pub mod xpath_tests;
