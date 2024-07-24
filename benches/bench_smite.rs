use std::rc::Rc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use xrust::item::Node;
use xrust::qname::QualifiedName;
use xrust::trees::smite::{Node as SmiteNode, RNode};
use xrust::value::Value;

fn make_rnode(n: u64) -> RNode {
    let mut a = Rc::new(SmiteNode::new());
    let mut b = a
        .new_element(QualifiedName::new(None, None, String::from("Test")))
        .expect("unable to create element");
    a.push(b.clone()).expect("unable to add node");
    (1..n).for_each(|i| {
        let mut l1 = a
            .new_element(QualifiedName::new(None, None, String::from("Level-1")))
            .expect("unable to create element");
        b.push(l1.clone()).expect("unable to add node");
        (1..n).for_each(|k| {
            let mut l2 = a
                .new_element(QualifiedName::new(None, None, String::from("Level-2")))
                .expect("unable to create element");
            l1.push(l2.clone()).expect("unable to add node");
            l2.push(
                a.new_text(Rc::new(Value::from(format!("node {}-{}", i, k))))
                    .expect("unable to create text node"),
            )
            .expect("unable to add node");
        });
    });
    a
}

fn rnode(c: &mut Criterion) {
    c.bench_function("rnode 100", |b| b.iter(|| make_rnode(black_box(1000))));
}

criterion_group!(benches, rnode);
criterion_main!(benches);
