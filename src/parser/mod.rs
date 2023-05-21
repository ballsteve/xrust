/*!
A parser combinator, inspired by nom.

This parser combinator passes a context into the function, which includes the string being parsed. This supports resolving context-based constructs such as general entities and XML Namespaces.
*/

use crate::intmuttree::DTD;
use std::collections::HashMap;
use std::fmt;
use crate::xdmerror::{Error, ErrorKind};

pub(crate) mod combinators;
pub(crate) mod common;
pub(crate) mod xml;


pub(crate) type ParseInput<'a> = (&'a str, ParserState);
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

#[derive(Clone)]
pub(crate) struct ParserState {
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
    /* eventual error location reporting */
    currentcol: usize,
    currentrow: usize,
    /* entity downloader function */
    entityresolver: Option<fn(String) -> Result<String, Error>>

}

impl ParserState {
    pub fn new(resolver: Option<fn(String) -> Result<String, Error>>) -> ParserState {
        return ParserState {
            dtd: DTD::new(),
            /*
            The below hashmap
             */
            namespace: vec![],
            maxentitydepth: 4,
            currententitydepth: 1,
            currentcol: 1,
            currentrow: 1,
            entityresolver: resolver
        };
    }

    pub fn resolve(self, uri: String) -> Result<String, Error> {
        match self.entityresolver {
            None => Err(Error {
                kind: ErrorKind::Unknown,
                message: "External DTDs are not supported".to_string(),
            }),
            Some(e) => { e(uri) }
        }
    }
}

impl PartialEq for ParserState {
    fn eq(&self, other: &ParserState) -> bool {
        true
    }
}

impl fmt::Debug for ParserState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParserState").finish()
    }
}