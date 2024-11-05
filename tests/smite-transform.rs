// Transform tests

use xrust::trees::smite::RNode;

mod smite;
mod transformgeneric;

#[test]
fn tr_empty() {
    transformgeneric::generic_tr_empty::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_singleton_literal() {
    transformgeneric::generic_tr_singleton_literal::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_literal_element() {
    transformgeneric::generic_tr_literal_element::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_literal_element_nested() {
    transformgeneric::generic_tr_literal_element_nested::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_element() {
    transformgeneric::generic_tr_element::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_literal_text_1() {
    transformgeneric::generic_tr_literal_text_1::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_literal_text_2() {
    transformgeneric::generic_tr_literal_text_2::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_literal_attribute() {
    transformgeneric::generic_tr_literal_attribute::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_literal_comment() {
    transformgeneric::generic_tr_literal_comment::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_literal_pi() {
    transformgeneric::generic_tr_literal_pi::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_generate_id_ctxt() {
    transformgeneric::generic_tr_generate_id_ctxt::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_generate_id_2() {
    transformgeneric::generic_tr_generate_id_2::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_message_1() {
    transformgeneric::generic_tr_message_1::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_message_2() {
    transformgeneric::generic_tr_message_2::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_message_term_1() {
    transformgeneric::generic_tr_message_term_1::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_set_attribute() {
    transformgeneric::generic_tr_set_attribute::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_copy_literal() {
    transformgeneric::generic_tr_copy_literal::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_copy_context_literal() {
    transformgeneric::generic_tr_copy_context_literal::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_copy_context_node() {
    transformgeneric::generic_tr_copy_context_node::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_current_node() {
    transformgeneric::generic_tr_current_node::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_deep_copy() {
    transformgeneric::generic_tr_deep_copy::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_seq_of_literals() {
    transformgeneric::generic_tr_seq_of_literals::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_seq_of_seqs() {
    transformgeneric::generic_tr_seq_of_seqs::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_switch_when() {
    transformgeneric::generic_tr_switch_when::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_switch_otherwise() {
    transformgeneric::generic_tr_switch_otherwise::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_loop_lit() {
    transformgeneric::generic_tr_loop_lit::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_context_item() {
    transformgeneric::generic_tr_context_item::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_context_item_seq() {
    transformgeneric::generic_tr_context_item_seq::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_root() {
    transformgeneric::generic_tr_root::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_path_of_lits() {
    transformgeneric::generic_tr_path_of_lits::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_step_child_1() {
    transformgeneric::generic_tr_step_child_1::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_step_child_many() {
    transformgeneric::generic_tr_step_child_many::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_step_self() {
    transformgeneric::generic_tr_step_self::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_step_selfdoc_pos() {
    transformgeneric::generic_tr_step_selfdoc_pos::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_step_selfdoc_neg() {
    transformgeneric::generic_tr_step_selfdoc_neg::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_step_parent() {
    transformgeneric::generic_tr_step_parent::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_step_parentdoc_pos() {
    transformgeneric::generic_tr_step_parentdoc_pos::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_step_parentdoc_neg() {
    transformgeneric::generic_tr_step_parentdoc_neg::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_step_descendant() {
    transformgeneric::generic_tr_step_descendant::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_step_descendant_or_self() {
    transformgeneric::generic_tr_step_descendant_or_self::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_step_descendant_or_self_or_root() {
    transformgeneric::generic_tr_step_descendant_or_self_or_root::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_step_ancestor() {
    transformgeneric::generic_tr_step_ancestor::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_step_ancestor_or_self() {
    transformgeneric::generic_tr_step_ancestor_or_self::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_step_following_sibling() {
    transformgeneric::generic_tr_step_following_sibling::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_step_preceding_sibling() {
    transformgeneric::generic_tr_step_preceding_sibling::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_step_following() {
    transformgeneric::generic_tr_step_following::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_step_preceding() {
    transformgeneric::generic_tr_step_preceding::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_path_step_child() {
    transformgeneric::generic_tr_path_step_child::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_step_attribute() {
    transformgeneric::generic_tr_step_attribute::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_step_self_attribute_pos() {
    transformgeneric::generic_tr_step_self_attribute_pos::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_step_self_attribute_neg() {
    transformgeneric::generic_tr_step_self_attribute_neg::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_predicate() {
    transformgeneric::generic_tr_predicate::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_or_true() {
    transformgeneric::generic_tr_or_true::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_or_false() {
    transformgeneric::generic_tr_or_false::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_and_true() {
    transformgeneric::generic_tr_and_true::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_and_false() {
    transformgeneric::generic_tr_and_false::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_general_compare_true() {
    transformgeneric::generic_tr_general_compare_true::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_general_compare_false() {
    transformgeneric::generic_tr_general_compare_false::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_value_compare_true() {
    transformgeneric::generic_tr_value_compare_true::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_value_compare_false() {
    transformgeneric::generic_tr_value_compare_false::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_range_empty() {
    transformgeneric::generic_tr_range_empty::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_range_many() {
    transformgeneric::generic_tr_range_many::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_range_one() {
    transformgeneric::generic_tr_range_one::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_arithmetic_add() {
    transformgeneric::generic_tr_arithmetic_add::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_var_declare() {
    transformgeneric::generic_tr_var_declare::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_union() {
    transformgeneric::generic_tr_union::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_for_each() {
    transformgeneric::generic_tr_for_each::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_for_each_sort_1() {
    transformgeneric::generic_tr_for_each_sort::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_group_by_1() {
    transformgeneric::generic_tr_group_by_1::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_group_by_sort_1() {
    transformgeneric::generic_tr_group_by_sort_1::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_group_adjacent_1() {
    transformgeneric::generic_tr_group_adjacent_1::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_group_adjacent_sort_1() {
    transformgeneric::generic_tr_group_adjacent_sort_1::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_apply_templates_builtins() {
    transformgeneric::generic_tr_apply_templates_builtins::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_apply_templates_1() {
    transformgeneric::generic_tr_apply_templates_1::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_apply_templates_2() {
    transformgeneric::generic_tr_apply_templates_2::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_apply_templates_3() {
    transformgeneric::generic_tr_apply_templates_3::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_apply_templates_import() {
    transformgeneric::generic_tr_apply_templates_import::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_apply_templates_next_match() {
    transformgeneric::generic_tr_apply_templates_next_match::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_apply_templates_mode() {
    transformgeneric::generic_tr_apply_templates_mode::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_apply_templates_sort_1() {
    transformgeneric::generic_tr_apply_templates_sort_1::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_position() {
    transformgeneric::generic_tr_position::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_last() {
    transformgeneric::generic_tr_last::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_count_0() {
    transformgeneric::generic_tr_count_0::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_count_1() {
    transformgeneric::generic_tr_count_1::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_localname_0() {
    transformgeneric::generic_tr_localname_0::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_name_0() {
    transformgeneric::generic_tr_name_0::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_string() {
    transformgeneric::generic_tr_string::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_concat_literal() {
    transformgeneric::generic_tr_concat_literal::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_starts_with_pos() {
    transformgeneric::generic_tr_starts_with_pos::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_starts_with_neg() {
    transformgeneric::generic_tr_starts_with_neg::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_contains_pos() {
    transformgeneric::generic_tr_contains_pos::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_contains_neg() {
    transformgeneric::generic_tr_contains_neg::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_substring_2args() {
    transformgeneric::generic_tr_substring_2args::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_substring_3args() {
    transformgeneric::generic_tr_substring_3args::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_substring_before() {
    transformgeneric::generic_tr_substring_before::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_substring_after() {
    transformgeneric::generic_tr_substring_after::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_normalize_space_1() {
    transformgeneric::generic_tr_normalize_space_1::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_translate_1() {
    transformgeneric::generic_tr_translate_1::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_boolean_string_pos() {
    transformgeneric::generic_tr_boolean_string_pos::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_boolean_string_neg() {
    transformgeneric::generic_tr_boolean_string_neg::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_boolean_int_pos() {
    transformgeneric::generic_tr_boolean_int_pos::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_boolean_int_neg() {
    transformgeneric::generic_tr_boolean_int_neg::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_not_pos() {
    transformgeneric::generic_tr_not_pos::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_not_neg() {
    transformgeneric::generic_tr_not_neg::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_true_literal() {
    transformgeneric::generic_tr_true_literal::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_false_literal() {
    transformgeneric::generic_tr_false_literal::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_number() {
    transformgeneric::generic_tr_number::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_sum() {
    transformgeneric::generic_tr_sum::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_avg() {
    transformgeneric::generic_tr_avg::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_min() {
    transformgeneric::generic_tr_min::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_max() {
    transformgeneric::generic_tr_max::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_floor() {
    transformgeneric::generic_tr_floor::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_ceiling() {
    transformgeneric::generic_tr_ceiling::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_round_1() {
    transformgeneric::generic_tr_round_1::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_round_2() {
    transformgeneric::generic_tr_round_2::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_current_date_time() {
    transformgeneric::generic_tr_current_date_time::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_current_date() {
    transformgeneric::generic_tr_current_date::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_current_time() {
    transformgeneric::generic_tr_current_time::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_format_date_time() {
    transformgeneric::generic_tr_format_date_time::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_format_date() {
    transformgeneric::generic_tr_format_date::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_format_time() {
    transformgeneric::generic_tr_format_time::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_format_number_1() {
    transformgeneric::generic_tr_format_number_1::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_key_1() {
    transformgeneric::generic_tr_key_1::<RNode, _, _>(smite::make_empty_doc, smite::make_sd)
        .expect("test failed")
}
#[test]
fn tr_callable_named_1() {
    transformgeneric::generic_tr_callable_named_1::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_callable_positional_1() {
    transformgeneric::generic_tr_callable_positional_1::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
    )
    .expect("test failed")
}
#[test]
fn tr_document_1() {
    transformgeneric::generic_tr_document_1::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
        Box::new(smite::make_from_str),
    )
    .expect("test failed")
}
#[test]
fn tr_generate_ints_1() {
    transformgeneric::generic_tr_generate_ints_1::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
        Box::new(smite::make_from_str),
    )
    .expect("test failed")
}
#[test]
fn tr_format_int_1() {
    transformgeneric::generic_tr_format_ints_1::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
        Box::new(smite::make_from_str),
    )
    .expect("test failed")
}
#[test]
fn tr_format_int_2() {
    transformgeneric::generic_tr_format_ints_2::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
        Box::new(smite::make_from_str),
    )
    .expect("test failed")
}
#[test]
fn tr_format_int_3() {
    transformgeneric::generic_tr_format_ints_3::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
        Box::new(smite::make_from_str),
    )
    .expect("test failed")
}
#[test]
fn tr_format_int_4() {
    transformgeneric::generic_tr_format_ints_4::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
        Box::new(smite::make_from_str),
    )
    .expect("test failed")
}
#[test]
fn tr_format_int_5() {
    transformgeneric::generic_tr_format_ints_5::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
        Box::new(smite::make_from_str),
    )
    .expect("test failed")
}
#[test]
fn tr_format_int_6() {
    transformgeneric::generic_tr_format_ints_6::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
        Box::new(smite::make_from_str),
    )
    .expect("test failed")
}
#[test]
fn tr_format_int_7() {
    transformgeneric::generic_tr_format_ints_7::<RNode, _, _>(
        smite::make_empty_doc,
        smite::make_sd,
        Box::new(smite::make_from_str),
    )
    .expect("test failed")
}
