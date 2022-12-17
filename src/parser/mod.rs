use std::collections::HashMap;
use crate::intmuttree::DTD;
use std::str::Chars;

pub(crate) mod combinators;
pub(crate) mod common;
pub(crate) mod xml;

pub(crate) type ParseInput<'a> = (Parserinput<'a>, usize);
pub(crate) type ParseResult<'a, Output> = Result<(Parserinput<'a>, usize, Output), usize>;


#[derive(Clone, Debug)]
pub(crate) struct Parserinput<'a> {
    entityfeed: Vec<char>,
    input: Chars<'a>,
    dtd: DTD,
    /*
    The namespaces are tracked in a hashmap of vectors, each prefix tracking which namespace you
    are dealing with in case aliases are redeclared in the child elements.
    NOTE: the "xmlns" vector in this hashmap is NOT the real xml namespace prefix, it is used to
    track the namespace when no alias is declared with the namespace.
     */
    namespace: HashMap<String, Vec<String>>,
    /*
    The below will track Entity Expansion, ensuring that there are no recursive entities and
    some protections from zip bombs
     */
    maxentitydepth: usize,
    currententitydepth: usize,
}

impl Parserinput<'_> {
    pub fn new(xmldoc: &str) -> Parserinput {
        return Parserinput {
            entityfeed: vec![],
            input: xmldoc.chars(),
            dtd: DTD::new(),
            /*
            The below hashmap
             */
            namespace: HashMap::from([
                                         ("xmlns".to_string(), vec![])
                                     ]),
            maxentitydepth: 4,
            currententitydepth: 0,
        };
    }
}

impl<'a> Iterator for Parserinput<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        match &self.entityfeed.pop() {
            Some(c) => Some(*c),
            None => {
                if self.currententitydepth > 0 {
                    self.currententitydepth = 0;
                }
                self.input.next()
            }
        }
    }
}

impl PartialEq for Parserinput<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.entityfeed == other.entityfeed
    }
}
