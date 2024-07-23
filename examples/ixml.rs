//! Integrating Invisible XML with Xrust.
//!
//! This example accepts to input files: an XSL Stylesheet and a Markdown document.
//! It uses Invisible XML to parse the Markdown document into a document for Xrust.
//! It then performs the XSL Transformation on the Markdown document
//! and writes the result as an XML document.

extern crate earleybird;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;
use url::Url;

use xrust::item::{Item, Node, NodeType, SequenceTrait};
use xrust::parser::xml::parse;
use xrust::qname::QualifiedName;
use xrust::transform::context::{StaticContext, StaticContextBuilder};
use xrust::trees::smite::{Node as SmiteNode, RNode};
use xrust::value::Value;
use xrust::xdmerror::{Error, ErrorKind};
use xrust::xslt::from_document;

//use earleybird::grammar::Grammar;
use earleybird::ixml_grammar::{ixml_grammar, ixml_tree_to_grammar};
use earleybird::parser::{Content, Parser};
use indextree::{Arena, NodeId};

// A quick-and-dirty converter from an indextree to an Xrust intmuttree RNode.
// A better solution will be to define a trait in IXML that is used to build the tree directly.
fn to_rnode(arena: &Arena<Content>) -> RNode {
    let t = Rc::new(SmiteNode::new());
    let root = arena.iter().next().unwrap();
    let root_id = arena.get_node_id(root).unwrap();
    for child in root_id.children(arena) {
        to_rnode_aux(arena, child, t.clone())
    }
    t
}
fn to_rnode_aux(arena: &Arena<Content>, n: NodeId, mut t: RNode) {
    if let Some(m) = arena.get(n) {
        match m.get() {
            Content::Root => {
                for child in n.children(arena) {
                    to_rnode_aux(arena, child, t.clone())
                }
            }
            Content::Element(name) => {
                let new = t
                    .new_element(QualifiedName::new(None, None, name.clone()))
                    .expect("unable to create element node");
                t.push(new.clone()).expect("unable to append node");
                for attr in n
                    .children(arena)
                    .filter(|a| arena.get(*a).unwrap().get().is_attr())
                {
                    if let Content::Attribute(attr_name, attr_value) =
                        arena.get(attr).unwrap().get()
                    {
                        let new_attr = t
                            .new_attribute(
                                QualifiedName::new(None, None, attr_name.clone()),
                                Rc::new(Value::from(attr_value.clone())),
                            )
                            .expect("unable to create attribute node");
                        t.add_attribute(new_attr).expect("unable to append node");
                    }
                }
                for child in n.children(arena) {
                    to_rnode_aux(arena, child, new.clone())
                }
            }
            Content::Attribute(name, value) => {
                let new = t
                    .new_attribute(
                        QualifiedName::new(None, None, name.clone()),
                        Rc::new(Value::from(value.clone())),
                    )
                    .expect("unable to create attribute node");
                t.add_attribute(new).expect("unable to append node");
            }
            Content::Text(value) => {
                let new = t
                    .new_text(Rc::new(Value::from(value.clone())))
                    .expect("unable to create text node");
                t.push(new).expect("unable to append node");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} stylesheet source", args[0]);
        return;
    }

    // Read the XSL Stylesheet document as XML
    let stylepath = Path::new(&args[1]);
    let mut stylefile = match File::open(&stylepath) {
        Err(why) => {
            panic!(
                "unable to open stylesheet \"{}\" due to \"{}\"",
                &args[1], why
            )
        }
        Ok(f) => f,
    };
    let mut stylexml = String::new();
    match stylefile.read_to_string(&mut stylexml) {
        Err(why) => panic!("unable to read from \"{}\" due to \"{}\"", &args[1], why),
        Ok(_) => {}
    };

    let style = Rc::new(SmiteNode::new());
    parse(style.clone(), stylexml.trim(), None).expect("failed to parse XSL stylesheet");

    // Read the Markdown text file
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
    let mut srcmd = String::new();
    match srcfile.read_to_string(&mut srcmd) {
        Err(why) => panic!("unable to read from \"{}\" due to \"{}\"", &args[2], why),
        Ok(_) => {}
    };

    // This is a grammar for simple Markdown documents.
    // Just headings and paragraphs.
    /*    let _gdbg = ixml_str_to_grammar(r###"doc = heading1, para+, heading2, para+.

    heading1 = -"#", -" ", ~[#a]*, ws.
    heading2 = -"#", -"#", -" ", ~[#a]*, ws.
    para = ~['#'], ~[#a]+, lf.
    -ws = (lf; cr)+.
    -lf = #a.
    -cr = #d.
    "###).expect("unable to parse grammar");

     */
    // Let's try something simpler for now
    let g = ixml_grammar();
    let ixml = r###"doc = para+.
para = letter+, eol?.
eol = "X".
-letter = " "|"a"|"b"|"c"|"A"|"B"|"C"."###;
    /*    let ixml = r###"doc = heading1, para+, heading2, para+.

    heading1 = -"#", -" ", ~[#a]*, ws.
    heading2 = -"#", -"#", -" ", ~[#a]*, ws.
    para = ~['#'], ~[#a]+, lf.
    -ws = (lf; cr)+.
    -lf = #a.
    -cr = #d.
    "###;

     */
    let mut parser = Parser::new(g);
    let arena = parser.parse(ixml).expect("unable to parse grammar");
    let gen_grammar = ixml_tree_to_grammar(&arena);
    let mut gen_parser = Parser::new(gen_grammar);

    // Now parse the Markdown document. IXML creates a temporary tree structure.
    //let md_arena = gen_parser.parse(&srcmd).expect("unable to parse input");
    let md_arena = gen_parser
        .parse("BaC caaa B bAcXabcX")
        .expect("unable to parse input");
    // Translate the temporary tree into an Xrust RNode
    let md = to_rnode(&md_arena);

    // Now compile the XSL Stylesheet
    let pwd = std::env::current_dir().expect("unable to get current directory");
    let pwds = pwd
        .into_os_string()
        .into_string()
        .expect("unable to convert pwd");
    let mut ctxt = from_document(
        style,
        vec![],
        Some(
            Url::parse(format!("file://{}/{}", pwds, &args[1]).as_str())
                .expect("unable to parse stylesheet URL"),
        ),
        |_| {
            Err(Error::new(
                ErrorKind::Unknown,
                String::from("loading resources not implemented"),
            ))
        },
        |_| {
            Err(Error::new(
                ErrorKind::Unknown,
                String::from("loading external resources not implemented"),
            ))
        },
    )
    .expect("failed to compile XSL stylesheet");

    // Set the Markdown RNode document as the context
    ctxt.context(vec![Item::Node(md)], 0);
    // Create a document for the result tree
    ctxt.result_document(Rc::new(SmiteNode::new()));

    // Create a static transformation contact
    // with dummy callbacks
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        .build();

    // Let 'er rip!
    let resultdoc = ctxt
        .evaluate(&mut stctxt)
        .expect("failed to evaluate stylesheet");

    // Serialise the result document as XML
    println!("{}", resultdoc.to_xml());
    // If you want pretty-printing then you would specify an OutputDefinition.
    //    println!("{}", resultdoc.to_xml_with_options(&dc.get_output_definition()));
}
