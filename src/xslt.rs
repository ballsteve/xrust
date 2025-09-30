/*! ## An XSLT compiler

Compile an XSLT stylesheet into a [Transform]ation.

Once the stylesheet has been compiled, it may then be evaluated with an appropriate context.

NB. This module, by default, does not resolve include or import statements. See the xrust-net crate for a helper module to do that.

```rust
use std::rc::Rc;
use xrust::xdmerror::{Error, ErrorKind};
use xrust::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use xrust::transform::Transform;
use xrust::transform::context::{StaticContext, StaticContextBuilder};
use xrust::trees::smite::RNode;
use xrust::parser::ParseError;
use xrust::parser::xml::parse;
use xrust::xslt::from_document;

// A little helper function to parse an XML document
fn make_from_str(s: &str) -> Result<RNode, Error> {
    let doc = RNode::new_document();
    let e = parse(doc.clone(), s,
        Some(|_: &_| Err(ParseError::MissingNameSpace)))?;
    Ok(doc)
}

// The source document (a tree)
let src = Item::Node(
    make_from_str("<Example><Title>XSLT in Rust</Title><Paragraph>A simple document.</Paragraph></Example>")
    .expect("unable to parse XML")
);

// The XSL stylesheet
let style = make_from_str("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:template match='child::Example'><html><xsl:apply-templates/></html></xsl:template>
  <xsl:template match='child::Title'><head><title><xsl:apply-templates/></title></head></xsl:template>
  <xsl:template match='child::Paragraph'><body><p><xsl:apply-templates/></p></body></xsl:template>
</xsl:stylesheet>")
    .expect("unable to parse stylesheet");

// Create a static context (with dummy callbacks)
let mut static_context = StaticContextBuilder::new()
    .message(|_| Ok(()))
    .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
    .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
    .build();

// Compile the stylesheet
let mut ctxt = from_document(
    style,
    None,
    make_from_str,
    |_| Ok(String::new())
).expect("failed to compile stylesheet");

// Set the source document as the context item
ctxt.context(vec![src], 0);
// Make an empty result document
ctxt.result_document(RNode::new_document());

// Let 'er rip!
// Evaluate the transformation
let seq = ctxt.evaluate(&mut static_context)
    .expect("evaluation failed");

// Serialise the sequence as XML
assert_eq!(seq.to_xml(), "<html><head><title>XSLT in Rust</title></head><body><p>A simple document.</p></body></html>")
 */

use std::collections::HashMap;
use std::rc::Rc;

use crate::item::{Item, Node, NodeType, Sequence};
use crate::output::{OutputDefinition, OutputSpec};
use crate::parser::avt::parse as parse_avt;
use crate::parser::xpath::parse;
use crate::pattern::{Branch, Pattern};
use crate::transform::callable::{ActualParameters, Callable, FormalParameters};
use crate::transform::context::{Context, ContextBuilder};
use crate::transform::numbers::{Level, Numbering};
use crate::transform::template::Template;
use crate::transform::{
    Axis, Grouping, KindTest, NameTest, NodeMatch, NodeTest, Order, Transform, WildcardOrName,
    WildcardOrNamespaceUri, in_scope_namespaces,
};
use crate::value::Value;
use crate::xdmerror::*;
use qualname::{NamespaceUri, NcName, QName};
use std::convert::TryFrom;
use std::sync::LazyLock;
use url::Url;

// Define constant QNames for faster comparison
static XSLTNS: LazyLock<Option<NamespaceUri>> =
    LazyLock::new(|| Some(NamespaceUri::try_from("http://www.w3.org/1999/XSL/Transform").unwrap()));
static XRUSTNS: LazyLock<Option<NamespaceUri>> =
    LazyLock::new(|| Some(NamespaceUri::try_from("http://github.com/ballsteve/xrust").unwrap()));
static XSLSTYLESHEET: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(NcName::try_from("stylesheet").unwrap(), XSLTNS.clone())
});
static XSLTRANSFORM: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("transform").unwrap(), XSLTNS.clone()));
static XSLOUTPUT: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("output").unwrap(), XSLTNS.clone()));
static XSLINCLUDE: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("include").unwrap(), XSLTNS.clone()));
static XRUSTIMPORT: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("import").unwrap(), XRUSTNS.clone()));
static XSLIMPORT: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("import").unwrap(), XSLTNS.clone()));
static XSLATTRIBUTESET: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(NcName::try_from("attribute-set").unwrap(), XSLTNS.clone())
});
static XSLATTRIBUTE: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("attribute").unwrap(), XSLTNS.clone()));
static XSLTEMPLATE: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("template").unwrap(), XSLTNS.clone()));
static XSLKEY: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("key").unwrap(), XSLTNS.clone()));
static XSLPARAM: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("param").unwrap(), XSLTNS.clone()));
static XSLFUNCTION: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("function").unwrap(), XSLTNS.clone()));
static XSLVARIABLE: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("variable").unwrap(), XSLTNS.clone()));
static XSLVALUEOF: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("value-of").unwrap(), XSLTNS.clone()));
static XSLTEXT: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("text").unwrap(), XSLTNS.clone()));
static XSLAPPLYTEMPLATES: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(NcName::try_from("apply-templates").unwrap(), XSLTNS.clone())
});
static XSLAPPLYIMPORTS: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(NcName::try_from("apply-imports").unwrap(), XSLTNS.clone())
});
static XSLSEQUENCE: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("sequence").unwrap(), XSLTNS.clone()));
static XSLIF: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("if").unwrap(), XSLTNS.clone()));
static XSLCHOOSE: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("choose").unwrap(), XSLTNS.clone()));
static XSLWHEN: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("when").unwrap(), XSLTNS.clone()));
static XSLOTHERWISE: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("otherwise").unwrap(), XSLTNS.clone()));
static XSLFOREACH: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("for-each").unwrap(), XSLTNS.clone()));
static XSLFOREACHGROUP: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(NcName::try_from("for-each-group").unwrap(), XSLTNS.clone())
});
static XSLCOPY: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("copy").unwrap(), XSLTNS.clone()));
static XSLCOPYOF: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("copy-of").unwrap(), XSLTNS.clone()));
static XSLCALLTEMPLATE: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(NcName::try_from("call-template").unwrap(), XSLTNS.clone())
});
static XSLWITHPARAM: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(NcName::try_from("with-param").unwrap(), XSLTNS.clone())
});
static XSLELEMENT: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("element").unwrap(), XSLTNS.clone()));
static XSLCOMMENT: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("comment").unwrap(), XSLTNS.clone()));
static XSLPROCESSINGINSTRUCTION: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(
        NcName::try_from("processing-instruction").unwrap(),
        XSLTNS.clone(),
    )
});
static XSLMESSAGE: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("message").unwrap(), XSLTNS.clone()));
static XSLNUMBER: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("number").unwrap(), XSLTNS.clone()));
static XSLDECIMALFORMAT: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(NcName::try_from("decimal-format").unwrap(), XSLTNS.clone())
});
static XSLSORT: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("sort").unwrap(), XSLTNS.clone()));
static XSLSTRIPSPACE: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(NcName::try_from("strip-space").unwrap(), XSLTNS.clone())
});
static XSLPRESERVESPACE: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(NcName::try_from("preserve-space").unwrap(), XSLTNS.clone())
});
static ATTRINDENT: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("indent").unwrap(), None));
static ATTRHREF: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("href").unwrap(), None));
static ATTRNAME: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("name").unwrap(), None));
static ATTRMATCH: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("match").unwrap(), None));
static ATTRMODE: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("mode").unwrap(), None));
static ATTRPRIORITY: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("priority").unwrap(), None));
static ATTRUSE: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("use").unwrap(), None));
static ATTRSELECT: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("select").unwrap(), None));
static ATTRDOE: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(NcName::try_from("disable-output-escaping").unwrap(), None)
});
static ATTRTEST: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("test").unwrap(), None));
static ATTRGROUPBY: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("group-by").unwrap(), None));
static ATTRGROUPADJACENT: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("group-adjacent").unwrap(), None));
static ATTRGROUPSTARTINGWITH: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("group-starting-with").unwrap(), None));
static ATTRGROUPENDINGWITH: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("group-ending-with").unwrap(), None));
//static ATTRUSEATTRIBUTESETS: LazyLock<QName> =
//    LazyLock::new(|| QName::new_from_parts(NcName::try_from("use-attribute-sets").unwrap(), None));
static XSLATTRUSEATTRIBUTESETS: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(
        NcName::try_from("use-attribute-sets").unwrap(),
        XSLTNS.clone(),
    )
});
static ATTRTERMINATE: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("terminate").unwrap(), None));
static ATTRVALUE: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("value").unwrap(), None));
static ATTRLEVEL: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("level").unwrap(), None));
static ATTRCOUNT: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("count").unwrap(), None));
static ATTRFROM: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("from").unwrap(), None));
static ATTRFORMAT: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("format").unwrap(), None));
static ATTRORDER: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("order").unwrap(), None));
static ATTRELEMENTS: LazyLock<QName> =
    LazyLock::new(|| QName::new_from_parts(NcName::try_from("elements").unwrap(), None));

