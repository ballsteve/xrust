use crate::item::NodeType;
use crate::qname::QualifiedName;
use crate::trees::smite::Node as SmiteNode;
use crate::trees::smite::RNode;
use crate::validators::relaxng::pattern::Param;
use crate::{Node, Value};
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) fn derive(doc: &RNode, pat: RNode, refs: &HashMap<String, RNode>) -> RNode {
    //println!("deriv-{:?}", doc.clone().child_iter().next().unwrap());
    child_deriv(pat, doc.child_iter().next().unwrap(), refs)
}

pub(crate) fn is_nullable(pat: RNode) -> bool {
    match pat.name().localname_to_string().as_str() {
        "empty" => true,
        "text" => true,
        "group" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            is_nullable(p1) && is_nullable(p2)
        }
        "interleave" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            is_nullable(p1) && is_nullable(p2)
        }
        "choice" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            is_nullable(p1) || is_nullable(p2)
        }
        "oneOrMore" => is_nullable(pat.first_child().unwrap()),
        //"element"
        //"attribute"
        //"list"
        //"value"
        //"data"
        //"dataExcept"
        //"notAllowed"
        //"after"
        _ => false,
    }
}

fn contains(nc: RNode, qn: Rc<QualifiedName>) -> bool {
    //println!("containsnc-{:?}", nc.clone());
    match nc.name().localname_to_string().as_str() {
        "anyName" => true,
        "anyNameExcept" => {
            let name = nc.first_child().unwrap();
            !contains(name, qn)
        }
        "NSName" => {
            let nsuri = nc.first_child().unwrap();
            Some(nsuri.to_string()) == qn.namespace_uri_to_string()
        }
        "NSNameExcept" => {
            let mut c = nc.child_iter();
            let ns1 = c.next().unwrap();
            let n = c.next().unwrap();
            (Some(ns1.to_string()) == qn.namespace_uri_to_string()) && !contains(n, qn)
        }
        "name" => {
            let ln1 = nc.first_child().unwrap();
            let ns1 = nc.get_attribute(&QualifiedName::new(None, None, "ns"));
            if ns1.to_string().is_empty() {
                qn.namespace_uri().is_none() && (ln1.to_string() == qn.localname_to_string())
            } else {
                (Some(ns1.to_string()) == qn.namespace_uri_to_string()) && (ln1.to_string() == qn.localname_to_string())
            }
        }
        "NameClassChoice" => {
            let mut c = nc.child_iter();
            let nc1 = c.next().unwrap();
            let nc2 = c.next().unwrap();
            contains(nc1, qn.clone()) || contains(nc2, qn)
        }
        _ => false,
    }
}

fn child_deriv(pat: RNode, cn: RNode, refs: &HashMap<String, RNode>) -> RNode {
    match cn.node_type() {
        NodeType::Document
        | NodeType::Attribute
        | NodeType::Comment
        | NodeType::ProcessingInstruction
        | NodeType::Reference
        | NodeType::Unknown
        | NodeType::Namespace => pat
            .owner_document()
            .new_element(Rc::new(QualifiedName::new(None, None, "notAllowed")))
            .unwrap(),
        NodeType::Text => text_deriv(pat, cn.value().to_string()),
        NodeType::Element => {
            //opening deriv
            //println!("start_tag_open_deriv");
            let mut pat1 = start_tag_open_deriv(pat, cn.name(), refs);
            //println!("pat1-{:?}", pat1.clone());

            //println!("att_deriv");
            //attsDeriv
            for attribute in cn.attribute_iter() {
                pat1 = att_deriv(attribute, pat1);
            }
            //println!("pat1-{:?}", pat1.clone());
            //println!("start_tag_close_deriv");
            //CloseTag
            pat1 = start_tag_close_deriv(pat1);

            //println!("pat1-{:?}", pat1.clone());
            //println!("children_deriv");
            //Children
            pat1 = children_deriv(pat1, cn.clone(), refs);

            //println!("pat1-{:?}", pat1.clone());

            //println!("end_tag_deriv");
            //end_tag_deriv
            pat1 = end_tag_deriv(pat1);

            //println!("pat1-{:?}", pat1.clone());
            pat1
        }
    }
}

