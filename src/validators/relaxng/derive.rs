use crate::item::NodeType;
use crate::Node;
use crate::qname::QualifiedName;
use crate::trees::smite::{RNode};
use crate::validators::relaxng::pattern::{DataType, NameClass, Param, Pattern};

pub(crate) fn derive(doc: &RNode, pat: Pattern) -> Pattern {
    childDeriv(pat, doc.child_iter().next().unwrap())
}

fn contains(nc: NameClass, qn: QualifiedName) -> bool {
    match (nc, qn) {
        (NameClass::AnyName, _) => true,
        (NameClass::AnyNameExcept(n), q) => !contains(*n, q),
        (NameClass::NSName(nsuri), q ) => Some(nsuri)==q.get_nsuri(),
        (NameClass::NSNameExcept(ns1, n), q) => (Some(ns1) ==q.get_nsuri()) && !contains(*n, q),
        (NameClass::Name(ns1, ln1), q) => (Some(ns1) == q.get_nsuri()) && (ln1 ==q.get_localname()),
        (NameClass::NameClassChoice(nc1, nc2), q) => contains(*nc1,q.clone()) || contains(*nc2, q)
    }
}



fn childDeriv(pat: Pattern, cn: RNode) -> Pattern {
    match cn.node_type(){
        NodeType::Document => {Pattern::NotAllowed}
        NodeType::Attribute => {Pattern::NotAllowed}
        NodeType::Comment => {Pattern::NotAllowed}
        NodeType::ProcessingInstruction => {Pattern::NotAllowed}
        NodeType::Reference => {Pattern::NotAllowed}
        NodeType::Unknown => {Pattern::NotAllowed}
        //NodeType::Text => {Pattern::NotAllowed}
        NodeType::Text => { textDeriv(pat, cn.value().to_string()) }
        //NodeType::Element => {Pattern::NotAllowed}

        NodeType::Element => {
            //opening deriv
            let mut pat1 = startTagOpenDeriv(pat, cn.name());
            println!("p1={:?}", &pat1);
            //attsDeriv
            for attribute in cn.attribute_iter() {
                pat1 = attDeriv(pat1, attribute)
            }
            //CloseTag
            pat1 = startTagCloseDeriv(pat1);
            //Children
            pat1 = childrenDeriv(pat1, cn.clone());
            //EndTagDeriv
            endTagDeriv(pat1)
        }

    }

    /*
    match (pat, cn) {
        (p, ChildNode::TextNode(s)) => textDeriv(c, p, s),
        (p, ChildNode::ElementNode(qn, c, atts, children)) => {
            let p1 = startTagOpeningDeriv(p, qn);
            let p2 = attsDeriv(c, p1, atts);
            let p3 = startTagCloseDeriv(p2);
            let p4 = childrenDeriv(c, p3, children);
            endTagDeriv(p4)
        }
    }

     */
}

fn startTagOpenDeriv(p: Pattern, q: QualifiedName) -> Pattern {
    println!("sto-p={:?}", &p);
    match p {
        Pattern::Choice(p1, p2) => {
            choice(
                startTagOpenDeriv(*p1, q.clone()),
                startTagOpenDeriv(*p2, q.clone())
            )
        }
        Pattern::Element(nc, p1) => {
            if contains(nc, q) {
                after(*p1, Pattern::Empty)
            } else {
                Pattern::NotAllowed
            }
        }
        Pattern::Interleave(p1, p2) => {
            choice(
                // applyAfter (flip interleave p2) (startTagOpenDeriv p1 qn)
                applyAfter(
                    |pat: Pattern| {Pattern::Interleave(Box::from(pat), p2.clone())},
                    startTagOpenDeriv(*p1.clone(), q.clone())
                ),
                applyAfter(
                    |pat: Pattern|{Pattern::Interleave(Box::from(pat), p1.clone())},
                    startTagOpenDeriv(*p2, q.clone())
                )
            )
        }
        Pattern::OneOrMore(p) => {
            applyAfter(
                Box::new(|pat: Pattern|{
                    group(
                        pat,
                        choice(
                            Pattern::OneOrMore(p.clone()),
                            Pattern::Empty
                        )
                    )
                }),
                startTagOpenDeriv(*p.clone(), q)
            )
        }
        Pattern::Group(p1, p2) => {
            let x = applyAfter(
                |pat: Pattern| { group(pat, *p2.clone())},
                startTagOpenDeriv(*p1.clone(), q.clone())
            );
            if p1.is_nullable(){
                choice(
                    x,
                    startTagOpenDeriv(*p2, q)
                )
            } else {
                x
            }
        }
        Pattern::After(p1, p2) => {
            applyAfter(
                |pat: Pattern|{after(pat, *p2.clone())},
                startTagOpenDeriv(*p1, q)
            )
        }
        _ => Pattern::NotAllowed
    }
}


