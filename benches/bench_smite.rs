use std::rc::Rc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use xrust::parser::xml::parse;
use xrust::item::Node;
use xrust::qname::QualifiedName;
use xrust::trees::smite::{Node as SmiteNode, RNode};
use xrust::value::Value;

fn make_rnode(n: u64) -> RNode {
    let mut a = Rc::new(SmiteNode::new());
    let mut b = a
        .new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
        .expect("unable to create element");
    a.push(b.clone()).expect("unable to add node");
    (1..n).for_each(|i| {
        let l1name = Rc::new(Value::from("Level-1"));
        let mut l1 = a
            .new_element(Rc::new(QualifiedName::new_from_values(None, None, l1name.clone())))
            .expect("unable to create element");
        b.push(l1.clone()).expect("unable to add node");
        let l2name = Rc::new(Value::from("Level-2"));
        (1..n).for_each(|k| {
            let mut l2 = a
                .new_element(Rc::new(QualifiedName::new_from_values(None, None, l2name.clone())))
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

fn parse_doc(n: u64) -> RNode {
    let mut a = String::from("<pre:top_level xmlns:pre='urn:benchmark.org'>\n");
    (1..n).for_each(|i| {
        a.push_str("  <pre:child>");
        a.push_str(format!("{}", i).as_str());
        a.push_str("</pre:child>\n");
    });
    a.push_str("</pre:top_level>\n");
    let doc = Rc::new(SmiteNode::new());
    parse(doc.clone(), a.as_str(), None).expect("failed to parse XML");
    doc
}

fn rnode(c: &mut Criterion) {
    c.bench_function("rnode 1000", |b| b.iter(|| make_rnode(black_box(1000))));
    c.bench_function("parse 100", |b| b.iter(|| parse_doc(black_box(100))));
    c.bench_function("parse 1000", |b| b.iter(|| parse_doc(black_box(1000))));
}

criterion_group!(benches, rnode);
criterion_main!(benches);
