mod conformance;
mod intmuttree;

use std::convert::TryFrom;
use std::fs;
use xrust::Document;

#[test]
#[ignore]
fn bigfile() {
    /* A million elements, each with an arrtribue and value */

    let testxml = Document::try_from(
        fs::read_to_string("tests/xml/45M.xml").unwrap()
    );

    assert!(testxml.is_ok());
}
