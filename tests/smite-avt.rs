// Smite tests for pattern module

use xrust::trees::smite::RNode;

mod avt;
mod smite;

#[test]
fn avt_empty() {
    avt::avt_empty::<RNode>().expect("test failed")
}
#[test]
fn avt_content() {
    avt::content::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn avt_var_ref() {
    avt::var_ref::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
