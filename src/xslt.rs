/*! ## An XSLT compiler

Compile an XSLT stylesheet into a [Transform]ation.

Once the stylesheet has been compiled, it may then be evaluated with an appropriate context.

NB. This module, by default, does not resolve include or import statements. See the xrust-net crate for a helper module to do that.

```rust
use std::rc::Rc;
use xrust::xdmerror::{Error, ErrorKind};
use xrust::qname::QualifiedName;
use xrust::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use xrust::transform::Transform;
use xrust::transform::context::{StaticContext, StaticContextBuilder};
use xrust::trees::smite::{RNode, Node as SmiteNode};
use xrust::parser::xml::parse;
use xrust::xslt::from_document;

// A little helper function to parse an XML document
fn make_from_str(s: &str) -> Result<RNode, Error> {
    let doc = Rc::new(SmiteNode::new());
    let e = parse(doc.clone(), s, None)?;
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
    vec![],
    None,
    make_from_str,
    |_| Ok(String::new())
).expect("failed to compile stylesheet");

// Set the source document as the context item
ctxt.context(vec![src], 0);
// Make an empty result document
ctxt.result_document(Rc::new(SmiteNode::new()));

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
use crate::output::*;
use crate::parser::avt::parse as parse_avt;
use crate::parser::xpath::parse;
use crate::pattern::Pattern;
use crate::qname::*;
use crate::transform::callable::{ActualParameters, Callable, FormalParameters};
use crate::transform::context::{Context, ContextBuilder};
use crate::transform::numbers::{Level, Numbering};
use crate::transform::template::Template;
use crate::transform::{
    Axis, Grouping, KindTest, NameTest, NodeMatch, NodeTest, Order, Transform, WildcardOrName,
};
use crate::value::*;
use crate::xdmerror::*;
use std::convert::TryFrom;
use url::Url;

const XSLTNS: &str = "http://www.w3.org/1999/XSL/Transform";

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
    stylens: Vec<HashMap<Option<String>, String>>,
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
            if !(root.name().get_nsuri_ref() == Some(XSLTNS)
                && (root.name().get_localname() == "stylesheet"
                    || root.name().get_localname() == "transform"))
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
            ))
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
            prefix: None,
            name: Some(WildcardOrName::Wildcard),
        })],
        &vec![NodeTest::Name(NameTest {
            ns: Some(WildcardOrName::Name(XSLTNS.to_string())),
            prefix: Some("xsl".to_string()),
            name: Some(WildcardOrName::Name("text".to_string())),
        })],
    )?;

    // Setup the serialization of the primary result document
    let mut od = OutputDefinition::new();
    if let Some(c) = stylenode.child_iter().find(|c| {
        !(c.is_element()
            && c.name().get_nsuri_ref() == Some(XSLTNS)
            && c.name().get_localname() == "output")
    }) {
        let b: bool = matches!(
            c.get_attribute(&QualifiedName::new(None, None, "indent".to_string()))
                .to_string()
                .as_str(),
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
        .filter(|c| {
            c.is_element()
                && c.name().get_nsuri_ref() == Some(XSLTNS)
                && c.name().get_localname() == "include"
        })
        .try_for_each(|mut c| {
            let h = c.get_attribute(&QualifiedName::new(None, None, "href".to_string()));
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
        .filter(|c| {
            c.is_element()
                && c.name().get_nsuri_ref() == Some(XSLTNS)
                && c.name().get_localname() == "import"
        })
        .try_for_each(|mut c| {
            let h = c.get_attribute(&QualifiedName::new(None, None, "href".to_string()));
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
                    let newat = styledoc.new_attribute(
                        QualifiedName::new(
                            Some(String::from("http://github.com/ballsteve/xrust")),
                            None,
                            String::from("import"),
                        ),
                        Rc::new(Value::from(1)),
                    )?;
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
    let mut attr_sets: HashMap<QualifiedName, Vec<Transform<N>>> = HashMap::new();

    stylenode
        .child_iter()
        .filter(|c| {
            c.is_element()
                && c.name().get_nsuri_ref() == Some(XSLTNS)
                && c.name().get_localname() == "attribute-set"
        })
        .try_for_each(|c| {
            let name = c.get_attribute(&QualifiedName::new(None, None, "name"));
            let eqname = QualifiedName::try_from((name.to_string().as_str(), &stylens))?;
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
                .filter(|c| {
                    c.is_element()
                        && c.name().get_nsuri_ref() == Some(XSLTNS)
                        && c.name().get_localname() == "attribute"
                })
                .try_for_each(|a| {
                    attrs.push(to_transform(a, &stylens, &attr_sets)?);
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
        .filter(|c| {
            c.is_element()
                && c.name().get_nsuri_ref() == Some(XSLTNS)
                && c.name().get_localname() == "template"
        })
        .filter(|c| {
            !c.get_attribute(&QualifiedName::new(None, None, "match"))
                .to_string()
                .is_empty()
        })
        .try_for_each(|c| {
            let m = c.get_attribute(&QualifiedName::new(None, None, "match"));
            let pat = Pattern::try_from(m.to_string()).map_err(|e| {
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
            let mut body = vec![];
            let mode = c.get_attribute_node(&QualifiedName::new(None, None, "mode"));
            c.child_iter().try_for_each(|d| {
                body.push(to_transform(d, &stylens, &attr_sets)?);
                Ok::<(), Error>(())
            })?;
            //sc.static_analysis(&mut pat);
            //sc.static_analysis(&mut body);
            // Determine the priority of the template
            let pr = c.get_attribute(&QualifiedName::new(None, None, "priority".to_string()));
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
                            let (t, nt, q) = s[0].get_ref();
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
            let im = c.get_attribute(&QualifiedName::new(
                Some(String::from("http://github.com/ballsteve/xrust")),
                None,
                String::from("import"),
            ));
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
                    QualifiedName::try_from((n.to_string().as_str(), &stylens))
                        .expect("unable to resolve qualified name")
                }), // TODO: don't panic
            ));
            Ok::<(), Error>(())
        })?;

    // Iterate over the children, looking for key declarations.
    // NB. could combine this with the previous loop, but performance shouldn't be an issue.
    let mut keys = vec![];
    stylenode
        .child_iter()
        .filter(|c| {
            c.is_element()
                && c.name().get_nsuri_ref() == Some(XSLTNS)
                && c.name().get_localname() == "key"
        })
        .try_for_each(|c| {
            let name = c.get_attribute(&QualifiedName::new(None, None, "name".to_string()));
            let m = c.get_attribute(&QualifiedName::new(None, None, "match".to_string()));
            let pat = Pattern::try_from(m.to_string())?;
            let u = c.get_attribute(&QualifiedName::new(None, None, "use".to_string()));
            keys.push((name, pat, parse::<N>(&u.to_string())?));
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
        ))
        // This matches "text()" and copies content
        .template(Template::new(
            Pattern::try_from("child::text()")?,
            Transform::ContextItem,
            None,
            vec![0],
            None,
            None,
        ))
        .template_all(templates)
        .output_definition(od)
        .namespaces(stylens.clone())
        .build();
    keys.iter()
        .for_each(|(name, m, u)| newctxt.declare_key(name.to_string(), m.clone(), u.clone()));

    // Add named templates
    stylenode
        .child_iter()
        .filter(|c| {
            c.is_element()
                && c.name().get_nsuri_ref() == Some(XSLTNS)
                && c.name().get_localname() == "template"
        })
        .filter(|c| {
            !c.get_attribute(&QualifiedName::new(None, None, "name"))
                .to_string()
                .is_empty()
        })
        .try_for_each(|c| {
            let name = c.get_attribute(&QualifiedName::new(None, None, "name"));
            // xsl:param for formal parameters
            // TODO: validate that xsl:param elements come first in the child list
            // TODO: validate that xsl:param elements have unique name attributes
            let mut params: Vec<(QualifiedName, Option<Transform<N>>)> = Vec::new();
            c.child_iter()
                .filter(|c| {
                    c.is_element()
                        && c.name().get_nsuri_ref() == Some(XSLTNS)
                        && c.name().get_localname() == "param"
                })
                .try_for_each(|c| {
                    let p_name = c.get_attribute(&QualifiedName::new(None, None, "name"));
                    if p_name.to_string().is_empty() {
                        Err(Error::new(
                            ErrorKind::StaticAbsent,
                            "name attribute is missing",
                        ))
                    } else {
                        let sel = c.get_attribute(&QualifiedName::new(None, None, "select"));
                        if sel.to_string().is_empty() {
                            // xsl:param content is the sequence constructor
                            let mut body = vec![];
                            c.child_iter().try_for_each(|d| {
                                body.push(to_transform(d, &stylens, &attr_sets)?);
                                Ok(())
                            })?;
                            params.push((
                                QualifiedName::new(None, None, p_name.to_string()),
                                Some(Transform::SequenceItems(body)),
                            ));
                            Ok(())
                        } else {
                            // select attribute value is an expression
                            params.push((
                                QualifiedName::new(None, None, p_name.to_string()),
                                Some(parse::<N>(&sel.to_string())?),
                            ));
                            Ok(())
                        }
                    }
                })?;
            // Content is the template body
            let mut body = vec![];
            c.child_iter()
                .filter(|c| {
                    !(c.is_element()
                        && c.name().get_nsuri_ref() == Some(XSLTNS)
                        && c.name().get_localname() == "param")
                })
                .try_for_each(|d| {
                    body.push(to_transform(d, &stylens, &attr_sets)?);
                    Ok::<(), Error>(())
                })?;
            newctxt.callable_push(
                QualifiedName::new(None, None, name.to_string()),
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
        .filter(|c| {
            c.is_element()
                && c.name().get_nsuri_ref() == Some(XSLTNS)
                && c.name().get_localname() == "function"
        })
        .try_for_each(|c| {
            let name = c.get_attribute(&QualifiedName::new(None, None, "name"));
            // Name must have a namespace. See XSLT 10.3.1.
            let eqname =
                QualifiedName::try_from((name.to_string().as_str(), newctxt.namespaces_ref()))?;
            if eqname.get_nsuri_ref().is_none() {
                return Err(Error::new_with_code(
                    ErrorKind::StaticAbsent,
                    "function name must have a namespace",
                    Some(QualifiedName::new(None, None, "XTSE0740")),
                ));
            }
            // xsl:param for formal parameters
            // TODO: validate that xsl:param elements come first in the child list
            // TODO: validate that xsl:param elements have unique name attributes
            let mut params: Vec<QualifiedName> = Vec::new();
            c.child_iter()
                .filter(|c| {
                    c.is_element()
                        && c.name().get_nsuri_ref() == Some(XSLTNS)
                        && c.name().get_localname() == "param"
                })
                .try_for_each(|c| {
                    let p_name = c.get_attribute(&QualifiedName::new(None, None, "name"));
                    if p_name.to_string().is_empty() {
                        Err(Error::new(
                            ErrorKind::StaticAbsent,
                            "name attribute is missing",
                        ))
                    } else {
                        // TODO: validate that xsl:param elements do not specify a default value. See XSLT 10.3.2.
                        params.push(QualifiedName::new(None, None, p_name.to_string()));
                        Ok(())
                    }
                })?;
            // Content is the function body
            let mut body = vec![];
            c.child_iter()
                .filter(|c| {
                    !(c.is_element()
                        && c.name().get_nsuri_ref() == Some(XSLTNS)
                        && c.name().get_localname() == "param")
                })
                .try_for_each(|d| {
                    body.push(to_transform(d, &stylens, &attr_sets)?);
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

    Ok(newctxt)
}

/// Compile a node in a template to a sequence [Combinator]
fn to_transform<N: Node>(
    n: N,
    ns: &Vec<HashMap<Option<String>, String>>,
    attr_sets: &HashMap<QualifiedName, Vec<Transform<N>>>,
) -> Result<Transform<N>, Error> {
    match n.node_type() {
        NodeType::Text => Ok(Transform::Literal(Item::Value(Rc::new(Value::String(
            n.to_string(),
        ))))),
        NodeType::Element => {
            match (n.name().get_nsuri_ref(), n.name().get_localname().as_str()) {
                (Some(XSLTNS), "text") => {
                    let doe = n.get_attribute(&QualifiedName::new(
                        None,
                        None,
                        "disable-output-escaping".to_string(),
                    ));
                    if !doe.to_string().is_empty() {
                        match &doe.to_string()[..] {
                            "yes" => Ok(Transform::Literal(Item::Value(Rc::new(Value::String(
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
                                "disable-output-escaping only accepts values yes or no."
                                    .to_string(),
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
                }
                (Some(XSLTNS), "value-of") => {
                    let sel =
                        n.get_attribute(&QualifiedName::new(None, None, "select".to_string()));
                    let doe = n.get_attribute(&QualifiedName::new(
                        None,
                        None,
                        "disable-output-escaping".to_string(),
                    ));
                    if !doe.to_string().is_empty() {
                        match &doe.to_string()[..] {
                            "yes" => Ok(Transform::LiteralText(
                                Box::new(parse::<N>(&sel.to_string())?),
                                true,
                            )),
                            "no" => Ok(Transform::LiteralText(
                                Box::new(parse::<N>(&sel.to_string())?),
                                false,
                            )),
                            _ => Err(Error::new(
                                ErrorKind::TypeError,
                                "disable-output-escaping only accepts values yes or no."
                                    .to_string(),
                            )),
                        }
                    } else {
                        Ok(Transform::LiteralText(
                            Box::new(parse::<N>(&sel.to_string())?),
                            false,
                        ))
                    }
                }
                (Some(XSLTNS), "apply-templates") => {
                    let sel = n.get_attribute(&QualifiedName::new(None, None, "select"));
                    let m = n.get_attribute_node(&QualifiedName::new(None, None, "mode"));
                    let sort_keys = get_sort_keys(&n)?;
                    if !sel.to_string().is_empty() {
                        Ok(Transform::ApplyTemplates(
                            Box::new(parse::<N>(&sel.to_string())?),
                            m.map(|s| {
                                QualifiedName::try_from((s.to_string().as_str(), ns))
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
                                QualifiedName::try_from((s.to_string().as_str(), ns))
                                    .expect("unable to resolve qualified name")
                            }),
                            sort_keys,
                        )) // TODO: don't panic
                    }
                }
                (Some(XSLTNS), "apply-imports") => Ok(Transform::ApplyImports),
                (Some(XSLTNS), "sequence") => {
                    let s = n.get_attribute(&QualifiedName::new(None, None, "select".to_string()));
                    if !s.to_string().is_empty() {
                        Ok(parse::<N>(&s.to_string())?)
                    } else {
                        Result::Err(Error::new(
                            ErrorKind::TypeError,
                            "missing select attribute".to_string(),
                        ))
                    }
                }
                (Some(XSLTNS), "if") => {
                    let t = n.get_attribute(&QualifiedName::new(None, None, "test".to_string()));
                    if !t.to_string().is_empty() {
                        Ok(Transform::Switch(
                            vec![(
                                parse::<N>(&t.to_string())?,
                                Transform::SequenceItems(n.child_iter().try_fold(
                                    vec![],
                                    |mut body, e| {
                                        body.push(to_transform(e, ns, attr_sets)?);
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
                }
                (Some(XSLTNS), "choose") => {
                    let mut clauses: Vec<(Transform<N>, Transform<N>)> = Vec::new();
                    let mut otherwise: Option<Transform<N>> = None;
                    let mut status: Option<Error> = None;
                    n.child_iter()
                        .try_for_each(|m| {
                            // look for when elements
                            // then find an otherwise
                            // fail on anything else (apart from whitespace, comments, PIs)
                            match m.node_type() {
                                NodeType::Element => {
                                    match (m.name().get_nsuri_ref(), m.name().get_localname().as_str()) {
                                        (Some(XSLTNS), "when") => {
                                            if otherwise.is_none() {
                                                let t = m.get_attribute(&QualifiedName::new(None, None, "test".to_string()));
                                                if !t.to_string().is_empty() {
                                                    clauses.push((
                                                        parse::<N>(&t.to_string())?,
                                                        Transform::SequenceItems(
                                                            m.child_iter()
                                                                .try_fold(
                                                                    vec![],
                                                                    |mut body, e| {
                                                                        body.push(to_transform(e, ns, attr_sets)?);
                                                                        Ok(body)
                                                                    },
                                                                )?
                                                        )
                                                    ));
                                                } else {
                                                    status.replace(Error::new(ErrorKind::TypeError, "missing test attribute".to_string()));
                                                }
                                            } else {
                                                status.replace(Error::new(ErrorKind::TypeError, "invalid content in choose element: when follows otherwise".to_string()));
                                            }
                                        }
                                        (Some(XSLTNS), "otherwise") => {
                                            if !clauses.is_empty() {
                                                otherwise = Some(Transform::SequenceItems(m.child_iter()
                                                    .try_fold(
                                                        vec![],
                                                        |mut o, e| {
                                                            o.push(to_transform(e, ns, attr_sets)?);
                                                            Ok(o)
                                                        },
                                                    )?));
                                            } else {
                                                status.replace(Error::new(ErrorKind::TypeError, "invalid content in choose element: no when elements".to_string()));
                                            }
                                        }
                                        _ => {
                                            status.replace(Error::new(ErrorKind::TypeError, "invalid element content in choose element".to_string()));
                                        }
                                    }
                                }
                                NodeType::Text => {
                                    if !n.to_string().trim().is_empty() {
                                        status.replace(Error::new(ErrorKind::TypeError, "invalid text content in choose element".to_string()));
                                    }
                                }
                                NodeType::Comment |
                                NodeType::ProcessingInstruction => {}
                                _ => {
                                    status.replace(Error::new(ErrorKind::TypeError, "invalid content in choose element".to_string()));
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
                }
                (Some(XSLTNS), "for-each") => {
                    let s = n.get_attribute(&QualifiedName::new(None, None, "select".to_string()));
                    if !s.to_string().is_empty() {
                        Ok(Transform::ForEach(
                            None,
                            Box::new(parse::<N>(&s.to_string())?),
                            Box::new(Transform::SequenceItems(n.child_iter().try_fold(
                                vec![],
                                |mut body, e| {
                                    body.push(to_transform(e, ns, attr_sets)?);
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
                }
                (Some(XSLTNS), "for-each-group") => {
                    let ord = get_sort_keys(&n)?;
                    let s = n.get_attribute(&QualifiedName::new(None, None, "select".to_string()));
                    if !s.to_string().is_empty() {
                        match (
                            n.get_attribute(&QualifiedName::new(
                                None,
                                None,
                                "group-by".to_string(),
                            ))
                            .to_string()
                            .as_str(),
                            n.get_attribute(&QualifiedName::new(
                                None,
                                None,
                                "group-adjacent".to_string(),
                            ))
                            .to_string()
                            .as_str(),
                            n.get_attribute(&QualifiedName::new(
                                None,
                                None,
                                "group-starting-with".to_string(),
                            ))
                            .to_string()
                            .as_str(),
                            n.get_attribute(&QualifiedName::new(
                                None,
                                None,
                                "group-ending-with".to_string(),
                            ))
                            .to_string()
                            .as_str(),
                        ) {
                            (by, "", "", "") => Ok(Transform::ForEach(
                                Some(Grouping::By(vec![parse::<N>(by)?])),
                                Box::new(parse::<N>(&s.to_string())?),
                                Box::new(Transform::SequenceItems(n.child_iter().try_fold(
                                    vec![],
                                    |mut body, e| {
                                        body.push(to_transform(e, ns, attr_sets)?);
                                        Ok(body)
                                    },
                                )?)),
                                ord,
                            )),
                            ("", adj, "", "") => Ok(Transform::ForEach(
                                Some(Grouping::Adjacent(vec![parse::<N>(adj)?])),
                                Box::new(parse::<N>(&s.to_string())?),
                                Box::new(Transform::SequenceItems(n.child_iter().try_fold(
                                    vec![],
                                    |mut body, e| {
                                        body.push(to_transform(e, ns, attr_sets)?);
                                        Ok(body)
                                    },
                                )?)),
                                ord,
                            )),
                            // TODO: group-starting-with and group-ending-with
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
                }
                (Some(XSLTNS), "copy") => {
                    // TODO: handle select attribute
                    let mut content: Vec<Transform<N>> =
                        n.child_iter().try_fold(vec![], |mut body, e| {
                            body.push(to_transform(e, ns, attr_sets)?);
                            Ok(body)
                        })?;
                    // Process @xsl:use-attribute-sets
                    let use_atts = n.get_attribute(&QualifiedName::new(
                        Some(XSLTNS.to_string()),
                        None,
                        "use-attribute-sets",
                    ));
                    let mut attrs = vec![];
                    use_atts.to_string().split_whitespace().try_for_each(|a| {
                        let eqa = QualifiedName::try_from((a, ns))?;
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
                }
                (Some(XSLTNS), "copy-of") => {
                    let s = n.get_attribute(&QualifiedName::new(None, None, "select".to_string()));
                    if !s.to_string().is_empty() {
                        Ok(Transform::DeepCopy(Box::new(parse::<N>(&s.to_string())?)))
                    } else {
                        Ok(Transform::DeepCopy(Box::new(Transform::ContextItem)))
                    }
                }
                (Some(XSLTNS), "call-template") => {
                    let name = n.get_attribute(&QualifiedName::new(None, None, "name"));
                    if !name.to_string().is_empty() {
                        // Iterate over the xsl:with-param elements to get the actual parameters
                        // TODO: validate that the children are only xsl:with-param elements
                        let mut ap = vec![];
                        n.child_iter()
                            .filter(|c| {
                                c.is_element()
                                    && c.name().get_nsuri_ref() == Some(XSLTNS)
                                    && c.name().get_localname() == "with-param"
                            })
                            .try_for_each(|c| {
                                let wp_name =
                                    c.get_attribute(&QualifiedName::new(None, None, "name"));
                                if !wp_name.to_string().is_empty() {
                                    let sel =
                                        c.get_attribute(&QualifiedName::new(None, None, "select"));
                                    if sel.to_string().is_empty() {
                                        // xsl:with-param content is the sequence constructor
                                        let mut body = vec![];
                                        c.child_iter().try_for_each(|d| {
                                            body.push(to_transform(d, ns, attr_sets)?);
                                            Ok(())
                                        })?;
                                        ap.push((
                                            QualifiedName::new(None, None, wp_name.to_string()),
                                            Transform::SequenceItems(body),
                                        ));
                                        Ok(())
                                    } else {
                                        // select attribute value is an expression
                                        ap.push((
                                            QualifiedName::new(None, None, wp_name.to_string()),
                                            parse::<N>(&sel.to_string())?,
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
                            QualifiedName::new(None, None, name.to_string()),
                            ActualParameters::Named(ap),
                        ))
                    } else {
                        Err(Error::new(
                            ErrorKind::StaticAbsent,
                            "name attribute missing",
                        ))
                    }
                }
                (Some(XSLTNS), "element") => {
                    let m = n.get_attribute(&QualifiedName::new(None, None, "name".to_string()));
                    if m.to_string().is_empty() {
                        return Err(Error::new(ErrorKind::TypeError, "missing name attribute"));
                    }
                    let mut content = n.child_iter().try_fold(vec![], |mut body, e| {
                        body.push(to_transform(e, ns, attr_sets)?);
                        Ok(body)
                    })?;
                    // Process @xsl:use-attribute-sets
                    let use_atts = n.get_attribute(&QualifiedName::new(
                        Some(XSLTNS.to_string()),
                        None,
                        "use-attribute-sets",
                    ));
                    let mut attrs = vec![];
                    use_atts.to_string().split_whitespace().try_for_each(|a| {
                        let eqa = QualifiedName::try_from((a, ns))?;
                        attr_sets
                            .get(&eqa)
                            .iter()
                            .cloned()
                            .for_each(|a| attrs.append(&mut a.clone()));
                        Ok(())
                    })?;

                    Ok(Transform::Element(
                        Box::new(parse_avt(m.to_string().as_str())?),
                        Box::new(if content.is_empty() && attrs.is_empty() {
                            Transform::Empty
                        } else {
                            // Attributes always come first
                            attrs.append(&mut content);
                            Transform::SequenceItems(attrs)
                        }),
                    ))
                }
                (Some(XSLTNS), "attribute") => {
                    let m = n.get_attribute(&QualifiedName::new(None, None, "name".to_string()));
                    if !m.to_string().is_empty() {
                        Ok(Transform::LiteralAttribute(
                            QualifiedName::new(None, None, m.to_string()),
                            Box::new(Transform::SequenceItems(n.child_iter().try_fold(
                                vec![],
                                |mut body, e| {
                                    body.push(to_transform(e, ns, attr_sets)?);
                                    Ok(body)
                                },
                            )?)),
                        ))
                    } else {
                        Err(Error::new(
                            ErrorKind::TypeError,
                            "missing name attribute".to_string(),
                        ))
                    }
                }
                (Some(XSLTNS), "comment") => Ok(Transform::LiteralComment(Box::new(
                    Transform::SequenceItems(n.child_iter().try_fold(vec![], |mut body, e| {
                        body.push(to_transform(e, ns, attr_sets)?);
                        Ok(body)
                    })?),
                ))),
                (Some(XSLTNS), "processing-instruction") => {
                    let m = n.get_attribute(&QualifiedName::new(None, None, "name".to_string()));
                    if m.to_string().is_empty() {
                        return Result::Err(Error::new(
                            ErrorKind::TypeError,
                            "missing name attribute".to_string(),
                        ));
                    }
                    Ok(Transform::LiteralProcessingInstruction(
                        Box::new(parse_avt(m.to_string().as_str())?),
                        Box::new(Transform::SequenceItems(n.child_iter().try_fold(
                            vec![],
                            |mut body, e| {
                                body.push(to_transform(e, ns, attr_sets)?);
                                Ok(body)
                            },
                        )?)),
                    ))
                }
                (Some(XSLTNS), "message") => {
                    let t =
                        n.get_attribute(&QualifiedName::new(None, None, "terminate".to_string()));
                    Ok(Transform::Message(
                        Box::new(Transform::SequenceItems(n.child_iter().try_fold(
                            vec![],
                            |mut body, e| {
                                body.push(to_transform(e, ns, attr_sets)?);
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
                }
                (Some(XSLTNS), "number") => {
                    let value = n.get_attribute(&QualifiedName::new(None, None, "value"));
                    let sel = n.get_attribute(&QualifiedName::new(None, None, "select"));
                    let level = n.get_attribute(&QualifiedName::new(None, None, "level"));
                    if level.to_string() != "" && level.to_string() != "single" {
                        return Err(Error::new(
                            ErrorKind::NotImplemented,
                            "only single level numbering is supported",
                        ));
                    }
                    let count = n.get_attribute(&QualifiedName::new(None, None, "count"));
                    let from = n.get_attribute(&QualifiedName::new(None, None, "from"));
                    let format = n.get_attribute(&QualifiedName::new(None, None, "format"));
                    // TODO: lang, letter-value, ordinal, start-at, grouping-separator, grouping-size
                    if value.to_string().is_empty() {
                        // Compute place marker
                        Ok(Transform::FormatInteger(
                            Box::new(Transform::GenerateIntegers(
                                Box::new(Transform::Empty), // start-at (TODO)
                                Box::new(if sel.to_string().is_empty() {
                                    Transform::ContextItem
                                } else {
                                    parse::<N>(&sel.to_string())?
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
                            Box::new(parse::<N>(&value.to_string())?),
                            Box::new(Transform::Literal(Item::Value(
                                if format.to_string().is_empty() {
                                    Rc::new(Value::from("1"))
                                } else {
                                    format
                                },
                            ))),
                        ))
                    }
                }
                (Some(XSLTNS), "decimal-format") => Ok(Transform::NotImplemented(String::from(
                    "unsupported XSL element \"decimal-format\"",
                ))),
                (Some(XSLTNS), u) => Ok(Transform::NotImplemented(format!(
                    "unsupported XSL element \"{}\"",
                    u
                ))),
                (u, a) => {
                    // Process @xsl:use-attribute-sets
                    let use_atts = n.get_attribute(&QualifiedName::new(
                        Some(XSLTNS.to_string()),
                        None,
                        "use-attribute-sets",
                    ));
                    let mut attrs = vec![];
                    use_atts.to_string().split_whitespace().try_for_each(|a| {
                        let eqa = QualifiedName::try_from((a, ns))?;
                        attr_sets
                            .get(&eqa)
                            .iter()
                            .cloned()
                            .for_each(|a| attrs.append(&mut a.clone()));
                        Ok(())
                    })?;
                    let mut content = vec![];
                    // Copy attributes to the result, except for XSLT directives
                    n.attribute_iter()
                        .filter(|e| e.name().get_nsuri_ref() != Some(XSLTNS))
                        .try_for_each(|e| {
                            content.push(to_transform(e, ns, attr_sets)?);
                            Ok::<(), Error>(())
                        })?;
                    n.child_iter().try_for_each(|e| {
                        content.push(to_transform(e, ns, attr_sets)?);
                        Ok::<(), Error>(())
                    })?;
                    Ok(Transform::LiteralElement(
                        QualifiedName::new(
                            u.map(|v| v.to_string()),
                            n.name().get_prefix(),
                            a.to_string(),
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
        }
        NodeType::Attribute => {
            // Get value as a Value
            Ok(Transform::LiteralAttribute(
                n.name(),
                Box::new(Transform::Literal(Item::Value(Rc::new(Value::String(
                    n.to_string(),
                ))))),
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
    n.child_iter()
        .try_fold(vec![], |mut acc, c| match c.node_type() {
            NodeType::Element => {
                if c.name() == QualifiedName::new(Some(XSLTNS.to_string()), None, "sort") {
                    let ordval = c.get_attribute(&QualifiedName::new(None, None, "order"));
                    let ord = match ordval.to_string().as_str() {
                        "descending" => Order::Descending,
                        _ => Order::Ascending,
                    };
                    let sortsel = c.get_attribute(&QualifiedName::new(None, None, "select"));
                    acc.push((ord, parse::<N>(&sortsel.to_string())?));
                    Ok(acc)
                } else {
                    Err(Error::new(
                        ErrorKind::TypeError,
                        "only sort elements allowed",
                    ))
                }
            }
            NodeType::Text => {
                if c.value()
                    .to_string()
                    .as_str()
                    .find(|d: char| !d.is_whitespace())
                    .is_none()
                {
                    Ok(acc)
                } else {
                    Err(Error::new(
                        ErrorKind::TypeError,
                        "only sort elements allowed",
                    ))
                }
            }
            NodeType::Comment | NodeType::ProcessingInstruction => Ok(acc),
            _ => Err(Error::new(
                ErrorKind::TypeError,
                "only sort elements allowed",
            )),
        })
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
            match (
                m.node_type(),
                m.name().get_nsuri_ref(),
                m.name().get_localname().as_str(),
            ) {
                (NodeType::Element, Some(XSLTNS), "strip-space") => {
                    let v =
                        m.get_attribute(&QualifiedName::new(None, None, "elements".to_string()));
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
                }
                (NodeType::Element, Some(XSLTNS), "preserve-space") => {
                    let v =
                        m.get_attribute(&QualifiedName::new(None, None, "elements".to_string()));
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
                _ => {}
            }
            Ok::<(), Error>(())
        })?;
        Ok::<(), Error>(())
    })?;

    strip_whitespace(src, false, &ss, &ps)
}

// TODO: the rules for stripping/preserving are a lot more complex
// TODO: Return Result so that errors can be propagated
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
                        match (n.name().get_nsuri(), n.name().get_localname()) {
                            (Some(_), _) => {}
                            (None, ename) => {
                                if *name == ename {
                                    ss = 0.5;
                                }
                            }
                        }
                    }
                    (Some(WildcardOrName::Name(ns)), Some(WildcardOrName::Name(name))) => {
                        match (n.name().get_nsuri(), n.name().get_localname()) {
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
                        }
                    }
                    (Some(WildcardOrName::Wildcard), Some(WildcardOrName::Name(_))) => {
                        ss = -0.25;
                    }
                    (Some(WildcardOrName::Name(_)), Some(WildcardOrName::Wildcard)) => {
                        ss = -0.25;
                    }
                    (Some(WildcardOrName::Wildcard), Some(WildcardOrName::Wildcard)) => {
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
                        match (n.name().get_nsuri(), n.name().get_localname()) {
                            (Some(_), _) => {}
                            (None, ename) => {
                                if *name == ename {
                                    ps = 0.5;
                                }
                            }
                        }
                    }
                    (Some(WildcardOrName::Name(ns)), Some(WildcardOrName::Name(name))) => {
                        match (n.name().get_nsuri(), n.name().get_localname()) {
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
                        }
                    }
                    (Some(WildcardOrName::Wildcard), Some(WildcardOrName::Name(_))) => {
                        ps = -0.25;
                    }
                    (Some(WildcardOrName::Name(_)), Some(WildcardOrName::Wildcard)) => {
                        ps = -0.25;
                    }
                    (Some(WildcardOrName::Wildcard), Some(WildcardOrName::Wildcard)) => {
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
