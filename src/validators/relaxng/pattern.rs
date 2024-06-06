use crate::item::{Node, NodeType};
use crate::qname::QualifiedName;
use crate::trees::smite::RNode;

pub(crate) type DataType = (QualifiedName, String);
type Attributenode = (QualifiedName, String);
pub(crate) type Param = (String, String);
type Context = (QualifiedName, Vec<(String, QualifiedName)>);

#[derive(Clone, Debug)]
pub(crate) enum NameClass {
    AnyName,
    AnyNameExcept(Box<NameClass>),
    Name(Option<String>, String),
    NSName(String),
    NSNameExcept(String, Box<NameClass>),
    NameClassChoice(Box<NameClass>, Box<NameClass>)
}

#[derive(Clone, Debug)]
pub(crate) enum Pattern {
    Empty,
    NotAllowed,
    Text,
    Choice(Box<Pattern>, Box<Pattern>),
    Interleave(Box<Pattern>, Box<Pattern>),
    Group(Box<Pattern>, Box<Pattern>),
    OneOrMore(Box<Pattern>),
    List(Box<Pattern>),
    Data(DataType, Vec<Param>),
    DataExcept(DataType, Vec<Param>, Box<Pattern>),
    Value(DataType, String, Context),
    Attribute(NameClass, Box<Pattern>),
    Element(NameClass, Box<Pattern>),
    After(Box<Pattern>, Box<Pattern>)
}

#[derive(Debug)]
pub(crate) enum PatternError<'a>{
    NotRelaxNG,
    MissingName,
    Other(&'a str)
}

impl Pattern{
    pub(crate) fn is_nullable(&self) -> bool {
        match self {
            Pattern::Group(p1, p2) => {p1.is_nullable() && p2.is_nullable()},
            Pattern::Interleave(p1, p2) => {p1.is_nullable() && p2.is_nullable()},
            Pattern::Choice(p1, p2) => {p1.is_nullable() || p2.is_nullable()},
            Pattern::OneOrMore(p) => { p.is_nullable()},
            Pattern::Empty => true,
            Pattern::Text => true,
            Pattern::Element(_, _) => false,
            Pattern::Attribute(_, _) => false,
            Pattern::List(_) => false,
            Pattern::Value(_, _, _) => false,
            Pattern::Data(_, _) => false,
            Pattern::DataExcept(_, _, _) => false,
            Pattern::NotAllowed => false,
            Pattern::After(_, _) => false
        }
    }

}

pub(crate) fn patternmaker(n: RNode) -> Result<Pattern, PatternError<'static>>{
    //let _ =
    match n.node_type(){
        NodeType::Document => {
            patternmaker(n.child_iter().next().unwrap())
        }
        NodeType::Element => {
            if n.name().get_nsuri() != Some("http://relaxng.org/ns/structure/1.0".to_string()) {
                Err(PatternError::NotRelaxNG)
            } else {
                match n.name().get_localname().as_str() {
                    "empty" => Ok(Pattern::Empty),
                    "element" => {
                        patternmaker_element(n)
                    }
                    _ => Ok(Pattern::Empty)
                }
            }

           // Err(PatternError::Other("not yet implemented"))
        }
        NodeType::Text => {
            Err(PatternError::NotRelaxNG)
        }
        NodeType::Attribute => {
            Err(PatternError::Other("not yet implemented"))
        }
        NodeType::Comment => {
            Err(PatternError::Other("not yet implemented"))
        }
        NodeType::ProcessingInstruction => {
            Err(PatternError::Other("not yet implemented"))
        }
        NodeType::Reference => {
            Err(PatternError::Other("not yet implemented"))
        }
        NodeType::Unknown => {
            Err(PatternError::Other("not yet implemented"))
        }
    }
    //;
    //Ok((Pattern::Empty))
}


fn patternmaker_element(n: RNode) -> Result<Pattern, PatternError<'static>> {

    let nsattr =  n.get_attribute(&QualifiedName::new(None, None, String::from("ns"))).to_string();
    let namespace = if nsattr.is_empty(){
        None
    } else {
        Some(nsattr)
    };
    let mut name =  n.get_attribute(&QualifiedName::new(None, None, String::from("name"))).to_string();
    let mut res = vec![];
    let mut children = n.child_iter();
    match children.next() {
        None => {
            if name.is_empty(){
                return Err(PatternError::MissingName)
            }
        }
        Some(rn) => {
            match (rn.name().get_nsuri().unwrap_or("".to_string()).as_str(), rn.name().get_localname().as_str()) {
                ("http://relaxng.org/ns/structure/1.0", "name") => {
                    name = rn.to_string();
                },
                ("http://relaxng.org/ns/structure/1.0", _) => {
                    match patternmaker(rn){
                        Err(PatternError::NotRelaxNG) => {}
                        Err(e) => {return Err(e)}
                        Ok(pat) => {res.push(pat)}
                    }
                },
                (_,_) => {}
            }
        }
    }
    for child in children{
        match patternmaker(child){
            Err(PatternError::NotRelaxNG) => {}
            Err(e) => {return Err(e)}
            Ok(pat) => {res.push(pat)}
        }
    }

    let p: Result<Pattern, PatternError>;

    match res.len(){
        0 => {p = Ok(Pattern::Empty)}
        1 => {p = Ok(res.iter().next().unwrap().clone())}
        _ => {
            p = patternmaker_group(res)
        }
    };
    match p {
        Ok(p1) => {
            Ok(Pattern::Element(
                NameClass::Name(
                    namespace,
                    name
                ),
                Box::new(p1))
            )
        }
        Err(e) => { Err(e)}
    }
}

fn patternmaker_group(nodes: Vec<Pattern>) -> Result<Pattern, PatternError<'static>> {
    if nodes.len() > 2 {
        let mut nodesi = nodes.into_iter();
        let first = nodesi.next().unwrap();
        match patternmaker_group(nodesi.collect()) {
            Ok(rest) => {
                Ok(
                    Pattern::Group(
                        Box::new(first),
                        Box::new(rest )
                    )
                )
            }
            Err(e) => Err(e)
        }
    } else {
        Ok(
            Pattern::Group(
                Box::new(nodes.first().unwrap().clone()),
                Box::new(nodes.last().unwrap().clone() )
            )
        )
    }
}