fn start_tag_open_deriv(pat: RNode, q: Rc<QualifiedName>, refs: &HashMap<String, RNode>) -> RNode {
    //println!("stod-{:?}",pat.name().get_localname().as_str());
    match pat.name().localname_to_string().as_str() {
        "ref" => {
            //We lookup the reference, and use that for the pattern going forward
            let patname = pat.get_attribute(&QualifiedName::new(None, None, "name"));
            let newpat = refs.get(patname.to_string().as_str());
            match newpat {
                //TODO proper error checking
                None => pat
                    .owner_document()
                    .new_element(Rc::new(QualifiedName::new(None, None, "notAllowed")))
                    .unwrap(),
                Some(rn) => start_tag_open_deriv(rn.clone(), q, refs),
            }
        }
        "element" => {
            let mut pc = pat.child_iter();
            let nc = pc.next().unwrap();
            let p = pc.next().unwrap();
            if contains(nc, q) {
                after(
                    p,
                    pat.owner_document()
                        .new_element(Rc::new(QualifiedName::new(None, None, "empty")))
                        .unwrap(),
                )
            } else {
                pat.owner_document()
                    .new_element(Rc::new(QualifiedName::new(None, None, "notAllowed".to_string())))
                    .unwrap()
            }
        }
        "choice" => {
            let mut pc = pat.child_iter();
            choice(
                start_tag_open_deriv(pc.next().unwrap(), q.clone(), refs),
                start_tag_open_deriv(pc.next().unwrap(), q.clone(), refs),
            )
        }
        "interleave" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(
                apply_after(
                    |pat: RNode| {
                        let mut i = pat
                            .owner_document()
                            .new_element(Rc::new(QualifiedName::new(None, None, "interleave")))
                            .unwrap();
                        let _ = i.push(pat);
                        let _ = i.push(p2.clone());
                        i
                    },
                    start_tag_open_deriv(p1.clone(), q.clone(), refs),
                ),
                apply_after(
                    |pat: RNode| {
                        let mut i = pat
                            .owner_document()
                            .new_element(Rc::new(QualifiedName::new(None, None, "interleave")))
                            .unwrap();
                        let _ = i.push(pat);
                        let _ = i.push(p1.clone());
                        i
                    },
                    start_tag_open_deriv(p2.clone(), q.clone(), refs),
                ),
            )
        }
        "oneOrMore" => {
            let p1 = pat.first_child().unwrap();
            apply_after(
                |pat: RNode| {
                    group(
                        pat.clone(),
                        choice(
                            pat.clone(),
                            pat.owner_document()
                                .new_element(Rc::new(QualifiedName::new(None, None, "empty")))
                                .unwrap(),
                        ),
                    )
                },
                start_tag_open_deriv(p1, q, refs),
            )
        }
        "group" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            let x = apply_after(
                |pat: RNode| group(pat, p2.clone()),
                start_tag_open_deriv(p1.clone(), q.clone(), refs),
            );
            if is_nullable(p1) {
                choice(x, start_tag_open_deriv(p2, q, refs))
            } else {
                x
            }
        }
        "after" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            apply_after(
                |pat: RNode| after(pat, p2.clone()),
                start_tag_open_deriv(p1, q, refs),
            )
        }
        _ => pat
            .owner_document()
            .new_element(Rc::new(QualifiedName::new(None, None, "notAllowed")))
            .unwrap(),
    }
}

