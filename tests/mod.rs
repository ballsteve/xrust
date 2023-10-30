
mod conformance;
mod intmuttree;

//use std::convert::TryFrom;
//use std::fs;
//use xrust::{Document, Error};

/*
#[test]
#[ignore]
fn bigfile() {
    /* A million elements, each with an attribute and value */

    let testxml =
        Document::try_from((fs::read_to_string("tests/xml/45M.xml").unwrap(), None, None));

    assert!(testxml.is_ok());
}
 */

/*
#[test]
#[ignore]
fn testfile() {

    let testxml = Document::try_from(
        fs::read_to_string("tests/xml/test.xml").unwrap()
    );

    match testxml {
        Ok(_) => {println!("OK")}
        Err(e) => {
            println!("{:?}-{:?}", e.kind, e.message)
        }
    }

    //assert!(testxml.is_ok());
}


 */
