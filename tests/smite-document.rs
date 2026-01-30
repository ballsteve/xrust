// Smite tests for documents

use xrust::trees::smite::RNode;

mod documentgeneric;
mod smite;

#[test]
fn document_create() {
    documentgeneric::document_create::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn document_descope_1() {
    documentgeneric::descope_1::<RNode, _>(smite::make_from_str).expect("test failed")
}
#[test]
fn document_descope_2() {
    documentgeneric::descope_2::<RNode, _>(smite::make_from_str).expect("test failed")
}
