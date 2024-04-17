//! Issue #30. An example using XPath, but not XSLT.
//!
//! Suggested by Micah Dubinko.

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use xrust::item::{Item, Node, NodeType, SequenceTrait};
use xrust::parser::xpath::parse;
use xrust::transform::context::{ContextBuilder, StaticContext};
use xrust::trees::intmuttree::{Document, NodeBuilder, RNode};
use xrust::xdmerror::Error;

type F = Box<dyn FnMut(&str) -> Result<(), Error>>;

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
    let xpath = parse::<RNode>(expr.trim()).expect("XPath expression not recognised");

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
    // content[0] is the root node
    let root = Document::try_from((srcxml, None, None))
        .expect("unable to parse XML")
        .content[0]
        .clone();
    let mut doc = NodeBuilder::new(NodeType::Document).build();
    doc.push(root).expect("unable to append root node");

    // Create a transformation context
    let context = ContextBuilder::new().current(vec![Item::Node(doc)]).build();

    // Let 'er rip!
    let result = context
        .dispatch(&mut StaticContext::<F>::new(), &xpath)
        .expect("failed to evaluate XPath");

    // Serialise the result document as XML
    println!("{}", result.to_xml());
    // If you want pretty-printing then you would specify an OutputDefinition.
    //    println!("{}", resultdoc.to_xml_with_options(&dc.get_output_definition()));
}
