use crate::Node;
use crate::item::NodeType;
use qualname::QName;

use crate::xmldecl::{DTD, DTDPattern};

pub(crate) fn is_nullable(pat: DTDPattern) -> bool {
    match pat {
        DTDPattern::Empty => true,
        DTDPattern::Text => true,
        DTDPattern::Any => true, //TODO Check
        DTDPattern::Group(pat1, pat2) => is_nullable(*pat1) && is_nullable(*pat2),
        DTDPattern::Interleave(pat1, pat2) => is_nullable(*pat1) && is_nullable(*pat2),
        DTDPattern::Choice(pat1, pat2) => is_nullable(*pat1) || is_nullable(*pat2),
        DTDPattern::OneOrMore(pat1) => is_nullable(*pat1),
        _ => false,
    }
}

/*fn contains(nc: QName, qn: QName) -> bool {
    nc == qn
    //nc.prefix() == qn.prefix() && nc.localname() == qn.localname()
}*/

fn after(pat1: DTDPattern, pat2: DTDPattern) -> DTDPattern {
    if pat1 == DTDPattern::NotAllowed || pat2 == DTDPattern::NotAllowed {
        DTDPattern::NotAllowed
    } else {
        DTDPattern::After(Box::new(pat1), Box::new(pat2))
    }
}
fn choice(pat1: DTDPattern, pat2: DTDPattern) -> DTDPattern {
    match (pat1, pat2) {
        (p, DTDPattern::NotAllowed) => p,
        (DTDPattern::NotAllowed, p) => p,
        (p1, p2) => DTDPattern::Choice(Box::new(p1), Box::new(p2)),
    }
}
fn interleave(pat1: DTDPattern, pat2: DTDPattern) -> DTDPattern {
    match (pat1, pat2) {
        (DTDPattern::NotAllowed, _) => DTDPattern::NotAllowed,
        (_, DTDPattern::NotAllowed) => DTDPattern::NotAllowed,
        (DTDPattern::Empty, p2) => p2,
        (p1, DTDPattern::Empty) => p1,
        (DTDPattern::Any, p2) => p2, //TODO CHECK
        (p1, DTDPattern::Any) => p1, //TODO CHECK
        (p1, p2) => DTDPattern::Interleave(Box::new(p1), Box::new(p2)),
    }
}
fn group(pat1: DTDPattern, pat2: DTDPattern) -> DTDPattern {
    match (pat1, pat2) {
        (DTDPattern::NotAllowed, _) => DTDPattern::NotAllowed,
        (_, DTDPattern::NotAllowed) => DTDPattern::NotAllowed,
        (DTDPattern::Empty, p2) => p2,
        (p1, DTDPattern::Empty) => p1,
        (p1, p2) => DTDPattern::Group(Box::new(p1), Box::new(p2)),
    }
}

pub fn apply_after<F1>(pat: DTDPattern, f: F1) -> DTDPattern
where
    F1: Fn(DTDPattern) -> DTDPattern + Clone,
{
    match pat {
        DTDPattern::After(pat1, pat2) => after(*pat1, f(*pat2)),
        DTDPattern::Choice(pat1, pat2) => {
            choice(apply_after(*pat1, f.clone()), apply_after(*pat2, f))
        }
        _ => DTDPattern::NotAllowed,
    }
}

fn value_match(pat: DTDPattern, s: String) -> bool {
    (is_nullable(pat.clone()) && whitespace(s.clone())) || is_nullable(text_deriv(pat, s))
}

fn text_deriv(pat: DTDPattern, s: String) -> DTDPattern {
    match pat {
        DTDPattern::Choice(pat1, pat2) => {
            choice(text_deriv(*pat1, s.clone()), text_deriv(*pat2, s))
        }
        DTDPattern::Interleave(pat1, pat2) => choice(
            interleave(text_deriv(*pat1.clone(), s.clone()), *pat2.clone()),
            interleave(*pat1, text_deriv(*pat2, s.clone())),
        ),
        DTDPattern::Group(pat1, pat2) => {
            let p = group(text_deriv(*pat1, s.clone()), *pat2.clone());
            if is_nullable(p.clone()) {
                choice(p, text_deriv(*pat2, s))
            } else {
                p
            }
        }
        DTDPattern::After(pat1, pat2) => after(text_deriv(*pat1, s), *pat2),
        DTDPattern::OneOrMore(pat1) => group(
            text_deriv(*pat1.clone(), s),
            choice(*pat1, DTDPattern::Empty),
        ),
        DTDPattern::List(pat1) => {
            if is_nullable(list_deriv(
                *pat1,
                s.split(' ').map(|st| st.to_string()).collect(),
            )) {
                DTDPattern::Empty
            } else {
                DTDPattern::NotAllowed
            }
        }
        DTDPattern::Value(val) => {
            if s == val {
                DTDPattern::Empty
            } else {
                DTDPattern::NotAllowed
            }
        }
        //textDeriv cx1 (Value dt value cx2) s = if datatypeEqual dt value cx2 s cx1 then Empty else NotAllowed
        DTDPattern::Text => pat,
        DTDPattern::Any => pat,
        DTDPattern::Empty => DTDPattern::NotAllowed,
        DTDPattern::NotAllowed => DTDPattern::NotAllowed,
        DTDPattern::Attribute(_, _) => DTDPattern::NotAllowed,
        DTDPattern::Element(_, _) => DTDPattern::NotAllowed,
        DTDPattern::Ref(_) => DTDPattern::NotAllowed,
    }
}

