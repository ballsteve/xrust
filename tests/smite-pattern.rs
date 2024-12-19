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
fn pattern_abbrev_1_pos() {
    patterngeneric::pattern_abbrev_1_pos::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn pattern_abbrev_1_neg() {
    patterngeneric::pattern_abbrev_1_neg::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn pattern_abbrev_2_pos() {
    patterngeneric::pattern_abbrev_2_pos::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn pattern_abbrev_2_neg() {
    patterngeneric::pattern_abbrev_2_neg::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn pattern_sel_text_kind_1_pos() {
    patterngeneric::pattern_sel_text_kind_1_pos::<RNode, _>(smite::make_empty_doc)
        .expect("test failed")
}
#[test]
fn pattern_issue_95() {
    patterngeneric::pattern_issue_95::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn pattern_union_1() {
    patterngeneric::pattern_union_1::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn pattern_union_2() {
    patterngeneric::pattern_union_2::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn pattern_union_3() {
    patterngeneric::pattern_union_3::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
#[test]
fn pattern_union_4() {
    patterngeneric::pattern_union_4::<RNode, _>(smite::make_empty_doc).expect("test failed")
}