/// The XSLT trait allows an object to use an XSL Stylesheet to transform a document into a [Sequence].
pub trait XSLT: Node {
    /// Interpret the object as an XSL Stylesheet and transform a source document.
    /// The [Node] that is given as the source document becomes the initial context for the transformation.
    fn transform<N: Node, F, G>(
        &self,
        src: Rc<Item<N>>,
        b: Option<Url>,
        f: F,
        g: G,
    ) -> Result<Sequence<N>, Error>
    where
        F: Fn(&str) -> Result<N, Error>,
        G: Fn(&Url) -> Result<String, Error>;
    //    {
    //        let sc = from_document(self.clone(), b, f, g)?;
    //        let ctxt = ContextBuilder::from(&sc)
    //            .current(vec![src])
    //            .build();
    //        ctxt.evaluate()
    //    }
}

/// Compiles a [Node] into a transformation [Context].
/// NB. Due to whitespace stripping, this is destructive of the stylesheet.
/// The argument f is a closure that parses a string to a [Node].
/// The argument g is a closure that resolves a URL to a string.
/// These are used for include and import modules.
/// They are not included in this module since some environments, in particular Wasm, do not have I/O facilities.
pub fn from_document<N: Node, F, G>(
    styledoc: N,
    base: Option<Url>,
    f: F,
    g: G,
) -> Result<Context<N>, Error>
where
    F: Fn(&str) -> Result<N, Error>,
    G: Fn(&Url) -> Result<String, Error>,
{
    // Check that this is a valid XSLT stylesheet
    // There must be a single element as a child of the root node, and it must be named xsl:stylesheet or xsl:transform
    let mut rnit = styledoc.child_iter();
    let stylenode = match rnit.next() {
        Some(root) => {
            // TODO: intern strings so that comparison is fast
            if !(root
                .name()
                .is_some_and(|rn| rn == *XSLSTYLESHEET || rn == *XSLTRANSFORM))
            {
                return Result::Err(Error::new(
                    ErrorKind::TypeError,
                    String::from("not an XSLT stylesheet"),
                ));
            } else {
                root
            }
        }
        None => {
            return Result::Err(Error::new(
                ErrorKind::TypeError,
                String::from("document does not have document element"),
            ));
        }
    };
    if rnit.next().is_some() {
        return Result::Err(Error::new(
            ErrorKind::TypeError,
            String::from("extra element: not an XSLT stylesheet"),
        ));
    }

    // TODO: check version attribute

    // Strip whitespace from the stylesheet
    strip_whitespace(
        styledoc.clone(),
        true,
        &vec![NodeTest::Name(NameTest {
            ns: None,
            //prefix: None,
            name: Some(WildcardOrName::Wildcard),
        })],
        &vec![NodeTest::Name(NameTest {
            ns: Some(WildcardOrNamespaceUri::NamespaceUri(
                XSLTNS.clone().unwrap(),
            )),
            //prefix: Some(Rc::new(Value::from("xsl"))),
            name: Some(WildcardOrName::Name(QName::from_local_name(
                NcName::try_from("text").unwrap(),
            ))),
        })],
    )?;

    // Setup the serialization of the primary result document
    let mut od = OutputDefinition::new();
    if let Some(c) = stylenode
        .child_iter()
        .find(|c| !(c.is_element() && c.name().is_some_and(|cn| cn == *XSLOUTPUT)))
    {
        let b: bool = matches!(
            c.get_attribute(&*ATTRINDENT).to_string().as_str(),
            "yes" | "true" | "1"
        );

        od.set_indent(b);
    };

    // Iterate over children, looking for includes
    // * resolve href
    // * fetch document
    // * parse XML
    // * replace xsl:include element with content
    stylenode
        .child_iter()
        .filter(|c| c.is_element() && c.name().is_some_and(|cn| cn == *XSLINCLUDE))
        .try_for_each(|mut c| {
            let h = c.get_attribute(&*ATTRHREF);
            let url = match base.clone().map_or_else(
                || Url::parse(h.to_string().as_str()),
                |full| full.join(h.to_string().as_str()),
            ) {
                Ok(u) => u,
                Err(_) => {
                    return Result::Err(Error::new(
                        ErrorKind::Unknown,
                        format!(
                            "unable to parse href URL \"{}\" baseurl \"{}\"",
                            h,
                            base.clone()
                                .map_or(String::from("--no base--"), |b| b.to_string())
                        ),
                    ));
                }
            };
            let xml = g(&url)?;
            let module = f(xml.as_str().trim())?;
            // TODO: check that the module is a valid XSLT stylesheet, etc
            // Copy each top-level element of the module to the main stylesheet,
            // inserting before the xsl:include node
            let moddoc = module.first_child().unwrap();
            moddoc.child_iter().try_for_each(|mc| {
                c.insert_before(mc)?;
                Ok::<(), Error>(())
            })?;
            // Remove the xsl:include element node
            c.pop()?;
            Ok(())
        })?;

    // Iterate over children, looking for imports
    // * resolve href
    // * fetch document
    // * parse XML
    // * replace xsl:import element with content
    stylenode
        .child_iter()
        .filter(|c| c.is_element() && c.name().is_some_and(|cn| cn == *XSLIMPORT))
        .try_for_each(|mut c| {
            let h = c.get_attribute(&*ATTRHREF);
            let url = match base.clone().map_or_else(
                || Url::parse(h.to_string().as_str()),
                |full| full.join(h.to_string().as_str()),
            ) {
                Ok(u) => u,
                Err(_) => {
                    return Result::Err(Error::new(
                        ErrorKind::Unknown,
                        format!(
                            "unable to parse href URL \"{}\" baseurl \"{}\"",
                            h,
                            base.clone()
                                .map_or(String::from("--no base--"), |b| b.to_string())
                        ),
                    ));
                }
            };
            let xml = g(&url)?;
            let module = f(xml.as_str().trim())?;
            // TODO: check that the module is a valid XSLT stylesheet, etc
            // Copy each top-level element of the module to the main stylesheet,
            // inserting before the xsl:include node
            // TODO: Don't Panic
            let moddoc = module.first_child().unwrap();
            moddoc.child_iter().try_for_each(|mc| {
                if mc.node_type() == NodeType::Element {
                    // Add the import precedence attribute
                    let newnode = mc.deep_copy()?;
                    let newat =
                        styledoc.new_attribute(XRUSTIMPORT.clone(), Rc::new(Value::from(1)))?;
                    newnode.add_attribute(newat)?;
                    c.insert_before(newnode)?;
                } else {
                    let newnode = mc.deep_copy()?;
                    c.insert_before(newnode)?;
                }
                Ok::<(), Error>(())
            })?;
            // Remove the xsl:import element node
            c.pop()?;
            Ok::<(), Error>(())
        })?;

    // Find named attribute sets

    // Store for named attribute sets
    let mut attr_sets: HashMap<QName, Vec<Transform<N>>> = HashMap::new();

    stylenode
        .child_iter()
        .filter(|c| c.is_element() && c.name().is_some_and(|cn| cn == *XSLATTRIBUTESET))
        .try_for_each(|c| {
            let name = c.get_attribute(&*ATTRNAME);
            let eqname = c.to_qname(name.to_string())?;
            if eqname.to_string().is_empty() {
                return Err(Error::new(
                    ErrorKind::DynamicAbsent,
                    "attribute sets must have a name",
                ));
            }
            // xsl:attribute children
            // TODO: check that there are no other children
            let mut attrs = vec![];
            c.child_iter()
                .filter(|c| c.is_element() && c.name().is_some_and(|cn| cn == *XSLATTRIBUTE))
                .try_for_each(|a| {
                    attrs.push(to_transform(a, &attr_sets)?);
                    Ok(())
                })?;
            attr_sets.insert(eqname, attrs);
            Ok(())
        })?;

    // Iterate over children, looking for templates
    // * compile match pattern
    // * compile content into sequence constructor
    // * register template in dynamic context
    let mut templates: Vec<Template<N>> = vec![];
    stylenode
        .child_iter()
        .filter(|c| c.is_element() && c.name().is_some_and(|cn| cn == *XSLTEMPLATE))
        .filter(|c| c.get_attribute_node(&*ATTRMATCH).is_some())
        .try_for_each(|c| {
            let m = c.get_attribute(&*ATTRMATCH);
            let pat = Pattern::try_from((m.to_string(), c.clone())).map_err(|e| {
                Error::new(
                    e.kind,
                    format!(
                        "Error parsing match pattern \"{}\": {}",
                        m.to_string(),
                        e.message
                    ),
                )
            })?;
            if pat.is_err() {
                return Err(pat.get_err().unwrap());
            }
            if let Pattern::Selection(Branch::Error(e)) = pat {
                return Err(e.clone());
            }
            let mut body = vec![];
            let mode = c.get_attribute_node(&*ATTRMODE);
            c.child_iter().try_for_each(|d| {
                body.push(to_transform(d, &attr_sets)?);
                Ok::<(), Error>(())
            })?;
            //sc.static_analysis(&mut pat);
            //sc.static_analysis(&mut body);
            // Determine the priority of the template
            let pr = c.get_attribute(&*ATTRPRIORITY);
            let prio: f64 = match pr.to_string().as_str() {
                "" => {
                    // Calculate the default priority
                    // TODO: more work to be done interpreting XSLT 6.5
                    match &pat {
                        Pattern::Predicate(p) => match p {
                            Transform::Empty => -1.0,
                            _ => 1.0,
                        },
                        Pattern::Selection(s) => {
                            if let Branch::Error(e) = s {
                                return Err(e.clone());
                            }

                            let (t, nt, q) = s.terminal_node_test();
                            // If "/" then -0.5
                            match (t, nt) {
                                (Axis::SelfAttribute, _) => -0.5,
                                (Axis::SelfAxis, Axis::Parent)
                                | (Axis::SelfAxis, Axis::Ancestor)
                                | (Axis::SelfAxis, Axis::AncestorOrSelf) => match q {
                                    NodeTest::Name(nm) => match nm.name {
                                        Some(WildcardOrName::Wildcard) => -0.5,
                                        Some(_) => 0.0,
                                        _ => -0.5,
                                    },
                                    NodeTest::Kind(_kt) => -0.5,
                                },
                                _ => 0.5,
                            }
                        }
                        _ => -1.0,
                    }
                }
                _ => pr.to_string().parse::<f64>().unwrap(), // TODO: better error handling
            };
            // Set the import precedence
            let mut import: usize = 0;
            let im = c.get_attribute(&*XRUSTIMPORT);
            if im.to_string() != "" {
                import = im.to_int()? as usize
            }
            templates.push(Template::new(
                pat,
                Transform::SequenceItems(body),
                Some(prio),
                vec![import],
                None,
                mode.map(|n| {
                    n.to_qname(n.to_string())
                        .expect("unable to resolve qualified name")
                }), // TODO: don't panic
                m.to_string(),
            ));
            Ok::<(), Error>(())
        })?;

    // Iterate over the children, looking for key declarations.
    // NB. could combine this with the previous loop, but performance shouldn't be an issue.
    let mut keys = vec![];
    stylenode
        .child_iter()
        .filter(|c| c.is_element() && c.name().is_some_and(|cn| cn == *XSLKEY))
        .try_for_each(|c| {
            let name = c.get_attribute(&*ATTRNAME);
            let m = c.get_attribute(&*ATTRMATCH);
            let pat = Pattern::try_from(m.to_string())?;
            let u = c.get_attribute(&*ATTRUSE);
            keys.push((
                name,
                pat,
                parse::<N>(&u.to_string(), Some(c.clone()), None)?,
            ));
            Ok(())
        })?;

    let mut newctxt = ContextBuilder::new()
        // Define the builtin templates
        // See XSLT 6.7. This implements text-only-copy.
        // TODO: Support deep-copy, shallow-copy, deep-skin, shallow-skip and fail
        // This matches "/" and processes the root element
        .template(Template::new(
            Pattern::try_from("/")?,
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch::new(
                    Axis::Child,
                    NodeTest::Kind(KindTest::Any),
                ))),
                None,
                vec![],
            ),
            None,
            vec![0],
            None,
            None,
            String::from("/"),
        ))
        // This matches "*" and applies templates to all children
        .template(Template::new(
            Pattern::try_from("child::*")?,
            Transform::ApplyTemplates(
                Box::new(Transform::Step(NodeMatch::new(
                    Axis::Child,
                    NodeTest::Kind(KindTest::Any),
                ))),
                None,
                vec![],
            ),
            None,
            vec![0],
            None,
            None,
            String::from("child::*"),
        ))
        // This matches "text()" and copies content
        .template(Template::new(
            Pattern::try_from("child::text()")?,
            Transform::ContextItem,
            None,
            vec![0],
            None,
            None,
            String::from("child::text()"),
        ))
        .template_all(templates)
        .output_definition(od)
        .build();

    keys.iter()
        .for_each(|(name, m, u)| newctxt.declare_key(name.to_string(), m.clone(), u.clone()));

    // Add named templates
    stylenode
        .child_iter()
        .filter(|c| c.is_element() && c.name().is_some_and(|cn| cn == *XSLTEMPLATE))
        .filter(|c| !c.get_attribute(&*ATTRNAME).to_string().is_empty())
        .try_for_each(|c| {
            let name = c.get_attribute(&*ATTRNAME);
            // xsl:param for formal parameters
            // TODO: validate that xsl:param elements come first in the child list
            // TODO: validate that xsl:param elements have unique name attributes
            let mut params: Vec<(QName, Option<Transform<N>>)> = Vec::new();
            c.child_iter()
                .filter(|c| c.is_element() && c.name().is_some_and(|cn| cn == *XSLPARAM))
                .try_for_each(|c| {
                    let p_name = c.get_attribute(&*ATTRNAME);
                    if p_name.to_string().is_empty() {
                        Err(Error::new(
                            ErrorKind::StaticAbsent,
                            "name attribute is missing",
                        ))
                    } else {
                        let sel = c.get_attribute(&*ATTRSELECT);
                        if sel.to_string().is_empty() {
                            // xsl:param content is the sequence constructor
                            let mut body = vec![];
                            c.child_iter().try_for_each(|d| {
                                body.push(to_transform(d, &attr_sets)?);
                                Ok(())
                            })?;
                            params.push((
                                QName::from_local_name(
                                    NcName::try_from(p_name.to_string().as_str()).map_err(
                                        |_| Error::new(ErrorKind::ParseError, "not a QName"),
                                    )?,
                                ),
                                Some(Transform::SequenceItems(body)),
                            ));
                            Ok(())
                        } else {
                            // select attribute value is an expression
                            params.push((
                                QName::from_local_name(
                                    NcName::try_from(p_name.to_string().as_str()).map_err(
                                        |_| Error::new(ErrorKind::ParseError, "not a QName"),
                                    )?,
                                ),
                                Some(parse::<N>(&sel.to_string(), Some(c.clone()), None)?),
                            ));
                            Ok(())
                        }
                    }
                })?;
            // Content is the template body
            let mut body = vec![];
            c.child_iter()
                .filter(|c| !(c.is_element() && c.name().is_some_and(|cn| cn == *XSLPARAM)))
                .try_for_each(|d| {
                    body.push(to_transform(d, &attr_sets)?);
                    Ok::<(), Error>(())
                })?;
            newctxt.callable_push(
                QName::from_local_name(
                    NcName::try_from(name.to_string().as_str())
                        .map_err(|_| Error::new(ErrorKind::ParseError, "not a QName"))?,
                ),
                Callable::new(
                    Transform::SequenceItems(body),
                    FormalParameters::Named(params),
                ),
            );
            Ok(())
        })?;

    // Add functions
    stylenode
        .child_iter()
        .filter(|c| c.is_element() && c.name().is_some_and(|cn| cn == *XSLFUNCTION))
        .try_for_each(|c| {
            let name = c.get_attribute(&*ATTRNAME);
            // Name must have a namespace. See XSLT 10.3.1.
            let eqname = c.to_qname(name.to_string())?;
            if eqname.namespace_uri().is_none() {
                return Err(Error::new_with_code(
                    ErrorKind::StaticAbsent,
                    "function name must have a namespace",
                    Some(QName::from_local_name(
                        NcName::try_from("XTSE0740").unwrap(),
                    )),
                ));
            }
            // xsl:param for formal parameters
            // TODO: validate that xsl:param elements come first in the child list
            // TODO: validate that xsl:param elements have unique name attributes
            let mut params: Vec<QName> = Vec::new();
            c.child_iter()
                .filter(|c| c.is_element() && c.name().is_some_and(|cn| cn == *XSLPARAM))
                .try_for_each(|c| {
                    let p_name = c.get_attribute(&*ATTRNAME);
                    if p_name.to_string().is_empty() {
                        Err(Error::new(
                            ErrorKind::StaticAbsent,
                            "name attribute is missing",
                        ))
                    } else {
                        // TODO: validate that xsl:param elements do not specify a default value. See XSLT 10.3.2.
                        params.push(QName::from_local_name(
                            NcName::try_from(p_name.to_string().as_str()).map_err(|_| {
                                Error::new(ErrorKind::ParseError, "not a valid QName")
                            })?,
                        ));
                        Ok(())
                    }
                })?;
            // Content is the function body
            let mut body = vec![];
            c.child_iter()
                .filter(|c| !(c.is_element() && c.name().is_some_and(|cn| cn == *XSLPARAM)))
                .try_for_each(|d| {
                    body.push(to_transform(d, &attr_sets)?);
                    Ok::<(), Error>(())
                })?;
            newctxt.callable_push(
                eqname,
                Callable::new(
                    Transform::SequenceItems(body),
                    FormalParameters::Positional(params),
                ),
            );
            Ok(())
        })?;
    // Add top-level variables
    // TODO: stylesheet parameters
    stylenode
        .child_iter()
        .filter(|c| c.is_element() && c.name().is_some_and(|cn| cn == *XSLVARIABLE))
        .try_for_each(|c| {
            let name = c.get_attribute(&*ATTRNAME).to_string();
            if name.is_empty() {
                return Err(Error::new(
                    ErrorKind::StaticAbsent,
                    "variable must have a name",
                ));
            }
            let sel = c.get_attribute(&*ATTRSELECT).to_string();
            if sel.is_empty() {
                // Use element content
                newctxt.pre_var_push(
                    name,
                    Transform::SequenceItems(c.child_iter().try_fold(vec![], |mut body, e| {
                        body.push(to_transform(e, &attr_sets)?);
                        Ok(body)
                    })?),
                );
                Ok(())
            } else {
                // Parse XPath
                newctxt.pre_var_push(name, parse::<N>(&sel.to_string(), Some(c.clone()), None)?);
                Ok(())
            }
        })?;

    Ok(newctxt)
}

