/*!
A parser combinator, inspired by nom.

This parser combinator passes a context into the function, which includes the string being parsed. This supports resolving context-based constructs such as general entities and XML Namespaces.
*/

use crate::externals::URLResolver;
use crate::item::Node;
use crate::namespace::NamespaceMap;
use crate::qname::QualifiedName;
use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};
use crate::xmldecl::DTD;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

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
    Unbalanced,
    Notimplemented,
    ExtDTDLoadError,
    IDError(String),
}

pub struct ParserConfig {
    /// If you need to resolve external DTDs, you will need to provide your own resolver.
    pub ext_dtd_resolver: Option<URLResolver>,
    /// The location of the string being parsed, which can be provided to your resolver to work out
    /// relative URLs
    pub docloc: Option<String>,
    /// Recursive entity depth, please note that setting this to a high value may leave
    /// you prone to the "billion laughs" attack. Set to eight by default.
    pub entitydepth: usize,
    /// Creates attributes as specified in ATTLIST declarations in the DTD. Currently only adds
    /// attributes where a default or fixed value is declared, does not enforce anything.
    /// Set to true by default.
    pub attr_defaults: bool,
    /// Track and assign XML IDs based on the DTDs.
    pub id_tracking: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self::new()
    }
}
impl ParserConfig {
    pub fn new() -> Self {
        ParserConfig {
            ext_dtd_resolver: None,
            docloc: None,
            entitydepth: 8,
            attr_defaults: true,
            id_tracking: true,
        }
    }
}

#[derive(Clone)]
pub struct ParserState<N: Node> {
    // Document node to use to create nodes
    doc: Option<N>,
    // Element to use to determine in-scope namespaces
    cur: Option<N>,

    dtd: DTD,
    // Do we add DTD specified attributes or not
    attr_defaults: bool,

    /*
      ID tracking:
      ids_read covers all IDs for duplicate checking. Where an IDREF is found and the ID is not
      yet encountered, we pull into ids_pending and will review those when we have finished
      parsing the document.
    */
    id_tracking: bool,
    ids_read: HashSet<String>,
    ids_pending: HashSet<String>,

    /*
       The in-scope namespaces are tracked in a hashmap.
       This is used during XML document creation.
       The HashMap is Rc-shared. If an element does not declare any new namespaces then it shares its parent's HashMap.
       NOTE: the "None" key in this hashmap is used to track the namespace when no alias is declared, i.e. unprefixed names.
    */
    namespace: Rc<NamespaceMap>, // (prefix, namespace node)
    /*
       Interning of values.
       Strings (represented in xrust as a Value) are often repeated.
       To cut down on data copying, we will intern the string and reuse it.
       NB. in a future version, we will intern values globally so that equality can be tested by comparing pointers.
    */
    interned_values: Rc<RefCell<HashMap<String, Rc<Value>>>>,
    // Intern QualifiedNames. Map (Option<Namespace URI>, local-part) -> QN
    interned_names: Rc<RefCell<HashMap<(Option<Rc<Value>>, Rc<Value>), Rc<QualifiedName>>>>,
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
    ext_dtd_resolver: Option<URLResolver>,
    ext_entities_to_parse: Vec<String>,
    docloc: Option<String>,
    /*
    ParamEntities are not allowed in internal subsets, but they are allowed in external DTDs,
    so we need to track when we are currently in the main document or outside it.
     */
    currentlyexternal: bool,
}

impl<N: Node> ParserState<N> {
    pub fn new(doc: Option<N>, cur: Option<N>, parser_config: Option<ParserConfig>) -> Self {
        let pc = if parser_config.is_some() {
            parser_config.unwrap()
        } else {
            ParserConfig::new()
        };
        let xnsprefix = Rc::new(Value::from("xml"));
        let xnsuri = Rc::new(Value::from("http://www.w3.org/XML/1998/namespace"));
        let mut ns_map = NamespaceMap::new();
        ns_map.insert(Some(xnsprefix.clone()), xnsuri.clone());

        ParserState {
            doc,
            cur,
            dtd: DTD::new(),
            standalone: false,
            xmlversion: "1.0".to_string(), // Always assume 1.0
            namespace: Rc::new(ns_map),
            interned_values: Rc::new(RefCell::new(HashMap::from([
                (String::from("xml"), xnsprefix.clone()),
                (
                    String::from("http://www.w3.org/XML/1998/namespace"),
                    xnsuri.clone(),
                ),
            ]))),
            id_tracking: pc.id_tracking,
            ids_read: Default::default(),
            ids_pending: Default::default(),
            interned_names: Rc::new(RefCell::new(HashMap::new())),
            maxentitydepth: pc.entitydepth,
            attr_defaults: pc.attr_defaults,
            currententitydepth: 1,
            currentcol: 1,
            currentrow: 1,
            //stack: vec![],
            //limit: None,
            ext_dtd_resolver: pc.ext_dtd_resolver,
            ext_entities_to_parse: vec![],
            docloc: pc.docloc,
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
    /// Get a copy of all namespaces
    pub fn namespaces_ref(&self) -> &NamespaceMap {
        &self.namespace
    }
    pub fn resolve(self, locdir: Option<String>, uri: String) -> Result<String, Error> {
        match self.ext_dtd_resolver {
            None => Err(Error::new(
                ErrorKind::Unknown,
                "No external DTD resolver provided.".to_string(),
            )),
            Some(e) => e(locdir, uri),
        }
    }
    pub fn get_value(&self, s: String) -> Rc<Value> {
        {
            if let Some(u) = self.interned_values.borrow().get(&s) {
                return u.clone();
            }
        }
        // Otherwise this is a new entry
        let v = Rc::new(Value::from(s.clone()));
        self.interned_values.borrow_mut().insert(s, v.clone());
        v
    }
    /// Find a QualifiedName. If the name exists in the interned names. then return a reference to the interned name.
    /// Otherwise, add this name to the interned names and return its reference.
    pub fn get_qualified_name(
        &self,
        nsuri: Option<Rc<Value>>,
        prefix: Option<Rc<Value>>,
        local_part: Rc<Value>,
    ) -> Rc<QualifiedName> {
        {
            if let Some(qn) = self
                .interned_names
                .borrow()
                .get(&(nsuri.clone(), local_part.clone()))
            {
                return qn.clone();
            }
        }
        // Otherwise this is a new entry
        let newqn = Rc::new(QualifiedName::new_from_values(
            nsuri.clone(),
            prefix.clone(),
            local_part.clone(),
        ));
        self.interned_names
            .borrow_mut()
            .insert((nsuri, local_part), newqn.clone());
        newqn
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
