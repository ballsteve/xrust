use std::collections::HashMap;
use std::rc::Rc;
use crate::item::NodeType;
use crate::{Node, Value};
use crate::qname::QualifiedName;
use crate::trees::smite::{RNode};
use crate::validators::relaxng::pattern::{DataType, Param};
use crate::trees::smite::{Node as SmiteNode};

pub(crate) fn derive(doc: &RNode, pat: RNode, refs: &HashMap<String, RNode>) -> RNode {
    childDeriv( doc.child_iter().next().unwrap(),pat, refs)
}


pub(crate) fn is_nullable(pat: RNode) -> bool {
    match pat.name().get_localname().as_str() {
        "Empty" => true,
        "Text" => true,
        "Group" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            is_nullable(p1) && is_nullable(p2)
        }
        "Interleave" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            is_nullable(p1) && is_nullable(p2)
        }
        "Choice" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            is_nullable(p1) || is_nullable(p2)
        }
        "OneOrMore" => {
            is_nullable(pat.first_child().unwrap())
        }
        //"Element"
        //"Attribute"
        //"List"
        //"Value"
        //"Data"
        //"DataExcept"
        //"NotAllowed"
        //"After"
        _ => false
    }
}

fn contains(nc: RNode, qn: QualifiedName) -> bool {
    match nc.name().get_localname().as_str() {
        "AnyName" => true,
        "AnyNameExcept" => {
            let name = nc.first_child().unwrap();
            !contains(name, qn)
        }
        "NSName" => {
            let nsuri = nc.first_child().unwrap();
            Some(nsuri.to_string()) == qn.get_nsuri()
        }
        "NSNameExcept" => {
            let mut c = nc.child_iter();
            let ns1 = c.next().unwrap();
            let n = c.next().unwrap();
            (Some(ns1.to_string()) ==qn.get_nsuri()) && !contains(n, qn)
        }
        "Name" => {
            let mut c = nc.child_iter();
            let ns1 = c.next().unwrap();
            let ln1 = c.next().unwrap();
            (Some(ns1.to_string()) == qn.get_nsuri()) && (ln1.to_string() ==qn.get_localname())
        }
        "NameClassChoice" => {
            let mut c = nc.child_iter();
            let nc1 = c.next().unwrap();
            let nc2 = c.next().unwrap();
            contains(nc1,qn.clone()) || contains(nc2, qn)
        }
        _ => false
    }
}


fn childDeriv(pat: RNode, cn: RNode, refs: &HashMap<String, RNode>) -> RNode {
    match cn.node_type(){
        NodeType::Document |
        NodeType::Attribute |
        NodeType::Comment |
        NodeType::ProcessingInstruction |
        NodeType::Reference |
        NodeType::Unknown |
        NodeType::Namespace => pat.owner_document().new_element(QualifiedName::new(None,None,"NotAllowed".to_string())).unwrap(),
        NodeType::Text =>  textDeriv(pat,cn.value().to_string()),
        NodeType::Element => {
            //opening deriv
            let mut pat1 = startTagOpenDeriv(pat, cn.name(), refs);
            //attsDeriv
            for attribute in cn.attribute_iter() {
                pat1 = attDeriv(attribute,pat1);
            }
            //CloseTag
            pat1 = startTagCloseDeriv(pat1);
            //Children
            pat1 = childrenDeriv(cn.clone(), pat1, refs);
            //EndTagDeriv
            pat1 = endTagDeriv(pat1);
            pat1
        }
    }
}