/// Compile a node in a template to a sequence [Combinator]
fn to_transform<N: Node>(
    n: N,
    attr_sets: &HashMap<QName, Vec<Transform<N>>>,
) -> Result<Transform<N>, Error> {
    // Define the in-scope namespaces once so they can be shared
    //let ns = in_scope_namespaces(Some(n.clone()));

    match n.node_type() {
        NodeType::Text => Ok(Transform::Literal(Item::Value(Rc::new(Value::from(
            n.to_string(),
        ))))),
        NodeType::Element => {
            let qn = n.name().unwrap();
            if qn == *XSLTEXT {
                let doe = n.get_attribute(&*ATTRDOE);
                if !doe.to_string().is_empty() {
                    match &doe.to_string()[..] {
                        "yes" => Ok(Transform::Literal(Item::Value(Rc::new(Value::from(
                            n.to_string(),
                        ))))),
                        "no" => {
                            let text = n
                                .to_string()
                                .replace('&', "&amp;")
                                .replace('>', "&gt;")
                                .replace('<', "&lt;")
                                .replace('\'', "&apos;")
                                .replace('\"', "&quot;");
                            Ok(Transform::Literal(Item::Value(Rc::new(Value::from(text)))))
                        }
                        _ => Err(Error::new(
                            ErrorKind::TypeError,
                            "disable-output-escaping only accepts values yes or no.".to_string(),
                        )),
                    }
                } else {
                    let text = n
                        .to_string()
                        .replace('&', "&amp;")
                        .replace('>', "&gt;")
                        .replace('<', "&lt;")
                        .replace('\'', "&apos;")
                        .replace('\"', "&quot;");
                    Ok(Transform::Literal(Item::Value(Rc::new(Value::from(text)))))
                }
            } else if qn == *XSLVALUEOF {
                let sel = n.get_attribute(&*ATTRSELECT);
                let doe = n.get_attribute(&*ATTRDOE);
                if !doe.to_string().is_empty() {
                    match &doe.to_string()[..] {
                        "yes" => Ok(Transform::LiteralText(
                            Box::new(parse::<N>(&sel.to_string(), Some(n.clone()), None)?),
                            OutputSpec::NoEscape,
                        )),
                        "no" => Ok(Transform::LiteralText(
                            Box::new(parse::<N>(&sel.to_string(), Some(n.clone()), None)?),
                            OutputSpec::Normal,
                        )),
                        _ => Err(Error::new(
                            ErrorKind::TypeError,
                            "disable-output-escaping only accepts values yes or no.".to_string(),
                        )),
                    }
                } else {
                    Ok(Transform::LiteralText(
                        Box::new(parse::<N>(&sel.to_string(), Some(n.clone()), None)?),
                        OutputSpec::Normal,
                    ))
                }
            } else if qn == *XSLAPPLYTEMPLATES {
                let sel = n.get_attribute(&*ATTRSELECT);
                let m = n.get_attribute_node(&*ATTRMODE);
                let sort_keys = get_sort_keys(&n)?;
                if !sel.to_string().is_empty() {
                    Ok(Transform::ApplyTemplates(
                        Box::new(parse::<N>(&sel.to_string(), Some(n.clone()), None)?),
                        m.map(|s| {
                            n.to_qname(s.value().to_string())
                                .expect("unable to resolve qualified name")
                        }),
                        sort_keys,
                    )) // TODO: don't panic
                } else {
                    // If there is no select attribute, then default is "child::node()"
                    Ok(Transform::ApplyTemplates(
                        Box::new(Transform::Step(NodeMatch::new(
                            Axis::Child,
                            NodeTest::Kind(KindTest::Any),
                        ))),
                        m.map(|s| {
                            n.to_qname(s.value().to_string())
                                .expect("unable to resolve qualified name")
                        }),
                        sort_keys,
                    )) // TODO: don't panic
                }
            } else if qn == *XSLAPPLYIMPORTS {
                Ok(Transform::ApplyImports)
            } else if qn == *XSLSEQUENCE {
                let s = n.get_attribute(&*ATTRSELECT);
                if !s.to_string().is_empty() {
                    Ok(parse::<N>(&s.to_string(), Some(n.clone()), None)?)
                } else {
                    Result::Err(Error::new(
                        ErrorKind::TypeError,
                        "missing select attribute".to_string(),
                    ))
                }
            } else if qn == *XSLIF {
                let t = n.get_attribute(&*ATTRTEST);
                if !t.to_string().is_empty() {
                    Ok(Transform::Switch(
                        vec![(
                            parse::<N>(&t.to_string(), Some(n.clone()), None)?,
                            Transform::SequenceItems(n.child_iter().try_fold(
                                vec![],
                                |mut body, e| {
                                    body.push(to_transform(e, attr_sets)?);
                                    Ok(body)
                                },
                            )?),
                        )],
                        Box::new(Transform::Empty),
                    ))
                } else {
                    Result::Err(Error::new(
                        ErrorKind::TypeError,
                        "missing test attribute".to_string(),
                    ))
                }
            } else if qn == *XSLCHOOSE {
                let mut clauses: Vec<(Transform<N>, Transform<N>)> = Vec::new();
                let mut otherwise: Option<Transform<N>> = None;
                let mut status: Option<Error> = None;
                n.child_iter().try_for_each(|m| {
                    // look for when elements
                    // then find an otherwise
                    // fail on anything else (apart from whitespace, comments, PIs)
                    match m.node_type() {
                        NodeType::Element => {
                            let mn = m.name().unwrap();
                            if mn == *XSLWHEN {
                                if otherwise.is_none() {
                                    let t = m.get_attribute(&*ATTRTEST);
                                    if !t.to_string().is_empty() {
                                        clauses.push((
                                            parse::<N>(&t.to_string(), Some(n.clone()), None)?,
                                            Transform::SequenceItems(m.child_iter().try_fold(
                                                vec![],
                                                |mut body, e| {
                                                    body.push(to_transform(e, attr_sets)?);
                                                    Ok(body)
                                                },
                                            )?),
                                        ));
                                    } else {
                                        status.replace(Error::new(
                                            ErrorKind::TypeError,
                                            "missing test attribute".to_string(),
                                        ));
                                    }
                                } else {
                                    status.replace(Error::new(
                                        ErrorKind::TypeError,
                                        "invalid content in choose element: when follows otherwise"
                                            .to_string(),
                                    ));
                                }
                            } else if mn == *XSLOTHERWISE {
                                if !clauses.is_empty() {
                                    otherwise = Some(Transform::SequenceItems(
                                        m.child_iter().try_fold(vec![], |mut o, e| {
                                            o.push(to_transform(e, attr_sets)?);
                                            Ok(o)
                                        })?,
                                    ));
                                } else {
                                    status.replace(Error::new(
                                        ErrorKind::TypeError,
                                        "invalid content in choose element: no when elements"
                                            .to_string(),
                                    ));
                                }
                            } else {
                                status.replace(Error::new(
                                    ErrorKind::TypeError,
                                    "invalid element content in choose element".to_string(),
                                ));
                            }
                        }
                        NodeType::Text => {
                            if !n.to_string().trim().is_empty() {
                                status.replace(Error::new(
                                    ErrorKind::TypeError,
                                    "invalid text content in choose element".to_string(),
                                ));
                            }
                        }
                        NodeType::Comment | NodeType::ProcessingInstruction => {}
                        _ => {
                            status.replace(Error::new(
                                ErrorKind::TypeError,
                                "invalid content in choose element".to_string(),
                            ));
                        }
                    }
                    Ok::<(), Error>(())
                })?;
                match status {
                    Some(e) => Result::Err(e),
                    None => Ok(Transform::Switch(
                        clauses,
                        otherwise.map_or(Box::new(Transform::Empty), Box::new),
                    )),
                }
            } else if qn == *XSLFOREACH {
                let s = n.get_attribute(&*ATTRSELECT);
                if !s.to_string().is_empty() {
                    Ok(Transform::ForEach(
                        None,
                        Box::new(parse::<N>(&s.to_string(), Some(n.clone()), None)?),
                        Box::new(Transform::SequenceItems(n.child_iter().try_fold(
                            vec![],
                            |mut body, e| {
                                body.push(to_transform(e, attr_sets)?);
                                Ok(body)
                            },
                        )?)),
                        get_sort_keys(&n)?,
                    ))
                } else {
                    Result::Err(Error::new(
                        ErrorKind::TypeError,
                        "missing select attribute".to_string(),
                    ))
                }
            } else if qn == *XSLFOREACHGROUP {
                let ord = get_sort_keys(&n)?;
                let s = n.get_attribute(&*ATTRSELECT);
                if !s.to_string().is_empty() {
                    match (
                        n.get_attribute(&*ATTRGROUPBY).to_string().as_str(),
                        n.get_attribute(&*ATTRGROUPADJACENT).to_string().as_str(),
                        n.get_attribute(&*ATTRGROUPSTARTINGWITH)
                            .to_string()
                            .as_str(),
                        n.get_attribute(&*ATTRGROUPENDINGWITH).to_string().as_str(),
                    ) {
                        (by, "", "", "") => Ok(Transform::ForEach(
                            Some(Grouping::By(vec![parse::<N>(by, Some(n.clone()), None)?])),
                            Box::new(parse::<N>(&s.to_string(), Some(n.clone()), None)?),
                            Box::new(Transform::SequenceItems(n.child_iter().try_fold(
                                vec![],
                                |mut body, e| {
                                    body.push(to_transform(e, attr_sets)?);
                                    Ok(body)
                                },
                            )?)),
                            ord,
                        )),
                        ("", adj, "", "") => Ok(Transform::ForEach(
                            Some(Grouping::Adjacent(vec![parse::<N>(
                                adj,
                                Some(n.clone()),
                                None,
                            )?])),
                            Box::new(parse::<N>(&s.to_string(), Some(n.clone()), None)?),
                            Box::new(Transform::SequenceItems(n.child_iter().try_fold(
                                vec![],
                                |mut body, e| {
                                    body.push(to_transform(e, attr_sets)?);
                                    Ok(body)
                                },
                            )?)),
                            ord,
                        )),
                        ("", "", start, "") => Ok(Transform::ForEach(
                            Some(Grouping::StartingWith(Box::new(Pattern::try_from(start)?))),
                            Box::new(parse::<N>(&s.to_string(), Some(n.clone()), None)?),
                            Box::new(Transform::SequenceItems(n.child_iter().try_fold(
                                vec![],
                                |mut body, e| {
                                    body.push(to_transform(e, attr_sets)?);
                                    Ok(body)
                                },
                            )?)),
                            ord,
                        )),
                        // TODO: group-ending-with
                        _ => Result::Err(Error::new(
                            ErrorKind::NotImplemented,
                            "invalid grouping attribute(s) specified".to_string(),
                        )),
                    }
                } else {
                    Result::Err(Error::new(
                        ErrorKind::TypeError,
                        "missing select attribute".to_string(),
                    ))
                }
            } else if qn == *XSLCOPY {
                // TODO: handle select attribute
                let mut content: Vec<Transform<N>> =
                    n.child_iter().try_fold(vec![], |mut body, e| {
                        body.push(to_transform(e, attr_sets)?);
                        Ok(body)
                    })?;
                // Process @xsl:use-attribute-sets
                let use_atts = n.get_attribute(&*XSLATTRUSEATTRIBUTESETS);
                let mut attrs = vec![];
                use_atts.to_string().split_whitespace().try_for_each(|a| {
                    let eqa = n.to_qname(a)?;
                    attr_sets
                        .get(&eqa)
                        .iter()
                        .cloned()
                        .for_each(|a| attrs.append(&mut a.clone()));
                    Ok(())
                })?;
                Ok(Transform::Copy(
                    Box::new(Transform::ContextItem), // TODO: this is where the select attribute would go
                    // The content of this element is a template for the content of the new item
                    Box::new(if content.is_empty() && attrs.is_empty() {
                        Transform::Empty
                    } else {
                        // Attributes always come first
                        attrs.append(&mut content);
                        Transform::SequenceItems(attrs)
                    }),
                ))
            } else if qn == *XSLCOPYOF {
                let s = n.get_attribute(&*ATTRSELECT);
                if !s.to_string().is_empty() {
                    Ok(Transform::DeepCopy(Box::new(parse::<N>(
                        &s.to_string(),
                        Some(n.clone()),
                        None,
                    )?)))
                } else {
                    Ok(Transform::DeepCopy(Box::new(Transform::ContextItem)))
                }
            } else if qn == *XSLCALLTEMPLATE {
                let name = n.get_attribute(&*ATTRNAME);
                if !name.to_string().is_empty() {
                    // Iterate over the xsl:with-param elements to get the actual parameters
                    // TODO: validate that the children are only xsl:with-param elements
                    let mut ap = vec![];
                    n.child_iter()
                        .filter(|c| c.is_element() && c.name().unwrap() == *XSLWITHPARAM)
                        .try_for_each(|c| {
                            let wp_name = c.get_attribute(&*ATTRNAME);
                            if !wp_name.to_string().is_empty() {
                                let sel = c.get_attribute(&*ATTRSELECT);
                                if sel.to_string().is_empty() {
                                    // xsl:with-param content is the sequence constructor
                                    let mut body = vec![];
                                    c.child_iter().try_for_each(|d| {
                                        body.push(to_transform(d, attr_sets)?);
                                        Ok(())
                                    })?;
                                    ap.push((
                                        QName::from_local_name(
                                            NcName::try_from(wp_name.to_string().as_str())
                                                .map_err(|_| {
                                                    Error::new(ErrorKind::ParseError, "not a QName")
                                                })?,
                                        ),
                                        Transform::SequenceItems(body),
                                    ));
                                    Ok(())
                                } else {
                                    // select attribute value is an expression
                                    ap.push((
                                        QName::from_local_name(
                                            NcName::try_from(wp_name.to_string().as_str())
                                                .map_err(|_| {
                                                    Error::new(ErrorKind::ParseError, "not a QName")
                                                })?,
                                        ),
                                        parse::<N>(&sel.to_string(), Some(n.clone()), None)?,
                                    ));
                                    Ok(())
                                }
                            } else {
                                Err(Error::new(
                                    ErrorKind::StaticAbsent,
                                    "missing name attribute",
                                ))
                            }
                        })?;
                    Ok(Transform::Invoke(
                        QName::from_local_name(
                            NcName::try_from(name.to_string().as_str())
                                .map_err(|_| Error::new(ErrorKind::ParseError, "not a NcName"))?,
                        ),
                        ActualParameters::Named(ap),
                        in_scope_namespaces(Some(n)),
                    ))
                } else {
                    Err(Error::new(
                        ErrorKind::StaticAbsent,
                        "name attribute missing",
                    ))
                }
            } else if qn == *XSLELEMENT {
                // TODO: insert namespace declaration if element's name is prefixed
                let m = n.get_attribute(&*ATTRNAME);
                if m.to_string().is_empty() {
                    return Err(Error::new(ErrorKind::TypeError, "missing name attribute"));
                }
                let mut content = n.child_iter().try_fold(vec![], |mut body, e| {
                    body.push(to_transform(e, attr_sets)?);
                    Ok(body)
                })?;
                // Process @xsl:use-attribute-sets
                let use_atts = n.get_attribute(&*XSLATTRUSEATTRIBUTESETS);
                let mut attrs = vec![];
                use_atts.to_string().split_whitespace().try_for_each(|a| {
                    let eqa = n.to_qname(a)?;
                    attr_sets
                        .get(&eqa)
                        .iter()
                        .cloned()
                        .for_each(|a| attrs.append(&mut a.clone()));
                    Ok(())
                })?;

                Ok(Transform::Element(
                    Box::new(parse_avt(m.to_string().as_str(), Some(n.clone()))?),
                    Box::new(if content.is_empty() && attrs.is_empty() {
                        Transform::Empty
                    } else {
                        // Attributes always come first
                        attrs.append(&mut content);
                        Transform::SequenceItems(attrs)
                    }),
                ))
            } else if qn == *XSLATTRIBUTE {
                let m = n.get_attribute(&*ATTRNAME);
                if !m.to_string().is_empty() {
                    Ok(Transform::LiteralAttribute(
                        QName::from_local_name(
                            NcName::try_from(m.to_string().as_str())
                                .map_err(|_| Error::new(ErrorKind::ParseError, "not a NcName"))?,
                        ),
                        Box::new(Transform::SequenceItems(n.child_iter().try_fold(
                            vec![],
                            |mut body, e| {
                                body.push(to_transform(e, attr_sets)?);
                                Ok(body)
                            },
                        )?)),
                    ))
                } else {
                    Err(Error::new(ErrorKind::TypeError, "missing name attribute"))
                }
            } else if qn == *XSLCOMMENT {
                Ok(Transform::LiteralComment(Box::new(
                    Transform::SequenceItems(n.child_iter().try_fold(vec![], |mut body, e| {
                        body.push(to_transform(e, attr_sets)?);
                        Ok(body)
                    })?),
                )))
            } else if qn == *XSLPROCESSINGINSTRUCTION {
                let m = n.get_attribute(&*ATTRNAME);
                if m.to_string().is_empty() {
                    return Result::Err(Error::new(ErrorKind::TypeError, "missing name attribute"));
                }
                Ok(Transform::LiteralProcessingInstruction(
                    Box::new(parse_avt(m.to_string().as_str(), Some(n.clone()))?),
                    Box::new(Transform::SequenceItems(n.child_iter().try_fold(
                        vec![],
                        |mut body, e| {
                            body.push(to_transform(e, attr_sets)?);
                            Ok(body)
                        },
                    )?)),
                ))
            } else if qn == *XSLMESSAGE {
                let t = n.get_attribute(&*ATTRTERMINATE);
                Ok(Transform::Message(
                    Box::new(Transform::SequenceItems(n.child_iter().try_fold(
                        vec![],
                        |mut body, e| {
                            body.push(to_transform(e, attr_sets)?);
                            Ok(body)
                        },
                    )?)),
                    None,
                    Box::new(Transform::Empty),
                    Box::new(if t.to_string().is_empty() {
                        Transform::False
                    } else {
                        Transform::Literal(Item::Value(Rc::new(Value::from(t.to_string()))))
                    }),
                ))
            } else if qn == *XSLNUMBER {
                let value = n.get_attribute(&*ATTRVALUE);
                let sel = n.get_attribute(&*ATTRSELECT);
                let level = n.get_attribute(&*ATTRLEVEL);
                if level.to_string() != "" && level.to_string() != "single" {
                    return Err(Error::new(
                        ErrorKind::NotImplemented,
                        "only single level numbering is supported",
                    ));
                }
                let count = n.get_attribute(&*ATTRCOUNT);
                let from = n.get_attribute(&*ATTRFROM);
                let format = n.get_attribute(&*ATTRFORMAT);
                // TODO: lang, letter-value, ordinal, start-at, grouping-separator, grouping-size
                if value.to_string().is_empty() {
                    // Compute place marker
                    Ok(Transform::FormatInteger(
                        Box::new(Transform::GenerateIntegers(
                            Box::new(Transform::Empty), // start-at (TODO)
                            Box::new(if sel.to_string().is_empty() {
                                Transform::ContextItem
                            } else {
                                parse::<N>(&sel.to_string(), Some(n.clone()), None)?
                            }), // select
                            Box::new(Numbering::new(
                                Level::Single, // TODO: parse level attribute value
                                if count.to_string().is_empty() {
                                    None
                                } else {
                                    Some(Pattern::try_from(count.to_string())?)
                                },
                                if from.to_string().is_empty() {
                                    None
                                } else {
                                    Some(Pattern::try_from(from.to_string())?)
                                },
                            )),
                        )),
                        Box::new(Transform::Literal(Item::Value(
                            if format.to_string().is_empty() {
                                Rc::new(Value::from("1"))
                            } else {
                                format
                            },
                        ))),
                    ))
                } else {
                    // Place marker is supplied
                    Ok(Transform::FormatInteger(
                        Box::new(parse::<N>(&value.to_string(), Some(n.clone()), None)?),
                        Box::new(Transform::Literal(Item::Value(
                            if format.to_string().is_empty() {
                                Rc::new(Value::from("1"))
                            } else {
                                format
                            },
                        ))),
                    ))
                }
            } else if qn == *XSLDECIMALFORMAT {
                Ok(Transform::NotImplemented(String::from(
                    "unsupported XSL element \"decimal-format\"",
                )))
            } else if qn.namespace_uri() == *XSLTNS {
                Ok(Transform::NotImplemented(format!(
                    "unsupported XSL element \"{}\"",
                    qn.local_name()
                )))
            } else {
                let u = qn.namespace_uri();
                let a = qn.local_name();

                // Uh-oh! Parsing the stylesheet has thrown away all qualified name prefixes
                // But there will be a namespace declaration, so recover it from there
                let prefix: Option<Box<Transform<N>>> = u.as_ref().map_or(None, |nsuri| {
                    n.namespace_iter()
                        .find(|nsd| nsd.as_namespace_uri().unwrap() == nsuri)
                        .unwrap()
                        .as_namespace_prefix()
                        .unwrap()
                        .map(|p| {
                            Box::new(Transform::Literal(Item::Value(Rc::new(Value::from(
                                p.to_string(),
                            )))))
                        })
                });

                // Process @xsl:use-attribute-sets
                let use_atts = n.get_attribute(&*XSLATTRUSEATTRIBUTESETS);
                let mut attrs = vec![];
                use_atts.to_string().split_whitespace().try_for_each(|a| {
                    let eqa = n.to_qname(a)?; //QName::try_from((a, ns.clone()))?;
                    attr_sets
                        .get(&eqa)
                        .iter()
                        .cloned()
                        .for_each(|a| attrs.append(&mut a.clone()));
                    Ok(())
                })?;
                let mut content = vec![];

                // Setup a namespace declaration if required for the element name
                if u.is_some() {
                    content.push(Transform::NamespaceDeclaration(
                        prefix,
                        Box::new(Transform::Literal(Item::Value(Rc::new(Value::from(
                            u.clone().unwrap(),
                        ))))),
                    ));
                }

                // Copy attributes to the result, except for XSLT directives
                n.attribute_iter()
                    .filter(|e| e.name().unwrap().namespace_uri() != *XSLTNS)
                    .try_for_each(|e| {
                        content.push(to_transform(e, attr_sets)?);
                        Ok::<(), Error>(())
                    })?;
                n.child_iter().try_for_each(|e| {
                    content.push(to_transform(e, attr_sets)?);
                    Ok::<(), Error>(())
                })?;
                Ok(Transform::LiteralElement(
                    QName::new_from_parts(
                        NcName::try_from(a.as_str())
                            .map_err(|_| Error::new(ErrorKind::ParseError, "not a NcName"))?,
                        u,
                    ),
                    Box::new(if content.is_empty() && attrs.is_empty() {
                        Transform::Empty
                    } else {
                        // Attributes always come first
                        attrs.append(&mut content);
                        Transform::SequenceItems(attrs)
                    }),
                ))
            }
        }
        NodeType::Attribute => {
            let x = parse_avt(n.to_string().as_str(), Some(n.clone()))?;
            // Get value as a Value
            Ok(Transform::LiteralAttribute(
                n.name().unwrap(),
                Box::new(x),
                //Box::new(Transform::Literal(Item::Value(Rc::new(Value::String(
                //n.to_string(),
                //))))),
            ))
        }
        _ => {
            // TODO: literal elements, etc, pretty much everything in the XSLT spec
            Ok(Transform::NotImplemented(
                "other template content".to_string(),
            ))
        }
    }
}