fn list_deriv(p: DTDPattern, vs: Vec<String>) -> DTDPattern {
    let mut vsi = vs.into_iter();
    match vsi.next() {
        None => p,
        Some(p1) => list_deriv(text_deriv(p, p1), vsi.collect()),
    }
}

pub(crate) fn child_deriv(pat: DTDPattern, n: impl Node, dtd: DTD) -> DTDPattern {
    //println!("child_deriv");
    //println!("    {:?}", &pat);
    //println!("    {:?}", &n);
    match n.node_type() {
        NodeType::Document => DTDPattern::NotAllowed,
        NodeType::Attribute => DTDPattern::NotAllowed,
        NodeType::Comment => DTDPattern::NotAllowed,
        NodeType::ProcessingInstruction => DTDPattern::NotAllowed,
        NodeType::Reference => DTDPattern::NotAllowed,
        NodeType::Namespace => DTDPattern::NotAllowed,
        NodeType::Unknown => DTDPattern::NotAllowed,
        NodeType::Text => text_deriv(pat, n.to_string()),
        NodeType::Element => {
            let mut pat1 = start_tag_open_deriv(pat, n.name().unwrap(), dtd.clone());
            //at this stage, we check if the DTD is for DTDPattern::Any. If it is present, we build a pattern
            //based on the child nodes, so that they are all validated individually.
            match pat1.clone() {
                DTDPattern::After(a, p) => {
                    match *a {
                        DTDPattern::Any => {
                            let mut newpat = DTDPattern::Empty;
                            let mut children = n
                                .child_iter()
                                .filter(|node| {
                                    node.node_type() != NodeType::ProcessingInstruction
                                        && node.node_type() != NodeType::Comment
                                        && !(node.node_type() == NodeType::Text
                                            && node.value().to_string() == *"")
                                })
                                .collect::<Vec<_>>();
                            //todo POP VECTOR UNTIL EMPTY
                            children.reverse();
                            for c in children {
                                match c.node_type() {
                                    NodeType::Element => {
                                        newpat = DTDPattern::Group(
                                            Box::new(DTDPattern::Ref((
                                                None,
                                                c.name().unwrap().to_string(),
                                            ))),
                                            Box::new(newpat),
                                        )
                                    }
                                    NodeType::Text => {
                                        newpat = DTDPattern::Group(
                                            Box::new(DTDPattern::Text),
                                            Box::new(newpat),
                                        )
                                    }
                                    _ => {}
                                }
                            }

                            pat1 = DTDPattern::After(Box::new(newpat), p)
                        }
                        DTDPattern::Group(an, p1) => {
                            if *an == DTDPattern::Any {
                                let mut newpat = DTDPattern::Empty;
                                let mut children = n
                                    .child_iter()
                                    .filter(|node| {
                                        node.node_type() != NodeType::ProcessingInstruction
                                            && node.node_type() != NodeType::Comment
                                            && !(node.node_type() == NodeType::Text
                                                && node.value().to_string() == *"")
                                    })
                                    .collect::<Vec<_>>();
                                //todo POP VECTOR UNTIL EMPTY
                                children.reverse();
                                for c in children {
                                    match c.node_type() {
                                        NodeType::Element => {
                                            newpat = DTDPattern::Group(
                                                Box::new(DTDPattern::Ref((
                                                    None,
                                                    c.name().unwrap().to_string(),
                                                ))),
                                                Box::new(newpat),
                                            )
                                        }
                                        NodeType::Text => {
                                            newpat = DTDPattern::Group(
                                                Box::new(DTDPattern::Text),
                                                Box::new(newpat),
                                            )
                                        }
                                        _ => {}
                                    }
                                }

                                pat1 = DTDPattern::After(
                                    Box::from(DTDPattern::Group(Box::new(newpat), p1)),
                                    p,
                                )
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }

            for attribute in n.attribute_iter() {
                pat1 = att_deriv(pat1, attribute)
            }
            pat1 = start_tag_close_deriv(pat1);
            pat1 = children_deriv(pat1, n, dtd);
            pat1 = end_tag_deriv(pat1);
            pat1
        }
    }
}

fn start_tag_open_deriv(pat: DTDPattern, qn: QName, dtd: DTD) -> DTDPattern {
    //println!("start_tag_open_deriv");
    //println!("    {:?}", &pat);
    //println!("    {:?}", &qn);
    match pat {
        DTDPattern::Ref(q) => match dtd.patterns.get(&q) {
            None => DTDPattern::NotAllowed,
            Some(p1) => start_tag_open_deriv(p1.clone(), qn, dtd),
        },
        DTDPattern::Any => after(pat, DTDPattern::Empty),
        DTDPattern::Element(nc, pat1) => {
            if nc.1 == qn.local_name() {
                //TODO THE ERROR MIGHT BE HERE????

                after(*pat1, DTDPattern::Empty)
            } else {
                DTDPattern::NotAllowed
            }
        }
        DTDPattern::Choice(pat1, pat2) => choice(
            start_tag_open_deriv(*pat1, qn.clone(), dtd.clone()),
            start_tag_open_deriv(*pat2, qn, dtd),
        ),
        DTDPattern::Interleave(pat1, pat2) => choice(
            apply_after(
                start_tag_open_deriv(*pat1.clone(), qn.clone(), dtd.clone()),
                |p: DTDPattern| DTDPattern::Interleave(Box::new(p), pat2.clone()),
            ),
            apply_after(
                start_tag_open_deriv(*pat2.clone(), qn.clone(), dtd),
                |p: DTDPattern| DTDPattern::Interleave(Box::new(p), pat1.clone()),
            ),
        ),
        DTDPattern::Group(pat1, pat2) => {
            let x = apply_after(
                start_tag_open_deriv(*pat1.clone(), qn.clone(), dtd.clone()),
                |pat| group(pat, *pat2.clone()),
            );
            if is_nullable(*pat1) {
                choice(x, start_tag_open_deriv(*pat2, qn, dtd))
            } else {
                x
            }
        }
        DTDPattern::OneOrMore(pat1) => {
            apply_after(start_tag_open_deriv(*pat1.clone(), qn, dtd), |pt| {
                group(
                    pt,
                    choice(DTDPattern::OneOrMore(pat1.clone()), DTDPattern::Empty),
                )
            })
        }
        DTDPattern::After(pat1, pat2) => apply_after(start_tag_open_deriv(*pat1, qn, dtd), |p| {
            after(p, *pat2.clone())
        }),
        DTDPattern::Value(_) => DTDPattern::NotAllowed,
        DTDPattern::Empty => DTDPattern::NotAllowed,
        DTDPattern::NotAllowed => DTDPattern::NotAllowed,
        DTDPattern::Text => DTDPattern::NotAllowed,
        DTDPattern::List(_) => DTDPattern::NotAllowed,
        DTDPattern::Attribute(_, _) => DTDPattern::NotAllowed,
    }
}

fn att_deriv(pat: DTDPattern, att: impl Node) -> DTDPattern {
    //println!("att_deriv");
    //println!("    {:?}", &pat);
    //println!("    {:?}", &att);
    match pat {
        DTDPattern::Choice(pat1, pat2) => {
            choice(att_deriv(*pat1, att.clone()), att_deriv(*pat2, att.clone()))
        }
        DTDPattern::Interleave(pat1, pat2) => choice(
            interleave(att_deriv(*pat1.clone(), att.clone()), *pat2.clone()),
            interleave(att_deriv(*pat2.clone(), att), *pat1),
        ),
        DTDPattern::Group(pat1, pat2) => choice(
            group(att_deriv(*pat1.clone(), att.clone()), *pat2.clone()),
            group(att_deriv(*pat2.clone(), att.clone()), *pat1),
        ),
        DTDPattern::OneOrMore(pat1) => group(
            att_deriv(*pat1.clone(), att),
            choice(*pat1, DTDPattern::Empty),
        ),
        DTDPattern::After(pat1, pat2) => after(att_deriv(*pat1, att), *pat2),
        DTDPattern::Attribute(nc, pat1) => {
            if nc.1 == att.name().unwrap().local_name()
                && value_match(*pat1, att.value().to_string())
            {
                DTDPattern::Empty
            } else {
                DTDPattern::NotAllowed
            }
        }
        DTDPattern::Any => DTDPattern::NotAllowed,
        DTDPattern::Value(_) => DTDPattern::NotAllowed,
        DTDPattern::Empty => DTDPattern::NotAllowed,
        DTDPattern::NotAllowed => DTDPattern::NotAllowed,
        DTDPattern::Text => DTDPattern::NotAllowed,
        DTDPattern::List(_) => DTDPattern::NotAllowed,
        DTDPattern::Element(_, _) => DTDPattern::NotAllowed,
        DTDPattern::Ref(_) => DTDPattern::NotAllowed,
    }
}

fn start_tag_close_deriv(pat: DTDPattern) -> DTDPattern {
    //println!("start_tag_close_deriv");
    //println!("    {:?}", &pat);
    match pat {
        DTDPattern::Choice(pat1, pat2) => {
            choice(start_tag_close_deriv(*pat1), start_tag_close_deriv(*pat2))
        }
        DTDPattern::Interleave(pat1, pat2) => {
            interleave(start_tag_close_deriv(*pat1), start_tag_close_deriv(*pat2))
        }
        DTDPattern::Group(pat1, pat2) => {
            group(start_tag_close_deriv(*pat1), start_tag_close_deriv(*pat2))
        }
        DTDPattern::OneOrMore(pat1) => match start_tag_close_deriv(*pat1.clone()) {
            DTDPattern::NotAllowed => DTDPattern::NotAllowed,
            _ => DTDPattern::OneOrMore(Box::new(start_tag_close_deriv(*pat1))),
        },
        DTDPattern::After(pat1, pat2) => after(start_tag_close_deriv(*pat1), *pat2),
        DTDPattern::Attribute(_, _) => DTDPattern::NotAllowed,
        DTDPattern::Any => pat,
        DTDPattern::Value(_) => pat,
        DTDPattern::Empty => pat,
        DTDPattern::NotAllowed => pat,
        DTDPattern::Text => pat,
        DTDPattern::List(_) => pat,
        DTDPattern::Element(_, _) => pat,
        DTDPattern::Ref(_) => pat,
    }
}

fn children_deriv(pat: DTDPattern, cn: impl Node, dtd: DTD) -> DTDPattern {
    //println!("children_deriv");
    //println!("    {:?}", &pat);
    //println!("    {:?}", cn.child_iter().collect::<Vec<_>>());
    //Filter out comments, processing instructions, empty text nodes
    let children: Vec<_> = cn
        .child_iter()
        .filter(|node| {
            node.node_type() != NodeType::ProcessingInstruction
                && node.node_type() != NodeType::Comment
                && !(node.node_type() == NodeType::Text && whitespace(node.value().to_string()))
        })
        .collect();
    //println!("children_deriv_children-{:?}", &children);
    let mut pat1 = pat;
    match children.len() {
        0 => {
            pat1 = choice(pat1.clone(), text_deriv(pat1, "".to_string()));
        }
        _ => {
            let mut c = children.into_iter().peekable();
            while let Some(n) = c.next() {
                if c.peek().is_none() {
                    match n.node_type() {
                        NodeType::Text => {
                            let p1 = child_deriv(pat1.clone(), n.clone(), dtd.clone());
                            pat1 = if whitespace(n.value().to_string()) {
                                choice(pat1.clone(), p1)
                            } else {
                                p1
                            }
                        }
                        _ => pat1 = strip_children_deriv(pat1.clone(), vec![n], dtd.clone()),
                    }
                } else if !strip(n.clone()) {
                    pat1 = child_deriv(pat1, n.clone(), dtd.clone())
                }
            }
        }
    }
    pat1
}

fn end_tag_deriv(pat: DTDPattern) -> DTDPattern {
    //println!("end_tag_deriv");
    //println!("    {:?}", &pat);
    match pat {
        DTDPattern::Choice(pat1, pat2) => choice(end_tag_deriv(*pat1), end_tag_deriv(*pat2)),
        DTDPattern::After(pat1, pat2) => {
            if is_nullable(*pat1) {
                *pat2
            } else {
                DTDPattern::NotAllowed
            }
        }
        DTDPattern::Any => pat,
        _ => DTDPattern::NotAllowed,
    }
}

fn strip_children_deriv<T>(pat: DTDPattern, cnodes: Vec<T>, dtd: DTD) -> DTDPattern
where
    T: Node,
{
    let mut ci = cnodes.iter();
    match ci.next() {
        None => pat,
        Some(h) => {
            strip_children_deriv(
                if strip(h.clone()) {
                    pat
                } else {
                    child_deriv(pat, h.clone(), dtd.clone())
                },
                ci.cloned().collect(),
                //ci.map(|c| c.clone()).collect(),
                dtd,
            )
        }
    }
}

fn whitespace(s: String) -> bool {
    s.chars().all(char::is_whitespace)
}
fn strip(c: impl Node) -> bool {
    match c.node_type() {
        NodeType::Text => whitespace(c.value().to_string()),
        _ => false,
    }
}
