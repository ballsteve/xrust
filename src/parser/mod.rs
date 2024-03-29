/*!
A parser combinator, inspired by nom.

This parser combinator passes a context into the function, which includes the string being parsed. This supports resolving context-based constructs such as general entities and XML Namespaces.
*/

use crate::trees::intmuttree::{ExtDTDresolver, DTD};
use crate::xdmerror::{Error, ErrorKind};
use std::collections::HashMap;
use std::fmt;

pub(crate) mod avt;
pub mod combinators;
pub(crate) mod common;
pub mod xml;
pub mod xpath;

pub type ParseInput<'a> = (&'a str, ParserState);
pub type ParseResult<'a, Output> = Result<(ParseInput<'a>, Output), ParseError>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ParseError {
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
    //Unknown { row: usize, col: usize },
    MissingNameSpace,
    IncorrectArguments,
    NotWellFormed,
    Unbalanced,
    Notimplemented,
    ExtDTDLoadError,
}

#[derive(Clone)]
pub struct ParserState {
    dtd: DTD,
    /*
    The namespaces are tracked in a hashmap of vectors, each prefix tracking which namespace you
    are dealing with in case aliases are redeclared in the child elements.
    NOTE: the "xmlns" vector in this hashmap is NOT the real xml namespace prefix, it is used to
    track the namespace when no alias is declared with the namespace.
     */
    namespace: Vec<HashMap<String, String>>,
    standalone: bool,
    xmlversion: String,
    /*
    The below will track Entity Expansion, ensuring that there are no recursive entities and
    some protections from zip bombs
     */
    maxentitydepth: usize,
    currententitydepth: usize,
    /* eventual error location reporting */
    currentcol: usize,
    currentrow: usize,
    /* For tracking down stack overflows */
    //stack: Vec<String>,
    //limit: Option<usize>,
    /* entity downloader function */
    ext_dtd_resolver: Option<ExtDTDresolver>,
    ext_entities_to_parse: Vec<String>,
    docloc: Option<String>,
    /*
    ParamEntities are not allowed in internal subsets, but they are allowed in external DTDs,
    so we need to track when we are currently in the main document or outside it.
     */
    currentlyexternal: bool,
}

impl ParserState {
    pub fn new(resolver: Option<ExtDTDresolver>, docloc: Option<String>) -> Self {
        ParserState {
            dtd: DTD::new(),
            standalone: false,
            xmlversion: "1.0".to_string(), // Always assume 1.0
            /*
            The below hashmap
             */
            namespace: vec![],
            maxentitydepth: 4,
            currententitydepth: 1,
            currentcol: 1,
            currentrow: 1,
            //stack: vec![],
            //limit: None,
            ext_dtd_resolver: resolver,
            ext_entities_to_parse: vec![],
            docloc,
            currentlyexternal: false,
        }
    }
    //pub fn stack_push(&mut self, msg: String) {
    //    self.stack.push(msg);
    //    if self.limit.is_some() {
    //        if self.limit.unwrap() < self.stack.len() {
    //            panic!("stack depth exceeded")
    //        }
    //    }
    //}
    //pub fn stack_depth(&self) -> usize {
    //    self.stack.len()
    //}
    //pub fn set_limit(&mut self, l: usize) {
    //    self.limit = Some(l)
    //}

    pub fn resolve(self, locdir: Option<String>, uri: String) -> Result<String, Error> {
        match self.ext_dtd_resolver {
            None => Err(Error::new(
                ErrorKind::Unknown,
                "No external DTD resolver provided.".to_string(),
            )),
            Some(e) => e(locdir, uri),
        }
    }
}

impl PartialEq for ParserState {
    fn eq(&self, _: &ParserState) -> bool {
        true
    }
}

impl fmt::Debug for ParserState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParserState").finish()
    }
}