fn get_sort_keys<N: Node>(n: &N) -> Result<Vec<(Order, Transform<N>)>, Error> {
    let mut result = vec![];
    let mut nit = n.child_iter();
    loop {
        match nit.next() {
            None => break,
            Some(c) => match c.node_type() {
                NodeType::Element => {
                    if c.name().is_some_and(|d| d == *XSLSORT) {
                        let ordval = c.get_attribute(&*ATTRORDER);
                        let ord = match ordval.to_string().as_str() {
                            "descending" => Order::Descending,
                            _ => Order::Ascending,
                        };
                        let sortsel = c.get_attribute(&*ATTRSELECT);
                        result.push((
                            ord,
                            parse::<N>(&sortsel.to_string(), Some(n.clone()), None)?,
                        ));
                    } else {
                        break;
                    }
                }
                NodeType::Text => {
                    if c.value()
                        .to_string()
                        .as_str()
                        .find(|d: char| !d.is_whitespace())
                        .is_some()
                    {
                        break;
                    }
                }
                NodeType::Comment | NodeType::ProcessingInstruction => {}
                _ => break,
            },
        }
    }
    // Check that there are no more sort elements
    if nit.any(|c| {
        if c.node_type() == NodeType::Element && c.name().is_some_and(|d| d == *XSLSORT) {
            true
        } else {
            false
        }
    }) {
        Err(Error::new(ErrorKind::TypeError, "sort elements in body"))
    } else {
        Ok(result)
    }
}

