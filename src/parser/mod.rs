/*!
A parser combinator, inspired by nom.

This parser combinator passes a context into the function, which includes the string being parsed. This supports resolving context-based constructs such as general entities and XML Namespaces.
*/

use crate::externals::URLResolver;
use crate::item::Node;
use crate::xdmerror::{Error, ErrorKind};
use crate::xmldecl::DTD;
use qualname::{NamespaceMap, NamespacePrefix, NamespaceUri};
use std::collections::HashSet;
use std::fmt;

pub mod avt;
pub mod combinators;
pub(crate) mod common;
pub mod xml;
pub mod xpath;

pub mod datetime;

#[allow(type_alias_bounds)]
pub type ParseInput<'a, N: Node> = (&'a str, ParserState<N>);

#[allow(type_alias_bounds)]
pub type ParseResult<'a, N: Node, Output> = Result<(ParseInput<'a, N>, Output), ParseError>;

#[derive(Clone, Debug, PartialEq)]
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
    // An unexpected character has been encountered
    NotWellFormed(String),
    // An attribute has been declared more than once
    DuplicateAttribute(String),
    Unbalanced,
    Notimplemented,
    ExtDTDLoadError,
    NSResolveError(String),
    IDError(String),
}

/// Parser state configuration that cannot be cloned.
/// Also state that needs to be persistent during parsing.
pub struct StaticState<L>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    // Tracking ID-type attributes
    ids_read: HashSet<String>,
    ids_pending: HashSet<String>,

    /*
       A method for resolving a prefix to a namespace URI.
    */
    pub namespace: Option<L>,

    /* entity downloader function */
    pub ext_dtd_resolver: Option<URLResolver>,
}

impl<L> StaticState<L>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    pub fn new() -> Self {
        Self {
            namespace: None,
            ext_dtd_resolver: None,
            ids_read: Default::default(),
            ids_pending: Default::default(),
        }
    }
    pub fn resolve(&self, locdir: Option<String>, uri: String) -> Result<String, Error> {
        self.ext_dtd_resolver.map_or(
            Err(Error::new(
                ErrorKind::Unknown,
                "No external DTD resolver provided.".to_string(),
            )),
            |e| e(locdir, uri),
        )
    }
}

pub struct StaticStateBuilder<L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>>(
    StaticState<L>,
);

impl<L> StaticStateBuilder<L>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    pub fn new() -> Self {
        StaticStateBuilder(StaticState::new())
    }
    pub fn namespace(mut self, n: L) -> Self {
        self.0.namespace = Some(n);
        self
    }
    pub fn dtd_resolver(mut self, r: URLResolver) -> Self {
        self.0.ext_dtd_resolver = Some(r);
        self
    }
    pub fn build(self) -> StaticState<L> {
        self.0
    }
}

/// Parser state that can be cloned
#[derive(Clone)]
pub struct ParserState<N: Node> {
    // Document node to use to create nodes
    doc: Option<N>,
    // Element to use to determine in-scope namespaces
    cur: Option<N>,

    dtd: DTD,
    // Do we add DTD specified attributes or not
    attr_defaults: bool,

    // The in-scope namespace declarations.
    // This will be reset when the parsing context changes
    pub(crate) in_scope_namespaces: NamespaceMap,

    /*
      ID tracking:
      ids_read covers all IDs for duplicate checking. Where an IDREF is found and the ID is not
      yet encountered, we pull into StaticState::ids_pending and will review those when we have finished
      parsing the document.
    */
    id_tracking: bool,

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
    ext_entities_to_parse: Vec<String>,
    docloc: Option<String>,
    /*
    ParamEntities are not allowed in internal subsets, but they are allowed in external DTDs,
    so we need to track when we are currently in the main document or outside it.
     */
    currentlyexternal: bool,
}

impl<N: Node> ParserState<N> {
    pub fn new() -> Self {
        ParserState {
            doc: None,
            cur: None,
            dtd: DTD::new(),
            standalone: false,
            xmlversion: "1.0".to_string(), // Always assume 1.0
            in_scope_namespaces: NamespaceMap::new(),
            id_tracking: true,
            maxentitydepth: 8,
            attr_defaults: true,
            currententitydepth: 1,
            currentcol: 1,
            currentrow: 1,
            //stack: vec![],
            //limit: None,
            ext_entities_to_parse: vec![],
            docloc: None,
            currentlyexternal: false,
        }
    }

    /// Get the result document
    pub fn doc(&self) -> Option<N> {
        self.doc.clone()
    }
    /// Get the current node
    pub fn current(&self) -> Option<N> {
        self.cur.clone()
    }
}

impl<N: Node> PartialEq for ParserState<N> {
    fn eq(&self, _: &ParserState<N>) -> bool {
        true
    }
}

impl<N: Node> fmt::Debug for ParserState<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParserState").finish()
    }
}

pub struct ParserStateBuilder<N: Node>(ParserState<N>);

impl<N: Node> ParserStateBuilder<N> {
    pub fn new() -> Self {
        ParserStateBuilder(ParserState::new())
    }
    pub fn doc(mut self, d: N) -> Self {
        self.0.doc = Some(d);
        self
    }
    pub fn current(mut self, d: N) -> Self {
        self.0.cur = Some(d);
        self
    }
    pub fn dtd(mut self, d: DTD) -> Self {
        self.0.dtd = d;
        self
    }
    pub fn attribute_defaults(mut self, a: bool) -> Self {
        self.0.attr_defaults = a;
        self
    }
    pub fn in_scope_namespaces(mut self, nsm: NamespaceMap) -> Self {
        self.0.in_scope_namespaces = nsm;
        self
    }
    pub fn id_tracking(mut self, a: bool) -> Self {
        self.0.id_tracking = a;
        self
    }
    pub fn standalone(mut self, a: bool) -> Self {
        self.0.standalone = a;
        self
    }
    pub fn currently_external(mut self, a: bool) -> Self {
        self.0.currentlyexternal = a;
        self
    }
    pub fn xml_version(mut self, x: String) -> Self {
        self.0.xmlversion = x;
        self
    }
    pub fn maximum_entity_depth(mut self, d: usize) -> Self {
        self.0.maxentitydepth = d;
        self
    }
    pub fn document_location(mut self, l: String) -> Self {
        self.0.docloc = Some(l);
        self
    }
    pub fn build(self) -> ParserState<N> {
        self.0
    }
}