fn att_deriv(pat: RNode, att: RNode) -> RNode {
    match pat.name().localname_to_string().as_str() {
        "after" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            after(att_deriv(p1, att), p2)
        }
        "choice" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(att_deriv(p1, att.clone()), att_deriv(p2, att))
        }
        "group" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(
                group(att_deriv(p1.clone(), att.clone()), p2.clone()),
                group(att_deriv(p2, att), p1),
            )
        }
        "interleave" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(
                interleave(att_deriv(p1.clone(), att.clone()), p2.clone()),
                interleave(att_deriv(p2, att), p1),
            )
        }
        "oneOrMore" => {
            let p1 = pat.first_child().unwrap();
            group(
                att_deriv(p1, att),
                choice(
                    pat.clone(),
                    pat.owner_document()
                        .new_element(Rc::new(QualifiedName::new(None, None, "empty")))
                        .unwrap(),
                ),
            )
        }
        "attribute" => {
            let (qn, av) = (att.name(), att.value());
            let mut i = pat.child_iter();
            let nc = i.next().unwrap();
            let p1 = i.next().unwrap();
            if contains(nc, qn) && value_match(p1, av.to_string()) {
                pat.owner_document()
                    .new_element(Rc::new(QualifiedName::new(None, None, "empty")))
                    .unwrap()
            } else {
                pat.owner_document()
                    .new_element(Rc::new(QualifiedName::new(None, None, "notAllowed")))
                    .unwrap()
            }
        }
        _ => pat
            .owner_document()
            .new_element(Rc::new(QualifiedName::new(None, None, "notAllowed")))
            .unwrap(),
    }
}

fn start_tag_close_deriv(pat: RNode) -> RNode {
    match pat.name().localname_to_string().as_str() {
        "after" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            after(start_tag_close_deriv(p1), p2)
        }
        "choice" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(start_tag_close_deriv(p1), start_tag_close_deriv(p2))
        }
        "group" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            group(start_tag_close_deriv(p1), start_tag_close_deriv(p2))
        }
        "interleave" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            interleave(start_tag_close_deriv(p1), start_tag_close_deriv(p2))
        }
        "oneOrMore" => {
            let p = pat.first_child().unwrap();
            one_or_more(start_tag_close_deriv(p))
        }
        "attribute" => pat
            .owner_document()
            .new_element(Rc::new(QualifiedName::new(None, None, "notAllowed")))
            .unwrap(),
        _ => pat,
    }
}

fn children_deriv(pat: RNode, cn: RNode, refs: &HashMap<String, RNode>) -> RNode {
    match cn.clone().child_iter().count() {
        //match cn.len() {
        0 => {
            //We treat self closed elements as <e></e>
            choice(pat.clone(), text_deriv(pat, "".to_string()))
        }
        1 => {
            let n = cn.first_child().unwrap();
            match n.node_type() {
                NodeType::Text => {
                    let p1 = child_deriv(n.clone(), pat.clone(), refs);
                    if whitespace(n.value().to_string()) {
                        choice(pat, p1)
                    } else {
                        p1
                    }
                }
                _ => strip_children_deriv(pat, cn.child_iter(), refs),
            }
        }
        _ => strip_children_deriv(pat, cn.child_iter(), refs),
    }
}

fn end_tag_deriv(pat: RNode) -> RNode {
    match pat.name().localname_to_string().as_str() {
        "choice" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(end_tag_deriv(p1), end_tag_deriv(p2))
        }
        "after" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            if is_nullable(p1) {
                p2
            } else {
                pat.owner_document()
                    .new_element(Rc::new(QualifiedName::new(None, None, "notAllowed")))
                    .unwrap()
            }
        }
        _ => pat
            .owner_document()
            .new_element(Rc::new(QualifiedName::new(None, None, "notAllowed")))
            .unwrap(),
    }
}