fn startTagOpenDeriv(pat: RNode, q: QualifiedName, refs: &HashMap<String, RNode>) -> RNode {
    match pat.name().get_localname().as_str() {
        "ref" => {
            //We lookup the reference, and use that for the pattern going forward
            let patname = pat.get_attribute(&QualifiedName::new(None,None,"name".to_string()));
            let newpat = refs.get(patname.to_string().as_str());
            match newpat{
                //TODO proper error checking
                None => pat.owner_document().new_element(QualifiedName::new(None,None,"NotAllowed".to_string())).unwrap(),
                Some(RN) => {
                    startTagOpenDeriv(RN.clone(), q, refs)
                }
            }
        }
        "element" =>{
            let mut pc = pat.child_iter();
            let nc = pc.next().unwrap();
            let p = pc.next().unwrap();
            if contains(nc, q) {
                after(p, pat.owner_document().new_element(QualifiedName::new(None,None,"NotAllowed".to_string())).unwrap())
            } else {
                pat.owner_document().new_element(QualifiedName::new(None,None,"NotAllowed".to_string())).unwrap()
            }
        }
        "choice" => {
            let mut pc = pat.child_iter();
            choice(
                startTagOpenDeriv( pc.next().unwrap(), q.clone(),refs),
                startTagOpenDeriv( pc.next().unwrap(), q.clone(),refs)
            )
        }
        "interleave" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(
                applyAfter(
                |pat: RNode| {
                    let mut i = pat.owner_document().new_element(QualifiedName::new(None,None,"Interleave".to_string())).unwrap();
                    i.push(pat);
                    i.push(p2.clone());
                    i
                },
                    startTagOpenDeriv( p1.clone(), q.clone(),refs)
                ),
                applyAfter(
                    |pat: RNode| {
                        let mut i = pat.owner_document().new_element(QualifiedName::new(None,None,"Interleave".to_string())).unwrap();
                        i.push(pat);
                        i.push(p1.clone());
                        i
                    },
                    startTagOpenDeriv( p2.clone(),q.clone(), refs)
                )
            )
        }
        "OneOrMore" => {
            let p1 = pat.first_child().unwrap();
            applyAfter(
                |pat: RNode|{
                    group(
                        pat.clone(),
                        choice(
                            pat.clone(),
                            pat.owner_document().new_element(QualifiedName::new(None,None,"Empty".to_string())).unwrap()
                        )
                    )
                },
                startTagOpenDeriv(p1, q, refs)
            )
        }
        "Group" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            let x =  applyAfter(
                |pat: RNode| { group(pat, p2.clone())},
                startTagOpenDeriv( p1.clone(),q.clone(), refs)
            );
            if is_nullable(p1){
                choice(
                    x,
                    startTagOpenDeriv( p2, q,refs)
                )
            } else{
                x
            }
        }
        "After" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            applyAfter(
                |pat: RNode| {
                    after(pat,  p2.clone())
                },
                startTagOpenDeriv( p1, q,refs)
            )
        }
        _ => pat.owner_document().new_element(QualifiedName::new(None,None,"NotAllowed".to_string())).unwrap()
    }
}


fn attDeriv( pat: RNode, att: RNode) -> RNode {
    match pat.name().get_localname().as_str() {
        "After" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            after(
                attDeriv( p1, att),
                p2
            )
        }
        "Choice" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(
                attDeriv( p1, att.clone()),
                attDeriv(p2, att)
            )
        }
        "Group" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(
                group(
                    attDeriv( p1.clone(), att.clone()),
                    p2.clone()
                ),
                group(
                    attDeriv( p2, att),
                    p1
                )
            )
        }
        "Interleave" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(
                interleave(
                    attDeriv(p1.clone(), att.clone()),
                    p2.clone()
                ),
                interleave(
                    attDeriv( p2, att),
                    p1
                )
            )
        }
        "OneOrMore" => {
            let p1 = pat.first_child().unwrap();
            group(
                attDeriv(p1,  att),
                choice(
                    pat.clone(),
                    pat.owner_document().new_element(QualifiedName::new(None,None,"Empty".to_string())).unwrap()
                )
            )
        }
        "Attribute" => {
            let (qn, av) = (att.name(), att.value());
            let mut i = pat.child_iter();
            let nc = i.next().unwrap();
            let p1 = i.next().unwrap();
            if contains(nc, qn) && valueMatch(p1, av.to_string()) {
                pat.owner_document().new_element(QualifiedName::new(None,None,"Empty".to_string())).unwrap()
            } else {
                pat.owner_document().new_element(QualifiedName::new(None,None,"NotAllowed".to_string())).unwrap()
            }
        }
        _ => pat.owner_document().new_element(QualifiedName::new(None,None,"NotAllowed".to_string())).unwrap()
    }
}

fn startTagCloseDeriv(pat: RNode) -> RNode {
    match pat.name().get_localname().as_str() {
        "After" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            after(
                startTagCloseDeriv(p1),
                p2
            )
        }
        "Choice" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(
                startTagCloseDeriv(p1),
                startTagCloseDeriv(p2)
            )
        }
        "Group" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            group(
                startTagCloseDeriv(p1),
                startTagCloseDeriv(p2)
            )
        }
        "Interleave" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            interleave(
                startTagCloseDeriv(p1),
                startTagCloseDeriv(p2)
            )
        }
        "OneOrMore" => {
            let p = pat.first_child().unwrap();
            oneOrMore(startTagCloseDeriv(p))
        }
        "Attribute" => {
            pat.owner_document().new_element(QualifiedName::new(None,None,"NotAllowed".to_string())).unwrap()
        }
        _ => pat
    }
}

