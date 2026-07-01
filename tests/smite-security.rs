// Smite tests for security-related features

mod security;
mod smite;

#[test]
fn security_max_depth_np_1() {
    security::max_depth_np_1(smite::make_empty_doc).expect("test failed")
}
#[test]
fn security_max_depth_np_2() {
    security::max_depth_np_2(smite::make_empty_doc).expect("test failed")
}
#[test]
fn security_max_depth_pol_none() {
    security::max_depth_pol_none(smite::make_empty_doc).expect("test failed")
}
#[test]
fn security_max_depth_pol_set_1() {
    security::max_depth_pol_set_1(smite::make_empty_doc).expect("test failed")
}
#[test]
fn security_max_depth_pol_set_2() {
    security::max_depth_pol_set_2(smite::make_empty_doc).expect("test failed")
}
#[test]
fn security_max_depth_callable_1() {
    security::max_depth_callable_1(smite::make_empty_doc, smite::make_from_str)
        .expect("test failed")
}
#[test]
fn security_feature() {
    security::sec_feature(smite::make_empty_doc).expect("test failed")
}
#[test]
fn security_from_1() {
    security::sec_from_1(smite::make_empty_doc, smite::make_from_str).expect("test failed")
}
