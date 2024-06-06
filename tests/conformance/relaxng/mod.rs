mod jamesclark;

/*
use std::fs;
use std::rc::Rc;
use xrust::Node;
use xrust::parser::xml;
use xrust::trees::smite::{Node as SmiteNode};
use xrust::validators::relaxng::validate_relaxng;

#[test]
fn rngtestone(){

    let s = "<?xml version=\"1.0\" encoding=\"utf-8\"?>
<element xmlns=\"http://relaxng.org/ns/structure/1.0\" name=\"foo\" ns=\"\">
    <empty/>
        </element>";

    let d = "<?xml version=\"1.0\" encoding=\"utf-8\"?>
<foo/>
";

    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d, None, None);

    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), s, None, None);


    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());
}
 */
