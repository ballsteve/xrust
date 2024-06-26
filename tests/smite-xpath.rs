// XPath tests

use xrust::trees::smite::RNode;

mod smite;
mod xpathgeneric;

#[test]
fn xpath_empty() {
    xpathgeneric::generic_empty::<RNode>().expect("test failed")
}
#[test]
fn xpath_step_1_pos() {
    xpathgeneric::generic_step_1_pos::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_step_2() {
    xpathgeneric::generic_step_2::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_step_wild_1() {
    xpathgeneric::generic_step_wild_1::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_step_wild_2() {
    xpathgeneric::generic_step_wild_2::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_path_1_pos() {
    xpathgeneric::generic_path_1_pos::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_path_1_neg() {
    xpathgeneric::generic_path_1_neg::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_path_2() {
    xpathgeneric::generic_path_2::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_generate_id() {
    xpathgeneric::generic_generate_id::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_union() {
    xpathgeneric::generic_union::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_intersectexcept() {
    xpathgeneric::generic_intersectexcept::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_instanceof() {
    xpathgeneric::generic_instanceof::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_treat() {
    xpathgeneric::generic_treat::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_castable() {
    xpathgeneric::generic_castable::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_cast() {
    xpathgeneric::generic_cast::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_arrow() {
    xpathgeneric::generic_arrow::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_unary() {
    xpathgeneric::generic_unary::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_simplemap() {
    xpathgeneric::generic_simplemap::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_int() {
    xpathgeneric::generic_int::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_decimal() {
    xpathgeneric::generic_decimal::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_exponent() {
    xpathgeneric::generic_exponent::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_string_apos() {
    xpathgeneric::generic_string_apos::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_string_apos_esc() {
    xpathgeneric::generic_string_apos_esc::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_string_quot() {
    xpathgeneric::generic_string_quot::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_string_quot_esc() {
    xpathgeneric::generic_string_quot_esc::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_literal_sequence() {
    xpathgeneric::generic_literal_sequence::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_literal_sequence_ws() {
    xpathgeneric::generic_literal_sequence_ws::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_comment() {
    xpathgeneric::generic_xpath_comment::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_context_item() {
    xpathgeneric::generic_xpath_context_item::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_parens_singleton() {
    xpathgeneric::generic_parens_singleton::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_root_desc_or_self_1() {
    xpathgeneric::generic_root_desc_or_self_1::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_root_desc_or_self_2() {
    xpathgeneric::generic_root_desc_or_self_2::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_root_desc_or_self_3() {
    xpathgeneric::generic_root_desc_or_self_3::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_rel_path_1() {
    xpathgeneric::generic_rel_path_1::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_rel_path_2() {
    xpathgeneric::generic_rel_path_2::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_string() {
    xpathgeneric::generic_fncall_string::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
#[should_panic]
fn xpath_fncall_current_1() {
    xpathgeneric::generic_fncall_current_1::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_current_2() {
    xpathgeneric::generic_fncall_current_2::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_current_3() {
    xpathgeneric::generic_fncall_current_3::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_concat() {
    xpathgeneric::generic_fncall_concat::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_startswith_pos() {
    xpathgeneric::generic_fncall_startswith_pos::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
        .expect("test failed")
}
#[test]
fn xpath_fncall_startswith_neg() {
    xpathgeneric::generic_fncall_startswith_neg::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
        .expect("test failed")
}
#[test]
fn xpath_fncall_contains_pos() {
    xpathgeneric::generic_fncall_contains_pos::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_contains_neg() {
    xpathgeneric::generic_fncall_contains_neg::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_substring_2arg() {
    xpathgeneric::generic_fncall_substring_2arg::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
        .expect("test failed")
}
#[test]
fn xpath_fncall_substring_3arg() {
    xpathgeneric::generic_fncall_substring_3arg::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
        .expect("test failed")
}
#[test]
fn xpath_fncall_substringbefore_pos() {
    xpathgeneric::generic_fncall_substringbefore_pos::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
        .expect("test failed")
}
#[test]
fn xpath_fncall_substringbefore_neg() {
    xpathgeneric::generic_fncall_substringbefore_neg::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
        .expect("test failed")
}
#[test]
fn xpath_fncall_substringafter_pos_1() {
    xpathgeneric::generic_fncall_substringafter_pos_1::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
        .expect("test failed")
}
#[test]
fn xpath_fncall_substringafter_pos_2() {
    xpathgeneric::generic_fncall_substringafter_pos_2::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
        .expect("test failed")
}
#[test]
fn xpath_fncall_substringafter_neg() {
    xpathgeneric::generic_fncall_substringafter_neg::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
        .expect("test failed")
}
#[test]
fn xpath_fncall_normalizespace() {
    xpathgeneric::generic_fncall_normalizespace::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
        .expect("test failed")
}
#[test]
fn xpath_fncall_translate() {
    xpathgeneric::generic_fncall_translate::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_boolean_true() {
    xpathgeneric::generic_fncall_boolean_true::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_boolean_false() {
    xpathgeneric::generic_fncall_boolean_false::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_not_true() {
    xpathgeneric::generic_fncall_not_true::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_not_false() {
    xpathgeneric::generic_fncall_not_false::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_true() {
    xpathgeneric::generic_fncall_true::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_false() {
    xpathgeneric::generic_fncall_false::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_number_int() {
    xpathgeneric::generic_fncall_number_int::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_number_double() {
    xpathgeneric::generic_fncall_number_double::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_sum() {
    xpathgeneric::generic_fncall_sum::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_floor() {
    xpathgeneric::generic_fncall_floor::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_ceiling() {
    xpathgeneric::generic_fncall_ceiling::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_round_down() {
    xpathgeneric::generic_fncall_round_down::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_round_up() {
    xpathgeneric::generic_fncall_round_up::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_count_1() {
    xpathgeneric::generic_fncall_count_1::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_count_2() {
    xpathgeneric::generic_fncall_count_2::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_fncall_user_defined() {
    xpathgeneric::generic_fncall_user_defined::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_let_1() {
    xpathgeneric::generic_let_1::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_let_2() {
    xpathgeneric::generic_let_2::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_for_1() {
    xpathgeneric::generic_for_1::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_for_2() {
    xpathgeneric::generic_for_2::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_if_1() {
    xpathgeneric::generic_if_1::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_if_2() {
    xpathgeneric::generic_if_2::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_sys_prop_vers_qual() {
    xpathgeneric::generic_sys_prop_vers_qual::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_sys_prop_product_vers() {
    xpathgeneric::generic_sys_prop_product_vers::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
        .expect("test failed")
}
#[test]
fn xpath_key_1() {
    xpathgeneric::generic_key_1::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn xpath_document_1() {
    xpathgeneric::generic_document_1::<RNode, _, _, _>(smite::make_empty_doc, smite::make_sd, smite::make_from_str)
        .expect("test failed")
}

