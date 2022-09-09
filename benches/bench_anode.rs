use std::rc::Rc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use xrust::xdmerror::Error;
use xrust::qname::QualifiedName;
use xrust::value::Value;
use xrust::item::{NodeType, MNode};
use xrust::rctree::{ANodeBuilder, RANode};

fn make_anode(n: u64) -> RANode {
    let mut b = Rc::new(
	ANodeBuilder::new(NodeType::Element)
	    .name(QualifiedName::new(None, None, String::from("Test")))
	    .build()
    );
    (1..n).for_each(|i| {
	let mut l1 = Rc::new(
	    ANodeBuilder::new(NodeType::Element)
		.name(QualifiedName::new(None, None, String::from("Level-1")))
		.build()
	);
	(1..n).for_each(|k| {
	    let mut l2 = Rc::new(
		ANodeBuilder::new(NodeType::Element)
		    .name(QualifiedName::new(None, None, String::from("Level-2")))
		    .build()
	    );
	    l2.push(Rc::new(
		ANodeBuilder::new(NodeType::Text)
		    .value(Value::from(format!("node {}-{}", i, k)))
		    .build()
	    ))
		.expect("unable to create text node");
	    l1.push(l2)
		.expect("unable to append level-2 element");
	});
	b.push(l1)
	    .expect("unable to append level-1 element");
    });
    let mut a = Rc::new(
	ANodeBuilder::new(NodeType::Document)
	    .build()
    );
    a.push(b)
	.expect("unable to append toplevel element");
    a
}

fn anode(c: &mut Criterion) {
    c.bench_function(
	"anode 100",
	|b| b.iter(|| make_anode(black_box(100)))
    );
}

criterion_group!(benches, anode);
criterion_main!(benches);

