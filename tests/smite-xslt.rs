mod smite;
mod xsltgeneric;

// XSLT tests

#[test]
fn xslt_literal_text() {
    xsltgeneric::generic_literal_text(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_sys_prop() {
    xsltgeneric::generic_sys_prop(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_value_of_1() {
    xsltgeneric::generic_value_of_1(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_value_of_2() {
    xsltgeneric::generic_value_of_2(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_literal_element() {
    xsltgeneric::generic_literal_element(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_empty_doc_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_element() {
    xsltgeneric::generic_element(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_apply_templates_1() {
    xsltgeneric::generic_apply_templates_1(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_apply_templates_2() {
    xsltgeneric::generic_apply_templates_2(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_apply_templates_mode() {
    xsltgeneric::generic_apply_templates_mode(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_apply_templates_mode_bad_qname() {
    xsltgeneric::generic_apply_templates_mode_bad_qname(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_apply_templates_sort() {
    xsltgeneric::generic_apply_templates_sort(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_comment() {
    xsltgeneric::generic_comment(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_pi() {
    xsltgeneric::generic_pi(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_message_1() {
    xsltgeneric::generic_message_1(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_message_term() {
    xsltgeneric::generic_message_term(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_issue_58() {
    xsltgeneric::generic_issue_58(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_issue_95() {
    xsltgeneric::generic_issue_95(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_callable_named_1() {
    xsltgeneric::generic_callable_named_1(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_callable_posn_1() {
    xsltgeneric::generic_callable_posn_1(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
#[should_panic]
fn xslt_include() {
    xsltgeneric::generic_include(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_current() {
    xsltgeneric::generic_current(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_key_1() {
    xsltgeneric::generic_key_1(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_document_1() {
    xsltgeneric::generic_document_1(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_number_1() {
    xsltgeneric::generic_number_1(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_attr_set_1() {
    xsltgeneric::attr_set_1(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_attr_set_2() {
    xsltgeneric::attr_set_2(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_attr_set_3() {
    xsltgeneric::attr_set_3(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_feg_starting_with_1() {
    xsltgeneric::feg_starting_with_1(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_feg_starting_with_2() {
    xsltgeneric::feg_starting_with_2(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_issue_96_abs() {
    xsltgeneric::issue_96_abs(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_issue_96_rel() {
    xsltgeneric::issue_96_rel(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_issue_96_mixed() {
    xsltgeneric::issue_96_mixed(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_sd_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_issue_126() {
    xsltgeneric::issue_126(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_empty_doc_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_issue_137_1() {
    xsltgeneric::issue_137_1(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_empty_doc_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_issue_137_2() {
    xsltgeneric::issue_137_2(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_empty_doc_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_dbk_1() {
    xsltgeneric::dbk_1(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_empty_doc_cooked,
    )
    .expect("test failed")
}
#[test]
fn xslt_md_1() {
    xsltgeneric::md_1(
        smite::make_from_str,
        smite::make_from_str_with_ns,
        smite::make_empty_doc_cooked,
    )
    .expect("test failed")
}
