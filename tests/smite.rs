use std::collections::HashMap;
use xrust::item::{Node, NodeType};
use xrust::item_node_tests;
use xrust::item_value_tests;
use xrust::parser::xml::{parse as xmlparse, parse_with_ns};
use xrust::pattern_tests;
use xrust::qname::QualifiedName;
use xrust::transform::context::{Context, ContextBuilder, StaticContext, StaticContextBuilder};
use xrust::transform_tests;
use xrust::trees::smite::{Node as SmiteNode, RNode};
use xrust::xdmerror::{Error, ErrorKind};

mod xpathgeneric;
mod xsltgeneric;

type F = Box<dyn FnMut(&str) -> Result<(), Error>>;

fn make_empty_doc() -> RNode {
    Rc::new(SmiteNode::new())
}

fn make_doc(n: QualifiedName, v: Value) -> RNode {
    let mut d = Rc::new(SmiteNode::new());
    let mut child = d.new_element(n).expect("unable to create element");
    d.push(child.clone()).expect("unable to add element node");
    child
        .push(
            child
                .new_text(Rc::new(v))
                .expect("unable to create text node"),
        )
        .expect("unable to add text node");
    d
}

fn make_sd_raw() -> RNode {
    let doc = Rc::new(SmiteNode::new());
    xmlparse(doc.clone(),
             "<a id='a1'><b id='b1'><a id='a2'><b id='b2'/><b id='b3'/></a><a id='a3'><b id='b4'/><b id='b5'/></a></b><b id='b6'><a id='a4'><b id='b7'/><b id='b8'/></a><a id='a5'><b id='b9'/><b id='b10'/></a></b></a>",
             None, None).expect("unable to parse XML");
    doc
}
fn make_sd_cooked() -> Result<RNode, Error> {
    Ok(make_sd_raw())
}
fn make_sd() -> Item<RNode> {
    Item::Node(make_sd_raw())
}

fn make_from_str(s: &str) -> Result<RNode, Error> {
    let doc = Rc::new(SmiteNode::new());
    xmlparse(doc.clone(), s, None, None)?;
    Ok(doc)
}

fn make_from_str_with_ns(s: &str) -> Result<(RNode, Vec<HashMap<String, String>>), Error> {
    let doc = Rc::new(SmiteNode::new());
    let r = parse_with_ns(doc.clone(), s, None, None)?;
    Ok(r)
}

// XPath tests