/// Strip whitespace nodes from a XDM tree.
/// See [XSLT 4.3](https://www.w3.org/TR/2017/REC-xslt-30-20170608/#stylesheet-stripping).
/// The [Node] argument must be the document node of the tree.
pub fn strip_whitespace<N: Node>(
    t: N,
    cpi: bool, // strip comments and PIs?
    strip: &Vec<NodeTest>,
    preserve: &Vec<NodeTest>,
) -> Result<(), Error> {
    t.child_iter().try_for_each(|n| {
        strip_whitespace_node(n, cpi, strip, preserve, true)?;
        Ok(())
    })?;
    Ok(())
}

/// Strip whitespace nodes from a XDM tree.
/// This function operates under the direction of the xsl:strip-space and xsl:preserve-space directives in a XSLT stylesheet.
pub fn strip_source_document<N: Node>(src: N, style: N) -> Result<(), Error> {
    // Find strip-space element, if any, and use it to construct a vector of NodeTests.
    // Ditto for preserve-space.
    let mut ss: Vec<NodeTest> = vec![];
    let mut ps: Vec<NodeTest> = vec![];
    style.child_iter().try_for_each(|n| {
        // n should be the xsl:stylesheet element
        n.child_iter().try_for_each(|m| {
            let nm = m.name();
            if nm.as_ref().is_some_and(|nms| *nms == *XSLSTRIPSPACE) {
                let v = m.get_attribute(&*ATTRELEMENTS);
                if !v.to_string().is_empty() {
                    v.to_string().split_whitespace().try_for_each(|t| {
                        ss.push(NodeTest::try_from(t)?);
                        Ok::<(), Error>(())
                    })?
                } else {
                    return Result::Err(Error::new(
                        ErrorKind::Unknown,
                        String::from("missing elements attribute"),
                    ));
                }
            } else if nm.as_ref().is_some_and(|nms| *nms == *XSLPRESERVESPACE) {
                let v = m.get_attribute(&*ATTRELEMENTS);
                if !v.to_string().is_empty() {
                    v.to_string().split_whitespace().try_for_each(|t| {
                        ps.push(NodeTest::try_from(t)?);
                        Ok::<(), Error>(())
                    })?
                } else {
                    return Result::Err(Error::new(
                        ErrorKind::Unknown,
                        String::from("missing elements attribute"),
                    ));
                }
            }
            Ok::<(), Error>(())
        })?;
        Ok::<(), Error>(())
    })?;

    strip_whitespace(src, false, &ss, &ps)
}