fn childrenDeriv(pat: RNode, cn: RNode, refs: &HashMap<String, RNode>) -> RNode {
    match cn.clone().child_iter().count(){
        //match cn.len() {
        0 => {
            //We treat self closed elements as <e></e>
            choice(pat.clone(), textDeriv(pat, "".to_string()))
        }
        1 => {
            let n = cn.first_child().unwrap();
            match n.node_type(){
                NodeType::Text => {
                    let p1 = childDeriv( n.clone(),pat.clone(), refs);
                    if whitespace(n.value().to_string()) {
                        choice(pat, p1)
                    } else {
                        p1
                    }
                }
                _ => {
                    stripChildrenDeriv(pat, cn.child_iter(), refs)
                }
            }
        },
        _ => {stripChildrenDeriv(pat, cn.child_iter(), refs)}
    }
}

fn endTagDeriv(pat: RNode) -> RNode {
    match pat.name().get_localname().as_str() {
        "Choice" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(
                endTagDeriv(p1),
                endTagDeriv(p2)
            )
        }
        "After" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            if is_nullable(p1) {
                p2
            } else {
                pat.owner_document().new_element(QualifiedName::new(None,None,"NotAllowed".to_string())).unwrap()
            }
        }
        _ => pat.owner_document().new_element(QualifiedName::new(None,None,"NotAllowed".to_string())).unwrap()
    }
}



fn textDeriv(pat: RNode, s: String) -> RNode {
    match pat.name().get_localname().as_str() {
        "Choice" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(
                textDeriv(p1, s.clone()),
                textDeriv(p2, s.clone())
            )
        }
        "Interleave" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(
                interleave(
                    textDeriv(p1.clone(), s.clone()),
                    p2.clone()),
                interleave(
                    p1,
                    textDeriv(p2, s.clone())
                )
            )
        }
        "Group" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            let p = group(
                textDeriv(p1, s.clone()),
                p2.clone()
            );
            if is_nullable(p.clone()){
                choice(
                    p.clone(),
                    textDeriv(
                        p2, s)
                )
            } else {
                p
            }
        }
        "After" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            after(
                textDeriv(p1, s),
                p2
            )
        }
        "OneOrMore" => {
            let p = pat.first_child().unwrap();
            group(
                textDeriv(p.clone(), s.clone()),
                choice(
                    pat.clone(),
                    pat.owner_document().new_element(QualifiedName::new(None,None,"Empty".to_string())).unwrap()
                )
            )
        }
        "Text" => pat,
        "Value" => {
            let dtlib = pat.get_attribute(&QualifiedName::new(None, None, "datatypeLibrary".to_string()));
            let dtname = pat.get_attribute(&QualifiedName::new(None, None, "type".to_string()));
            let v = pat.value().to_string();
            if datatypeEqual((dtlib,dtname), v,  s) {
                pat.owner_document().new_element(QualifiedName::new(None,None,"Empty".to_string())).unwrap()
            } else {
                pat.owner_document().new_element(QualifiedName::new(None,None,"NotAllowed".to_string())).unwrap()
            }
        }
        "Data" => {
            let mut c = pat.child_iter();
            let dt = c.next().unwrap();
            //let params = c.collect();
            let params = vec![];
            if dataTypeAllows(dt, params, s) {
                pat.owner_document().new_element(QualifiedName::new(None,None,"Empty".to_string())).unwrap()
            } else {
                pat.owner_document().new_element(QualifiedName::new(None,None,"NotAllowed".to_string())).unwrap()
            }
        }
        "DataExcept" => {
            let mut c = pat.clone().child_iter();
            let dt = c.next().unwrap();
            //let params = c.collect();
            let params = vec![];
            if dataTypeAllows(dt, params, s.clone()) && !is_nullable(textDeriv(pat.clone(), s)) {
                pat.owner_document().new_element(QualifiedName::new(None,None,"Empty".to_string())).unwrap()
            } else {
                pat.owner_document().new_element(QualifiedName::new(None,None,"NotAllowed".to_string())).unwrap()
            }
        }
        "List" => {
            let p = pat.first_child().unwrap();
            if is_nullable(listDeriv(p, stringsplit(s))){
                pat.owner_document().new_element(QualifiedName::new(None,None,"Empty".to_string())).unwrap()
            } else {
                pat.owner_document().new_element(QualifiedName::new(None,None,"NotAllowed".to_string())).unwrap()
            }
        }
        _ => pat.owner_document().new_element(QualifiedName::new(None,None,"NotAllowed".to_string())).unwrap()
    }
}

fn listDeriv(p: RNode, vs: Vec<String>) -> RNode {
    let mut vsi = vs.into_iter();
    match vsi.next(){
        None => { p }
        Some(p1) => {
            listDeriv(textDeriv(p, p1), vsi.collect())
        }
    }
}

fn stripChildrenDeriv(pat: RNode, mut cn: Box<dyn Iterator<Item=RNode>>, refs: &HashMap<String, RNode>) -> RNode {
    match cn.next(){
        None => { pat },
        Some(h) => {
            stripChildrenDeriv(
                if strip(h.clone()){
                    pat
                } else {
                    childDeriv(pat, h, refs)
                },
                cn,
                refs
            )
        }
    }
}

