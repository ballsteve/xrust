#![feature(test)]
#![feature(associated_type_defaults)]

extern crate test;

use std::fs::File;
use std::path::Path;
use std::io::Read;
use xrust::xdmerror::*;
use xrust::output::*;
use xrust::qname::*;
use xrust::value::*;
use xrust::rctree::*;

#[cfg(test)]
mod tests {
    use test::Bencher;

    #[bench]
    fn bench_anode(b: &mut Bencher) {
	b.iter(|| {
	    let mut a = ANodeBuilder::new(NodeType::Document)
		.build();
	    let b = ANodeBuilder::new(NodeType::Element)
		.name(QualifiedName::new(None, None, String::from("Test")))
		.build();
	    a.push(b)
		.expect("unable to append toplevel element");
	    (1..3).for_each(|i| {
		let l1 = ANodeBuilder::new(NodeType::Element)
		    .name(QualifiedName::new(None, None, String::from("Level-1")))
		    .build();
		b.push(l1)
		    .expect("unable to append level-1 element");
		(1..3).for_each(|k| {
		    let l2 = ANodeBuilder::new(NodeType::Element)
			.name(QualifiedName::new(None, None, String::from("Level-2")))
			.build();
		    l1.push(l2)
			.expect("unable to append level-2 element");
		    l2.push(
			ANodeBuilder::new(NodeType::Text)
			    .value(Value::from(format!("node {}-{}", i, k)))
			    .build()
		    ).expect("unable to create text node");
		});
	    });
	})
    }
}