fn attDeriv(pat: Pattern, att: RNode) -> Pattern {
    match pat {
        Pattern::After(p1, p2) => {
            after(
                attDeriv(*p1, att),
                *p2
            )
        },
        Pattern::Choice(p1, p2)=>{
            choice(
                attDeriv(*p1, att.clone()),
                attDeriv(*p2, att)
            )
        },
        Pattern::Group(p1, p2) => {
            choice(
                group(
                    attDeriv( *p1.clone(), att.clone()),
                    *p2.clone()
                ),
                group(
                    attDeriv( *p2, att),
                    *p1
                )
            )
        }
        Pattern::Interleave(p1, p2) => {
            choice(
                interleave(
                    attDeriv(*p1.clone(), att.clone()),
                    *p2.clone()
                ),
                interleave(
                    attDeriv(*p2, att),
                    *p1
                )
            )
        }
        Pattern::OneOrMore(p) => {
            group(
                attDeriv( *p.clone(), att),
                choice(
                    Pattern::OneOrMore(p),
                    Pattern::Empty
                )
            )
        }
        //attDeriv cx (Attribute nc p) (AttributeNode qn s) =
        //    if contains nc qn && valueMatch cx p s then Empty else NotAllowed
        Pattern::Attribute(nc, p) => {
            let (qn, av) = (att.name(), att.value());
            if contains(nc, qn) && valueMatch(*p, av.to_string()) {
                Pattern::Empty
            } else {
                Pattern::NotAllowed
            }
        }
        _ => Pattern::NotAllowed
    }
}

fn startTagCloseDeriv(pat: Pattern) -> Pattern {
    match pat {
        Pattern::After(p1, p2) => {
            after(startTagCloseDeriv(*p1), *p2)
        },
        Pattern::Choice(p1, p2) => {
            choice(
                startTagCloseDeriv(*p1),
                startTagCloseDeriv(*p2)
            )
        },
        Pattern::Group(p1, p2) => {
            group(
                startTagCloseDeriv(*p1),
                startTagCloseDeriv(*p2),
            )
        }
        Pattern::Interleave(p1, p2) => {
            interleave(
                startTagCloseDeriv(*p1),
                startTagCloseDeriv(*p2),
            )
        }
        Pattern::OneOrMore(p) => {
            oneOrMore(startTagCloseDeriv(*p))
        }
        Pattern::Attribute(_,_) => Pattern::NotAllowed,
        p => p
    }
}

fn childrenDeriv(pat: Pattern, cn: RNode) -> Pattern {
    match cn.clone().child_iter().count(){
        //match cn.len() {
        0 => {
            //We treat self closed elements as <e></e>
            choice(pat.clone(), textDeriv(pat, "".to_string()))
        }
        1 => {
            let n = cn.child_iter().next().unwrap();
            match n.node_type(){
                NodeType::Text => {
                    let p1 = childDeriv( pat.clone(), n.clone());
                    if whitespace(n.value().to_string()) {
                        choice(pat, p1)
                    } else {
                        p1
                    }
                }
                _ => {
                    stripChildrenDeriv(pat, cn.child_iter())
                }
            }
        },
        _ => {stripChildrenDeriv(pat, cn.child_iter())}
    }
}

fn endTagDeriv(pat: Pattern) -> Pattern {
    match pat {
        Pattern::Choice(p1, p2) => {
            choice(
                endTagDeriv(*p1),
                endTagDeriv(*p2)
            )
        },
        Pattern::After(p1, p2) => {
            if p1.is_nullable(){
                *p2
            }else {
                Pattern::NotAllowed
            }
        }
        _ => Pattern::NotAllowed
    }
}








fn textDeriv(pat: Pattern, s: String) -> Pattern {
    match pat {
        Pattern::Choice(p1, p2) => {
            choice(
                textDeriv(*p1, s.clone()),
                textDeriv(*p2, s.clone())
            )
        }
        Pattern::Interleave(p1, p2) => {
            choice(
                interleave(
                    textDeriv(*p1.clone(), s.clone()),
                    *p2.clone()),
                interleave(
                    *p1,
                    textDeriv(*p2, s.clone())
                )
            )
        }
        Pattern::Group(p1, p2) => {
            let p = group(
                textDeriv(*p1, s.clone()),
                *p2.clone()
            );
            if p.is_nullable(){
                choice(
                    p,
                    textDeriv(
                        *p2, s)
                )
            } else {
                p
            }
        }
        Pattern::After(p1, p2) => {
            //textDeriv cx (After p1 p2) s = after (textDeriv cx p1 s) p2
            after(
                textDeriv(*p1, s),
                *p2
            )
        }
        Pattern::OneOrMore(p1) => {
            group(
                textDeriv(*p1.clone(), s.clone()),
                choice(
                    Pattern::OneOrMore(p1),
                    Pattern::Empty
                )
            )
        }
        Pattern::Text => {
            Pattern::Text
        }
        Pattern::Value(dt, v, cx2) => {
            if datatypeEqual(dt, v,  s) {
                Pattern::Empty
            } else {
                Pattern::NotAllowed
            }
        }
        Pattern::Data(dt, params) => {
            if dataTypeAllows(dt, params, s) {
                Pattern::Empty
            } else {
                Pattern::NotAllowed
            }
        }
        Pattern::DataExcept(dt, params, p) => {
            if dataTypeAllows(dt, params, s.clone()) && !textDeriv(*p, s).is_nullable() {
                Pattern::Empty
            } else {
                Pattern::NotAllowed
            }
        }
        Pattern::List(p) => {
            if listDeriv(*p, stringsplit(s)).is_nullable(){
                Pattern::Empty
            } else {
                Pattern::NotAllowed
            }
        }
        _ => Pattern::NotAllowed
    }
}