#[test]
fn xpath_empty() {
    xpathgeneric::generic_empty::<RNode>().expect("test failed")
}
#[test]
fn xpath_step_1_pos() {
    xpathgeneric::generic_step_1_pos::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_step_2() {
    xpathgeneric::generic_step_2::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_step_wild_1() {
    xpathgeneric::generic_step_wild_1::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_step_wild_2() {
    xpathgeneric::generic_step_wild_2::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_path_1_pos() {
    xpathgeneric::generic_path_1_pos::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_path_1_neg() {
    xpathgeneric::generic_path_1_neg::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_path_2() {
    xpathgeneric::generic_path_2::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_generate_id() {
    xpathgeneric::generic_generate_id::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_union() {
    xpathgeneric::generic_union::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_intersectexcept() {
    xpathgeneric::generic_intersectexcept::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_instanceof() {
    xpathgeneric::generic_instanceof::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_treat() {
    xpathgeneric::generic_treat::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_castable() {
    xpathgeneric::generic_castable::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_cast() {
    xpathgeneric::generic_cast::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_arrow() {
    xpathgeneric::generic_arrow::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_unary() {
    xpathgeneric::generic_unary::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_simplemap() {
    xpathgeneric::generic_simplemap::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_int() {
    xpathgeneric::generic_int::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_decimal() {
    xpathgeneric::generic_decimal::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_exponent() {
    xpathgeneric::generic_exponent::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_string_apos() {
    xpathgeneric::generic_string_apos::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_string_apos_esc() {
    xpathgeneric::generic_string_apos_esc::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_string_quot() {
    xpathgeneric::generic_string_quot::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_string_quot_esc() {
    xpathgeneric::generic_string_quot_esc::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_literal_sequence() {
    xpathgeneric::generic_literal_sequence::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_literal_sequence_ws() {
    xpathgeneric::generic_literal_sequence_ws::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_comment() {
    xpathgeneric::generic_xpath_comment::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_context_item() {
    xpathgeneric::generic_xpath_context_item::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_parens_singleton() {
    xpathgeneric::generic_parens_singleton::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_root_desc_or_self_1() {
    xpathgeneric::generic_root_desc_or_self_1::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_root_desc_or_self_2() {
    xpathgeneric::generic_root_desc_or_self_2::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_root_desc_or_self_3() {
    xpathgeneric::generic_root_desc_or_self_3::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_rel_path_1() {
    xpathgeneric::generic_rel_path_1::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_rel_path_2() {
    xpathgeneric::generic_rel_path_2::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_fncall_string() {
    xpathgeneric::generic_fncall_string::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
#[should_panic]
fn xpath_fncall_current_1() {
    xpathgeneric::generic_fncall_current_1::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_current_2() {
    xpathgeneric::generic_fncall_current_2::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_current_3() {
    xpathgeneric::generic_fncall_current_3::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_concat() {
    xpathgeneric::generic_fncall_concat::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_startswith_pos() {
    xpathgeneric::generic_fncall_startswith_pos::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_startswith_neg() {
    xpathgeneric::generic_fncall_startswith_neg::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_contains_pos() {
    xpathgeneric::generic_fncall_contains_pos::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_contains_neg() {
    xpathgeneric::generic_fncall_contains_neg::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_substring_2arg() {
    xpathgeneric::generic_fncall_substring_2arg::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_substring_3arg() {
    xpathgeneric::generic_fncall_substring_3arg::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_substringbefore_pos() {
    xpathgeneric::generic_fncall_substringbefore_pos::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_substringbefore_neg() {
    xpathgeneric::generic_fncall_substringbefore_neg::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_substringafter_pos_1() {
    xpathgeneric::generic_fncall_substringafter_pos_1::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_substringafter_pos_2() {
    xpathgeneric::generic_fncall_substringafter_pos_2::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_substringafter_neg() {
    xpathgeneric::generic_fncall_substringafter_neg::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_normalizespace() {
    xpathgeneric::generic_fncall_normalizespace::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_translate() {
    xpathgeneric::generic_fncall_translate::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_boolean_true() {
    xpathgeneric::generic_fncall_boolean_true::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_boolean_false() {
    xpathgeneric::generic_fncall_boolean_false::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_not_true() {
    xpathgeneric::generic_fncall_not_true::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_not_false() {
    xpathgeneric::generic_fncall_not_false::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_true() {
    xpathgeneric::generic_fncall_true::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_fncall_false() {
    xpathgeneric::generic_fncall_false::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_fncall_number_int() {
    xpathgeneric::generic_fncall_number_int::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_number_double() {
    xpathgeneric::generic_fncall_number_double::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_sum() {
    xpathgeneric::generic_fncall_sum::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_fncall_floor() {
    xpathgeneric::generic_fncall_floor::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_fncall_ceiling() {
    xpathgeneric::generic_fncall_ceiling::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_round_down() {
    xpathgeneric::generic_fncall_round_down::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_round_up() {
    xpathgeneric::generic_fncall_round_up::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_count_1() {
    xpathgeneric::generic_fncall_count_1::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_count_2() {
    xpathgeneric::generic_fncall_count_2::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_user_defined() {
    xpathgeneric::generic_fncall_user_defined::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_let_1() {
    xpathgeneric::generic_let_1::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_let_2() {
    xpathgeneric::generic_let_2::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_for_1() {
    xpathgeneric::generic_for_1::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_for_2() {
    xpathgeneric::generic_for_2::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_if_1() {
    xpathgeneric::generic_if_1::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_if_2() {
    xpathgeneric::generic_if_2::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}
#[test]
fn xpath_sys_prop_vers_qual() {
    xpathgeneric::generic_sys_prop_vers_qual::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_sys_prop_product_vers() {
    xpathgeneric::generic_sys_prop_product_vers::<RNode, _, _>(make_empty_doc, make_sd)
        .expect("test failed")
}
#[test]
fn xpath_key_1() {
    xpathgeneric::generic_key_1::<RNode, _, _>(make_empty_doc, make_sd).expect("test failed")
}

// XSLT tests

#[test]
fn xslt_literal_text() {
    xsltgeneric::generic_literal_text(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}
#[test]
fn xslt_sys_prop() {
    xsltgeneric::generic_sys_prop(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}
#[test]
fn xslt_value_of_1() {
    xsltgeneric::generic_value_of_1(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}
#[test]
fn xslt_value_of_2() {
    xsltgeneric::generic_value_of_2(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}
#[test]
fn xslt_literal_element() {
    xsltgeneric::generic_literal_element(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}
#[test]
fn xslt_element() {
    xsltgeneric::generic_element(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}
#[test]
fn xslt_apply_templates_1() {
    xsltgeneric::generic_apply_templates_1(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}
#[test]
fn xslt_apply_templates_2() {
    xsltgeneric::generic_apply_templates_2(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}
#[test]
fn xslt_comment() {
    xsltgeneric::generic_comment(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}
#[test]
fn xslt_pi() {
    xsltgeneric::generic_pi(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}
#[test]
fn xslt_message_1() {
    xsltgeneric::generic_message_1(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}
#[test]
fn xslt_message_term() {
    xsltgeneric::generic_message_term(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}
#[test]
fn xslt_issue_58() {
    xsltgeneric::generic_issue_58(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}
#[test]
fn xslt_callable_named_1() {
    xsltgeneric::generic_callable_named_1(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}
#[test]
fn xslt_callable_posn_1() {
    xsltgeneric::generic_callable_posn_1(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}
#[test]
#[should_panic]
fn xslt_include() {
    xsltgeneric::generic_include(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}
#[test]
fn xslt_current() {
    xsltgeneric::generic_current(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}
#[test]
fn xslt_key_1() {
    xsltgeneric::generic_key_1(make_from_str, make_from_str_with_ns, make_sd_cooked)
        .expect("test failed")
}

item_value_tests!(RNode);
item_node_tests!(make_empty_doc, make_doc, make_sd_raw);
pattern_tests!(RNode, make_empty_doc, make_sd);
transform_tests!(RNode, make_empty_doc, make_doc);
