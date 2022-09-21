use std::rc::Rc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use xrust::xdmerror::Error;
use xrust::qname::QualifiedName;
use xrust::value::Value;
use xrust::item::{NodeType, MNode};
use xrust::intmuttree::{NodeBuilder, RNode};

fn make_rnode(n: u64) -> RNode {
    let mut a = NodeBuilder::new(NodeType::Document)
	.build();
    let mut b = NodeBuilder::new(NodeType::Element)
	.name(QualifiedName::new(None, None, String::from("Test")))
	.build();
    a.push(b.clone());
    (1..n).for_each(|i| {
	let mut l1 = NodeBuilder::new(NodeType::Element)
	    .name(QualifiedName::new(None, None, String::from("Level-1")))
	    .build();
	b.push(l1.clone());
	(1..n).for_each(|k| {
	    let mut l2 = NodeBuilder::new(NodeType::Element)
		.name(QualifiedName::new(None, None, String::from("Level-2")))
		.build();
	    l1.push(l2.clone());
	    l2.push(NodeBuilder::new(NodeType::Text)
		    .value(Value::from(format!("node {}-{}", i, k)))
		    .build()
	    );
	});
    });
    a
}

fn rnode(c: &mut Criterion) {
    c.bench_function(
	"rnode 100",
	|b| b.iter(|| make_rnode(black_box(1000)))
    );
}

criterion_group!(benches, rnode);
criterion_main!(benches);

