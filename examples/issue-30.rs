//! Issue #30. An example using XPath, but not XSLT.
//!
//! Suggested by Micah Dubinko.

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use xrust::ErrorKind;

use xrust::item::{Item, Node, SequenceTrait};
use xrust::parser::xml::parse as xmlparse;
use xrust::parser::xpath::parse;
use xrust::transform::context::{ContextBuilder, StaticContextBuilder};
use xrust::trees::smite::RNode;
use xrust::xdmerror::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} xpath xml", args[0]);
        return;
    }

    // Read the XPath expression.
    // A future version could read each line as an expression.
    let exprpath = Path::new(&args[1]);
    let mut exprfile = match File::open(&exprpath) {
        Err(why) => {
            panic!(
                "unable to open XPath expression file \"{}\" due to \"{}\"",
                &args[1], why
            )
        }
        Ok(f) => f,
    };
    let mut expr = String::new();
    match exprfile.read_to_string(&mut expr) {
        Err(why) => panic!("unable to read from \"{}\" due to \"{}\"", &args[1], why),
        Ok(_) => {}
    };
    // Parse the XPath expression
    let xpath = parse::<RNode>(expr.trim(), None).expect("XPath expression not recognised");

    // Read the XML file
    let srcpath = Path::new(&args[2]);
    let mut srcfile = match File::open(&srcpath) {
        Err(why) => {
            panic!(
                "unable to open source document \"{}\" due to \"{}\"",
                &args[2], why
            )
        }
        Ok(f) => f,
    };
    let mut srcxml = String::new();
    match srcfile.read_to_string(&mut srcxml) {
        Err(why) => panic!("unable to read from \"{}\" due to \"{}\"", &args[2], why),
        Ok(_) => {}
    };
    // Parse the XML into a RNode
    let root = RNode::new_document();
    xmlparse(root.clone(), srcxml.as_str(), None).expect("unable to parse XML");

    // Create a dynamic transformation context
    let context = ContextBuilder::new()
        .context(vec![Item::Node(root)])
        .build();
    // Create a static transformation contact
    // with dummy callbacks
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();

    // Let 'er rip!
    let result = context
        .dispatch(&mut stctxt, &xpath)
        .expect("failed to evaluate XPath");

    // Serialise the result document as XML
    println!("{}", result.to_xml());
    // If you want pretty-printing then you would specify an OutputDefinition.
    //    println!("{}", resultdoc.to_xml_with_options(&dc.get_output_definition()));
}
