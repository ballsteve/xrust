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
    fn bench_tree(b: &mut Bencher) {
	b.iter(|| {
	    let mut f = Forest::new();
	    let ti = f.plant_tree();
	    let r = f.get_ref_mut(ti).unwrap()
		.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element node");
	    f.get_ref_mut(ti).unwrap()
		.push_doc_node(r)
		.expect("unable to add doc node");
	    (1..3).for_each(|i| {
		let j = f.get_ref_mut(ti).unwrap()
		    .new_element(QualifiedName::new(None, None, String::from("Level-1")))
		    .expect("unable to create element node");
		r.append_child(&mut f, j).expect("unable to append node");
		(1..3).for_each(|k| {
		    let l = f.get_ref_mut(ti).unwrap()
			.new_element(QualifiedName::new(None, None, String::from("Level-2")))
			.expect("unable to create element node");
		    j.append_child(&mut f, l).expect("unable to append node");
		    let m = f.get_ref_mut(ti).unwrap()
			.new_text(Value::from(format!("node {}-{}", i, k)))
			.expect("unable to create text node");
		    l.append_child(&mut f, m).expect("unable to append node");
		});
	    });
	})
    }
}