fn text_deriv(pat: RNode, s: String) -> RNode {
    match pat.name().localname_to_string().as_str() {
        "choice" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(text_deriv(p1, s.clone()), text_deriv(p2, s.clone()))
        }
        "interleave" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(
                interleave(text_deriv(p1.clone(), s.clone()), p2.clone()),
                interleave(p1, text_deriv(p2, s.clone())),
            )
        }
        "group" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            let p = group(text_deriv(p1, s.clone()), p2.clone());
            if is_nullable(p.clone()) {
                choice(p.clone(), text_deriv(p2, s))
            } else {
                p
            }
        }
        "after" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            after(text_deriv(p1, s), p2)
        }
        "oneOrMore" => {
            let p = pat.first_child().unwrap();
            group(
                text_deriv(p.clone(), s.clone()),
                choice(
                    pat.clone(),
                    pat.owner_document()
                        .new_element(Rc::new(QualifiedName::new(None, None, "empty")))
                        .unwrap(),
                ),
            )
        }
        "Text" => pat,
        "Value" => {
            let dtlib = pat.get_attribute(&QualifiedName::new(
                None,
                None,
                "datatypeLibrary",
            ));
            let dtname = pat.get_attribute(&QualifiedName::new(None, None, "type"));
            let v = pat.value().to_string();
            if datatype_equal((dtlib, dtname), v, s) {
                pat.owner_document()
                    .new_element(Rc::new(QualifiedName::new(None, None, "empty")))
                    .unwrap()
            } else {
                pat.owner_document()
                    .new_element(Rc::new(QualifiedName::new(None, None, "notAllowed")))
                    .unwrap()
            }
        }
        "Data" => {
            let mut c = pat.child_iter();
            let dt = c.next().unwrap();
            //let params = c.collect();
            let params = vec![];
            if data_type_allows(dt, params, s) {
                pat.owner_document()
                    .new_element(Rc::new(QualifiedName::new(None, None, "empty")))
                    .unwrap()
            } else {
                pat.owner_document()
                    .new_element(Rc::new(QualifiedName::new(None, None, "notAllowed")))
                    .unwrap()
            }
        }
        "DataExcept" => {
            let mut c = pat.clone().child_iter();
            let dt = c.next().unwrap();
            //let params = c.collect();
            let params = vec![];
            if data_type_allows(dt, params, s.clone()) && !is_nullable(text_deriv(pat.clone(), s)) {
                pat.owner_document()
                    .new_element(Rc::new(QualifiedName::new(None, None, "empty")))
                    .unwrap()
            } else {
                pat.owner_document()
                    .new_element(Rc::new(QualifiedName::new(None, None, "notAllowed")))
                    .unwrap()
            }
        }
        "List" => {
            let p = pat.first_child().unwrap();
            if is_nullable(list_deriv(p, stringsplit(s))) {
                pat.owner_document()
                    .new_element(Rc::new(QualifiedName::new(None, None, "empty")))
                    .unwrap()
            } else {
                pat.owner_document()
                    .new_element(Rc::new(QualifiedName::new(None, None, "notAllowed")))
                    .unwrap()
            }
        }
        _ => pat
            .owner_document()
            .new_element(Rc::new(QualifiedName::new(None, None, "notAllowed")))
            .unwrap(),
    }
}

fn list_deriv(p: RNode, vs: Vec<String>) -> RNode {
    let mut vsi = vs.into_iter();
    match vsi.next() {
        None => p,
        Some(p1) => list_deriv(text_deriv(p, p1), vsi.collect()),
    }
}

fn strip_children_deriv(
    pat: RNode,
    mut cn: Box<dyn Iterator<Item = RNode>>,
    refs: &HashMap<String, RNode>,
) -> RNode {
    match cn.next() {
        None => pat,
        Some(h) => strip_children_deriv(
            if strip(h.clone()) {
                pat
            } else {
                child_deriv(pat, h, refs)
            },
            cn,
            refs,
        ),
    }
}

pub fn apply_after<F1>(f: F1, pat: RNode) -> RNode
where
    F1: Fn(RNode) -> RNode + Clone,
{
    match pat.name().localname_to_string().as_str() {
        "after" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            after(p1, f(p2))
        }
        "choice" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(apply_after(f.clone(), p1), apply_after(f, p2))
        }
        "notAllowed" => pat,
        _ => pat
            .owner_document()
            .new_element(Rc::new(QualifiedName::new(None, None, "notAllowed")))
            .unwrap(),
    }
}