pub fn applyAfter<F1>(
    f: F1,
    pat: RNode
) -> RNode
    where
        F1: Fn(RNode) -> RNode + Clone,
{
    match pat.name().get_localname().as_str() {
        "After" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            after(
                p1,
                f(p2)
            )
        }
        "Choice" => {
            let mut pc = pat.child_iter();
            let p1 = pc.next().unwrap();
            let p2 = pc.next().unwrap();
            choice(
                applyAfter(f.clone(), p1),
                applyAfter(f, p2)
            )
        }
        "NotAllowed" => pat,
        _ => pat.owner_document().new_element(QualifiedName::new(None,None,"NotAllowed".to_string())).unwrap()
    }
}


fn choice(pat1: RNode, pat2: RNode) -> RNode{
    /*
        choice :: Pattern -> Pattern -> Pattern
        choice p NotAllowed = p
        choice NotAllowed p = p
        choice p1 p2 = Choice p1 p2
    */
    match (pat1.name().get_localname().as_str(), pat2.name().get_localname().as_str()){
        ("NotAllowed", _) =>  pat2,
        (_, "NotAllowed") =>  pat1,
        (_, _) => {
            let mut c = pat1.owner_document().new_element(QualifiedName::new(None,None,"Choice".to_string())).unwrap();
            c.push(pat1);
            c.push(pat2);
            c
        }
    }
}
fn group(pat1: RNode, pat2: RNode) -> RNode {
    match (pat1.name().get_localname().as_str(), pat2.name().get_localname().as_str()) {
        ("NotAllowed", _) => pat1,
        (_,"NotAllowed") => pat2,
        ("Empty",_) => pat2,
        (_,"Empty") => pat1,
        (_,_) => {
            let mut g = pat1.owner_document().new_element(QualifiedName::new(None,None,"Group".to_string())).unwrap();
            g.push(pat1);
            g.push(pat2);
            g
        }
    }
}
fn after(pat1: RNode, pat2: RNode) -> RNode {
    match (pat1.name().get_localname().as_str(), pat2.name().get_localname().as_str()) {
        (_, "NotAllowed") => pat2,
        ("NotAllowed", _) => pat1,
        (_,_) => {
            let mut a = pat1.owner_document().new_element(QualifiedName::new(None,None,"NotAllowed".to_string())).unwrap();
            a.push(pat1);
            a.push(pat2);
            a
        }
    }
}
fn interleave(pat1: RNode, pat2: RNode) -> RNode {
    match (pat1.name().get_localname().as_str(), pat2.name().get_localname().as_str()) {
        ("NotAllowed", _) => pat1,
        (_,"NotAllowed") => pat2,
        ("Empty",_) => pat2,
        (_,"Empty") => pat1,
        (_,_) => {
            let mut i = pat1.owner_document().new_element(QualifiedName::new(None,None,"Interleave".to_string())).unwrap();
            i.push(pat1);
            i.push(pat2);
            i
        }
    }
}
fn valueMatch(pat: RNode, s: String) -> bool {
    (is_nullable(pat.clone()) && whitespace(s.clone()))
        ||
    is_nullable(textDeriv(pat, s))
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
fn oneOrMore(pat: RNode) -> RNode {
    match pat.name().get_localname().as_str() {
        "NotAllowed" => pat,
        _ => {
            let mut o = Rc::new(SmiteNode::new()).new_element(QualifiedName::new(None, None, "OneOrMore".to_string())).unwrap();
            o.push(pat);
            o
        }
    }
}
fn dataTypeAllows(dt: RNode, params: Vec<Param>, s: String) -> bool {
    let datatypens = dt.name().get_nsuri();
    let datatype = dt.name().get_localname();
    match datatype.as_str(){
        "string" => true,
        "token" => true,
        _ => false
    }
}
fn datatypeEqual((d, s): (Rc<Value>,Rc<Value>), s1: String, s2:String) -> bool {
    match s.as_ref() {
        Value::String(_) => { s1 == s2}
        Value::Token => {
            normalizeWhitespace(s1) == normalizeWhitespace(s2)
        }
        _ => false
    }
    /*
    match s.as_str() {
        "string" => {s1 == s2},
        "token" => {
            normalizeWhitespace(s1) == normalizeWhitespace(s2)
        }
        _ => false
    }

     */
}
fn normalizeWhitespace(s: String) -> String {
    s.trim().split(' ').filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ")
}
fn stringsplit(s: String) -> Vec<String>{
    let t = s.split(' ').map(|u| u.to_string()).collect();
    t
}
