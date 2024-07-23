// Smite tests for pattern module

use xrust::trees::smite::RNode;

mod patterngeneric;
mod smite;

#[test]
#[should_panic]
fn pattern_empty() {
    patterngeneric::pattern_empty::<RNode>().expect("test failed")
}
#[test]
fn pattern_predicate_1_pos() {
    patterngeneric::pattern_predicate_1_pos::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn pattern_predicate_1_neg() {
    patterngeneric::pattern_predicate_1_neg::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn pattern_sel_root_pos() {
    patterngeneric::pattern_sel_root_pos::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn pattern_sel_root_neg() {
    patterngeneric::pattern_sel_root_neg::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn pattern_sel_1_pos() {
    patterngeneric::pattern_sel_1_pos::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn pattern_sel_1_neg() {
    patterngeneric::pattern_sel_1_neg::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn pattern_sel_2_pos() {
    patterngeneric::pattern_sel_2_pos::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn pattern_sel_2_neg() {
    patterngeneric::pattern_sel_2_neg::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn pattern_sel_text_kind_1_pos() {
    patterngeneric::pattern_sel_text_kind_1_pos::<RNode, _>(smite::make_empty_doc)
        .expect("test failed")
}