// TODO: the rules for stripping/preserving are a lot more complex
fn strip_whitespace_node<N: Node>(
    mut n: N,
    cpi: bool, // strip comments and PIs?
    strip: &Vec<NodeTest>,
    preserve: &Vec<NodeTest>,
    keep: bool,
) -> Result<(), Error> {
    match n.node_type() {
        NodeType::Comment | NodeType::ProcessingInstruction => {
            if cpi {
                n.pop()?;
                // TODO: Merge text nodes that are now adjacent
            }
        }
        NodeType::Element => {
            // Determine if this element toggles the strip/preserve setting
            // Match a strip NodeTest or a preserve NodeTest
            // The 'strength' of the match determines which setting wins
            let mut ss = -1.0;
            let mut ps = -1.0;
            strip.iter().for_each(|t| match t {
                NodeTest::Kind(KindTest::Any) | NodeTest::Kind(KindTest::Element) => ss = -0.5,
                NodeTest::Name(nt) => match (nt.ns.as_ref(), nt.name.as_ref()) {
                    (None, Some(WildcardOrName::Wildcard)) => {
                        ss = -0.25;
                    }
                    (None, Some(WildcardOrName::Name(name))) => {
                        if *name == n.name().unwrap() {
                            ss = 0.5
                        }
                        /*match (
                            n.name().unwrap().namespace_uri(),
                            n.name().unwrap().local_name(),
                        ) {
                            (Some(_), _) => {}
                            (None, ename) => {
                                if *name == ename {
                                    ss = 0.5;
                                }
                            }
                        }*/
                    }
                    (
                        Some(WildcardOrNamespaceUri::NamespaceUri(ns)),
                        Some(WildcardOrName::Name(name)),
                    ) => {
                        if n.name().unwrap().namespace_uri().is_some_and(|f| f == *ns) {
                            if n.name().unwrap() == *name {
                                ss = 0.5;
                            }
                        } else if n.name().unwrap() == *name {
                            ss = 0.5;
                        }
                        /*match (
                            n.name().map_or("", |f| f.namespace_uri()),
                            n.name().map_or("", |g| g.local_name()),
                        ) {
                            (Some(ens), ename) => {
                                if *ns == ens && *name == ename {
                                    ss = 0.5;
                                }
                            }
                            (None, ename) => {
                                if *name == ename {
                                    ss = 0.5;
                                }
                            }
                        }*/
                    }
                    (Some(WildcardOrNamespaceUri::Wildcard), Some(WildcardOrName::Name(_))) => {
                        ss = -0.25;
                    }
                    (
                        Some(WildcardOrNamespaceUri::NamespaceUri(_)),
                        Some(WildcardOrName::Wildcard),
                    ) => {
                        ss = -0.25;
                    }
                    (Some(WildcardOrNamespaceUri::Wildcard), Some(WildcardOrName::Wildcard)) => {
                        ss = -0.5;
                    }
                    _ => {}
                },
                _ => {}
            });
            preserve.iter().for_each(|t| match t {
                NodeTest::Kind(KindTest::Any) | NodeTest::Kind(KindTest::Element) => ps = -0.5,
                NodeTest::Name(nt) => match (nt.ns.as_ref(), nt.name.as_ref()) {
                    (None, Some(WildcardOrName::Name(name))) => {
                        if *name == n.name().unwrap() {
                            ps = 0.5
                        }
                        /*match (
                            n.name().map_or("", |f| f.namespace_uri()),
                            n.name().map_or("", |g| g.local_name()),
                        ) {
                            (Some(_), _) => {}
                            (None, ename) => {
                                if *name == ename {
                                    ps = 0.5;
                                }
                            }
                        }*/
                    }
                    (
                        Some(WildcardOrNamespaceUri::NamespaceUri(ns)),
                        Some(WildcardOrName::Name(name)),
                    ) => {
                        if n.name().unwrap().namespace_uri().is_some_and(|f| f == *ns) {
                            if n.name().unwrap() == *name {
                                ps = 0.5;
                            }
                        } else if n.name().unwrap() == *name {
                            ps = 0.5;
                        }
                        /*match (
                            n.name().map_or("", |f| f.namespace_uri()),
                            n.name().map_or("", |g| g.local_name()),
                        ) {
                            (Some(ens), ename) => {
                                if *ns == ens && *name == ename {
                                    ps = 0.5;
                                }
                            }
                            (None, ename) => {
                                if *name == ename {
                                    ps = 0.5;
                                }
                            }
                        }*/
                    }
                    (Some(WildcardOrNamespaceUri::Wildcard), Some(WildcardOrName::Name(_))) => {
                        ps = -0.25;
                    }
                    (
                        Some(WildcardOrNamespaceUri::NamespaceUri(_)),
                        Some(WildcardOrName::Wildcard),
                    ) => {
                        ps = -0.25;
                    }
                    (Some(WildcardOrNamespaceUri::Wildcard), Some(WildcardOrName::Wildcard)) => {
                        ps = -0.5;
                    }
                    _ => {}
                },
                _ => {}
            });
            n.child_iter().try_for_each(|m| {
                strip_whitespace_node(
                    m,
                    cpi,
                    strip,
                    preserve,
                    if ss > -1.0 {
                        ps >= ss
                    } else if ps > -1.0 {
                        true
                    } else {
                        keep
                    },
                )
            })?
        }
        NodeType::Text => {
            if n.to_string().trim().is_empty() && !keep {
                n.pop()?;
            }
        }
        _ => {}
    }
    Ok(())
}
