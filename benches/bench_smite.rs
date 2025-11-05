use std::collections::HashMap;
use std::rc::Rc;
use std::sync::LazyLock;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use qualname::{NamespaceUri, NcName, QName};
use xrust::item::Node;
use xrust::parser::ParseError;
use xrust::parser::xml::parse;
use xrust::trees::smite::RNode;
use xrust::value::Value;

static NSURI: LazyLock<Option<NamespaceUri>> =
    LazyLock::new(|| Some(NamespaceUri::try_from("http://www.example.com/namespace").unwrap()));

fn make_rnode(n: u64) -> RNode {
    let mut a = RNode::new_document();
    let mut b = a
        .new_element(QName::from_local_name(NcName::try_from("Test").unwrap()))
        .expect("unable to create element");
    a.push(b.clone()).expect("unable to add node");
    let l1name = QName::from_local_name(NcName::try_from("Level-1").unwrap());
    let l2name = QName::from_local_name(NcName::try_from("Level-2").unwrap());
    (1..n).for_each(|i| {
        let mut l1 = a
            .new_element(l1name.clone())
            .expect("unable to create element");
        b.push(l1.clone()).expect("unable to add node");
        (1..n).for_each(|k| {
            let mut l2 = a
                .new_element(l2name.clone())
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
    let doc = RNode::new_document();
    parse(
        doc.clone(),
        a.as_str(),
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    )
    .expect("failed to parse XML");
    doc
}

/// Create N nodes with M distinct names repeated R times (N = M*R) and each name at least L characters in length
fn make_m_r_b(s: (usize, usize, &str)) -> RNode {
    let (m, r, base) = s;
    // base is the prefix to use when creating a name. An integer will be appended to make a name at least L characters in length.
    let mut names = HashMap::new();
    let mut doc = RNode::new_document();
    let mut top = doc
        .new_element(QName::new_from_parts(
            NcName::try_from("top").unwrap(),
            NSURI.clone(),
        ))
        .unwrap();
    doc.push(top.clone()).unwrap();
    for i in 0..m {
        names.insert(
            format!("{}{}", base, i),
            QName::new_from_parts(
                NcName::try_from(format!("{}{}", base, i).as_str()).unwrap(),
                NSURI.clone(),
            ),
        );
    }
    for _j in 0..r {
        for i in 0..m {
            let nd = doc
                .new_element(names.get(&format!("{}{}", base, i)).unwrap().clone())
                .unwrap();
            top.push(nd).unwrap();
        }
    }
    doc
}

/// Search through a list of nodes looking for a particular named node
fn search_nodes(s: (RNode, QName)) -> usize {
    let (parent, name) = s;
    parent
        .child_iter()
        .filter(|c| c.name().is_some_and(|nm| nm == name))
        .collect::<Vec<RNode>>()
        .len()
}

fn rnode(c: &mut Criterion) {
    c.bench_function("rnode 1000", |b| b.iter(|| make_rnode(black_box(1000))));
    c.bench_function("parse 100", |b| b.iter(|| parse_doc(black_box(100))));
    c.bench_function("parse 1000", |b| b.iter(|| parse_doc(black_box(1000))));
}

fn qname(c: &mut Criterion) {
    c.bench_function("make_m_r_b", |b| {
        b.iter(|| make_m_r_b(black_box((100, 100, "basebasebasebase"))))
    });
    let doc100 = make_m_r_b((100, 100, "basebasebasebase"));
    let doc1000 = make_m_r_b((1000, 100, "basebasebasebase"));
    let mut group = c.benchmark_group("search");
    group.bench_with_input(BenchmarkId::from_parameter(100), &100, |b, _| {
        b.iter(|| {
            search_nodes(black_box((
                doc100.clone(),
                QName::new_from_parts(
                    NcName::try_from("basebasebasebase1").unwrap(),
                    NSURI.clone(),
                ),
            )))
        })
    });
    group.bench_with_input(BenchmarkId::from_parameter(1000), &1000, |b, _| {
        b.iter(|| {
            search_nodes(black_box((
                doc1000.clone(),
                QName::new_from_parts(
                    NcName::try_from("basebasebasebase1").unwrap(),
                    NSURI.clone(),
                ),
            )))
        })
    });
}

//criterion_group!(benches, rnode);
criterion_group!(benches, qname);
criterion_main!(benches);
