use criterion::{black_box, criterion_group, criterion_main, Criterion};

use xrust::xdmerror::*;
use xrust::qname::*;
use xrust::value::*;
use xrust::forest::*;

fn make_tree(n: u64) -> TreeIndex {
    let mut f = Forest::new();
    let ti = f.plant_tree();
    let r = f.get_ref_mut(ti).unwrap()
	.new_element(QualifiedName::new(None, None, String::from("Test")))
	.expect("unable to create element node");
    f.get_ref_mut(ti).unwrap()
	.push_doc_node(r)
	.expect("unable to add doc node");
    (1..n).for_each(|i| {
	let j = f.get_ref_mut(ti).unwrap()
	    .new_element(QualifiedName::new(None, None, String::from("Level-1")))
	    .expect("unable to create element node");
	r.append_child(&mut f, j).expect("unable to append node");
	(1..n).for_each(|k| {
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
    ti
}

fn forest(c: &mut Criterion) {
    c.bench_function(
	"forest 100",
	|b| b.iter(|| make_tree(black_box(100)))
    );
}

criterion_group!(benches, forest);
criterion_main!(benches);



