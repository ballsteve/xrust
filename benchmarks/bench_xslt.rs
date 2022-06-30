#![feature(test)]
#![feature(associated_type_defaults)]

extern crate test;

use std::fs::File;
use std::path::Path;
use std::io::Read;
use crate::xdmerror::*;
use crate::output::*;
use crate::qname::*;
use crate::value::*;
use crate::forest::*;
use crate::evaluate::*;
use crate::xslt::*;

#[cfg(test)]
mod tests {
    use test::Bencher;

    #[bench]
    fn bench_identity(b: &mut Bencher) {
	let stylepath = Path::new("tests/xsl/identity.xsl");
	let mut stylefile = match File::open(&stylepath) {
	    Ok(f) => f,
	    Err(why) => {
		panic!("unable to open stylesheet due to \"{}\"", why)
	    }
	};
	let mut stylexmlraw = String::new();
	match stylefile.read_to_string(&mut stylexmlraw) {
	    Ok(f) => f,
	    Err(why) => {
		panic!("unable to read stylesheet due to \"{}\"", why)
	    }
	};
	let stylexml = stylexmlraw.trim();

	for x in vec!["1K.xml", "10K.xml", "100K.xml"] {
	    let xmlname = format!("tests/xml/{}", x);
	    let xmlpath = Path::new(xmlname.as_str());
	    let mut xmlfile = match File::open(&xmlpath) {
		Ok(f) => f,
		Err(why) => {
		    panic!("unable to open XML due to \"{}\"", why)
		}
	    };
	    let mut xmldataraw = String::new();
	    match xmlfile.read_to_string(&mut xmldataraw) {
		Ok(f) => f,
		Err(why) => {
		    panic!("unable to read XML due to \"{}\"", why)
		}
	    };
	    let xmldata = xmldataraw.trim();

	    b.iter(|| {
		let mut sc = StaticContext::new_with_xslt_builtins();
		let mut f = Forest::new();
		let src = f.grow_tree(xmldata)
		    .expect("unable to parse XML");
		let isrc = Rc::new(Item::Node(f.get_ref(src).unwrap().get_doc_node()));
		let style = f.grow_tree(stylexml)
		    .expect("unable to parse stylesheet");
		let ev = from_document(
		    &mut f,
		    style,
		    &mut sc,
		    None,
		)
		    .expect("failed to compile stylesheet");
		ev.dump_templates();
		let rd = f.plant_tree();
		let t = ev.find_match(&isrc, &mut f, src, rd, None)
		    .expect("unable to find match");
		ev.evaluate(Some(vec![Rc::clone(&isrc)]), Some(0), &t, &mut f, src, rd)
		    .expect("evaluation failed");
	    })
	}
    }
}