fn choice(pat1: RNode, pat2: RNode) -> RNode {
    /*
        choice :: Pattern -> Pattern -> Pattern
        choice p NotAllowed = p
        choice NotAllowed p = p
        choice p1 p2 = Choice p1 p2
    */
    match (
        pat1.name().localname_to_string().as_str(),
        pat2.name().localname_to_string().as_str(),
    ) {
        ("notAllowed", _) => pat2,
        (_, "notAllowed") => pat1,
        (_, _) => {
            let mut c = pat1
                .owner_document()
                .new_element(Rc::new(QualifiedName::new(None, None, "choice")))
                .unwrap();
            let _ = c.push(pat1);
            let _ = c.push(pat2);
            c
        }
    }
}
fn group(pat1: RNode, pat2: RNode) -> RNode {
    match (
        pat1.name().localname_to_string().as_str(),
        pat2.name().localname_to_string().as_str(),
    ) {
        ("notAllowed", _) => pat1,
        (_, "notAllowed") => pat2,
        ("empty", _) => pat2,
        (_, "empty") => pat1,
        (_, _) => {
            let mut g = pat1
                .owner_document()
                .new_element(Rc::new(QualifiedName::new(None, None, "group")))
                .unwrap();
            let _ = g.push(pat1);
            let _ = g.push(pat2);
            g
        }
    }
}
fn after(pat1: RNode, pat2: RNode) -> RNode {
    //println!("afterpat1-{:?}",pat1.name().get_localname().as_str());
    //println!("afterpat2-{:?}",pat2.name().get_localname().as_str());
    match (
        pat1.name().localname_to_string().as_str(),
        pat2.name().localname_to_string().as_str(),
    ) {
        (_, "notAllowed") => pat2,
        ("notAllowed", _) => pat1,
        (_, _) => {
            let mut a = pat1
                .owner_document()
                .new_element(Rc::new(QualifiedName::new(None, None, "after")))
                .unwrap();
            let _ = a.push(pat1);
            let _ = a.push(pat2);
            a
        }
    }
}
fn interleave(pat1: RNode, pat2: RNode) -> RNode {
    match (
        pat1.name().localname_to_string().as_str(),
        pat2.name().localname_to_string().as_str(),
    ) {
        ("notAllowed", _) => pat1,
        (_, "notAllowed") => pat2,
        ("empty", _) => pat2,
        (_, "empty") => pat1,
        (_, _) => {
            let mut i = pat1
                .owner_document()
                .new_element(Rc::new(QualifiedName::new(None, None, "interleave")))
                .unwrap();
            let _ = i.push(pat1);
            let _ = i.push(pat2);
            i
        }
    }
}
fn value_match(pat: RNode, s: String) -> bool {
    (is_nullable(pat.clone()) && whitespace(s.clone())) || is_nullable(text_deriv(pat, s))
}
fn whitespace(s: String) -> bool {
    //tests whether a string is contains only whitespace.
    !s.contains(|c| !char::is_whitespace(c))
}
fn strip(c: RNode) -> bool {
    match c.node_type() {
        NodeType::Text => whitespace(c.value().to_string()),
        _ => false,
    }
}
fn one_or_more(pat: RNode) -> RNode {
    match pat.name().localname_to_string().as_str() {
        "notAllowed" => pat,
        _ => {
            let mut o = Rc::new(SmiteNode::new())
                .new_element(Rc::new(QualifiedName::new(None, None, "oneOrMore")))
                .unwrap();
            let _ = o.push(pat);
            o
        }
    }
}
fn data_type_allows(dt: RNode, _params: Vec<Param>, _s: String) -> bool {
    let _datatypens = dt.name().namespace_uri();
    let datatype = dt.name().localname_to_string();
    match datatype.as_str() {
        "string" => true,
        "token" => true,
        _ => false,
    }
}
fn datatype_equal((_d, s): (Rc<Value>, Rc<Value>), s1: String, s2: String) -> bool {
    match s.as_ref() {
        Value::String(_) => s1 == s2,
        Value::Token => normalize_whitespace(s1) == normalize_whitespace(s2),
        _ => false,
    }
    /*
    match s.as_str() {
        "string" => {s1 == s2},
        "token" => {
            normalize_whitespace(s1) == normalize_whitespace(s2)
        }
        _ => false
    }

     */
}
fn normalize_whitespace(s: String) -> String {
    s.trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}
fn stringsplit(s: String) -> Vec<String> {
    let t = s.split(' ').map(|u| u.to_string()).collect();
    t
}