fn listDeriv(p: Pattern, vs: Vec<String>) -> Pattern {
    let mut vsi = vs.into_iter();
    match vsi.next(){
        None => { p }
        Some(p1) => {
            listDeriv(textDeriv(p, p1), vsi.collect())
        }
    }
}

fn stripChildrenDeriv(pat: Pattern, mut cn: Box<dyn Iterator<Item=RNode>>) -> Pattern {
    match cn.next(){
        None => { pat },
        Some(h) => {
            stripChildrenDeriv(
                if strip(h.clone()){
                    pat
                } else {
                    childDeriv(pat, h)
                },
                cn
            )
        }
    }
}

pub fn applyAfter<F1>(
    f: F1,
    p: Pattern
) -> Pattern
    where
        F1: Fn(Pattern) -> Pattern + Clone,
{
    match p {
        Pattern::After(p1, p2) => {
            after(*p1, f(*p2))
        }
        Pattern::Choice(p1, p2) => {
            choice(
                applyAfter(f.clone(), *p1),
                applyAfter(f, *p2)
            )
        }
        Pattern::NotAllowed => Pattern::NotAllowed,
        _ => Pattern::NotAllowed
    }
}


fn choice(pat1: Pattern, pat2: Pattern) -> Pattern{
    /*
        choice :: Pattern -> Pattern -> Pattern
        choice p NotAllowed = p
        choice NotAllowed p = p
        choice p1 p2 = Choice p1 p2
    */
    match (pat1, pat2){
        (p1, Pattern::NotAllowed) =>  p1,
        (Pattern::NotAllowed, p2) =>  p2,
        (p1, p2) => Pattern::Choice(Box::from(p1), Box::from(p2))
    }
}
fn group(p1: Pattern, p2: Pattern) -> Pattern {
    match (p1, p2) {
        (_, Pattern::NotAllowed) => Pattern::NotAllowed,
        (Pattern::NotAllowed, _) => Pattern::NotAllowed,
        (p, Pattern::Empty) => p,
        (Pattern::Empty, p) => p,
        (p1, p2) => Pattern::Group(Box::from(p1), Box::from(p2))
    }
}
fn after(p1: Pattern, p2: Pattern) -> Pattern {
    match (p1, p2) {
        (_, Pattern::NotAllowed) => Pattern::NotAllowed,
        (Pattern::NotAllowed, _) => Pattern::NotAllowed,
        (p1, p2) => Pattern::After(Box::from(p1), Box::from(p2))
    }
}
fn interleave(p1: Pattern, p2: Pattern) -> Pattern {
    match (p1, p2) {
        (_, Pattern::NotAllowed) => Pattern::NotAllowed,
        (Pattern::NotAllowed, _) => Pattern::NotAllowed,
        (p, Pattern::Empty) => p,
        (Pattern::Empty, p) => p,
        (p1, p2) => Pattern::Interleave(Box::from(p1), Box::from(p2))
    }
}
fn valueMatch(pat: Pattern, s: String) -> bool {
    (pat.clone().is_nullable() && whitespace(s.clone()))
        ||
        textDeriv(pat, s).is_nullable()
}
fn whitespace(s: String) -> bool {
    //tests whether a string is contains only whitespace.
    !s.contains(|c| !char::is_whitespace(c))
}
fn strip(c: RNode) -> bool {
    match c.node_type() {
        NodeType::Text => {whitespace(c.value().to_string()) }
        _ => false
    }
}
fn oneOrMore(pat: Pattern) -> Pattern {
    match pat {
        Pattern::NotAllowed => Pattern::NotAllowed,
        p => Pattern::OneOrMore(Box::from(p))
    }
}
fn dataTypeAllows((dt, st): DataType, params: Vec<Param>, s: String) -> bool {
    match st.as_str(){
        "string" => true,
        "token" => true,
        _ => false
    }
}
fn datatypeEqual((d, s): DataType, s1: String, s2:String) -> bool {
    match s.as_str() {
        "string" => {s1 == s2},
        "token" => {
            normalizeWhitespace(s1) == normalizeWhitespace(s2)
        }
        _ => false
    }
}
fn normalizeWhitespace(s: String) -> String {
    s.trim().split(' ').filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ")
}
fn stringsplit(s: String) -> Vec<String>{
    let t = s.split(' ').map(|u| u.to_string()).collect();
    t
}
