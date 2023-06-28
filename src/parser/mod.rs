/*!
A parser combinator, inspired by nom.

This parser combinator passes a context into the function, which includes the string being parsed. This supports resolving context-based constructs such as general entities and XML Namespaces.
*/

use crate::intmuttree::DTD;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::str::Chars;

pub(crate) mod combinators;
pub(crate) mod common;
pub(crate) mod xml;

//pub(crate) type ParseInput<'a> = (Parserinput<'a>, usize);
pub(crate) type ParseResult<'a, Output> = Result<(ParseInput<'a>, Output), ParseError>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum ParseError {
    // The "Combinator" error just means a parser hasn't matched, its not serious necessarily.
    // Every other error should get returned.
    Combinator, // Combinator isn't correct, not a serious error.
    //InvalidChar{ row:usize, col:usize },
    //MissingClosingElement{ row:usize, col:usize, element: String},
    //IncorrectClosingElement{ row:usize, col:usize, open: String, close:String},
    MissingGenEntity { row: usize, col: usize },
    MissingParamEntity { row: usize, col: usize },
    EntityDepth { row: usize, col: usize },
    Validation { row: usize, col: usize },
    Unknown { row: usize, col: usize },
    MissingNameSpace,
    NotWellFormed,
    Notimplemented,
}

#[derive(Clone, Debug)]
pub(crate) struct ParseInput<'a> {
    entityfeed: Vec<char>,
    input: Chars<'a>,
    dtd: DTD,
    /*
    The namespaces are tracked in a hashmap of vectors, each prefix tracking which namespace you
    are dealing with in case aliases are redeclared in the child elements.
    NOTE: the "xmlns" vector in this hashmap is NOT the real xml namespace prefix, it is used to
    track the namespace when no alias is declared with the namespace.
     */
    namespace: Vec<HashMap<String, String>>,
    /*
    The below will track Entity Expansion, ensuring that there are no recursive entities and
    some protections from zip bombs
     */
    maxentitydepth: usize,
    currententitydepth: usize,
    currentcol: usize,
    currentrow: usize,
}

impl ParseInput<'_> {
    pub fn new(xmldoc: &str) -> ParseInput {
        return ParseInput {
            entityfeed: vec![],
            input: xmldoc.chars(),
            dtd: DTD::new(),
            /*
            The below hashmap
             */
            namespace: vec![],
            maxentitydepth: 4,
            currententitydepth: 0,
            currentcol: 1,
            currentrow: 1,
        };
    }
}

impl<'a> Iterator for ParseInput<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        match &self.entityfeed.pop() {
            Some(c) => Some(*c),
            None => {
                if self.currententitydepth > 0 {
                    self.currententitydepth = 0;
                }
                match self.input.next() {
                    Some('\n') => {
                        self.currentrow += 1;
                        self.currentcol = 1;
                        Some('\n')
                    }
                    Some(c) => {
                        self.currentcol += 1;
                        Some(c)
                    }
                    None => None,
                }
            }
        }
    }
}

impl PartialEq for ParseInput<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.entityfeed == other.entityfeed
    }
}

impl fmt::Display for ParseInput<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.input.as_str())
    }
}
