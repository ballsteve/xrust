use crate::item::Node;
use crate::namespace::NamespaceMap;
use crate::parser::combinators::alt::{alt2, alt3};
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_while;
use crate::parser::combinators::tuple::tuple6;
use crate::parser::combinators::wellformed::wellformed;
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::common::{is_char10, is_char11};
use crate::parser::xml::chardata::chardata_unicode_codepoint;
use crate::parser::xml::qname::qualname;
use crate::parser::xml::reference::textreference;
use crate::parser::{ParseError, ParseInput};
use crate::qname::{Interner, QualifiedName};
use crate::{Error, ErrorKind};
use std::collections::HashSet;
use std::rc::Rc;

/// Parse all of the attributes in an element's start tag.
/// Returns (attribute nodes, namespace declaration nodes).
pub(crate) fn attributes<'a, 'i, I: Interner, N: Node>() -> impl Fn(
    ParseInput<'a, 'i, I, N>,
) -> Result<
    (
        ParseInput<'a, 'i, I, N>,
        (Vec<(QualifiedName<'i, I>, String)>, Vec<N>),
    ),
    ParseError,
> {
    move |input| match many0(attribute())(input) {
        Ok(((input1, mut state1), nodes)) => {
            let doc = state1.doc.clone().unwrap().clone();

            // If new namespaces are declared, then construct a new namespace hashmap
            // with the old entries overlaid with the new entries.
            // Otherwise, use the existing hashmap.
            // To do this, we need to know whether new namespaces are being declared,
            // so put these in a vector.
            let mut new_namespaces = vec![];
            let mut new_namespace_prefixes = HashSet::new();

            for (qn, val) in nodes.clone() {
                // Cache qn, val string values for faster comparison
                let qn_str = qn.to_string();
                let qn_prefix = qn.prefix();
                let qn_prefix_str = qn_prefix.map_or(String::from(""), |p| p);
                let qn_localname = qn.local_part();
                let val_str = val.to_string();

                //Return error if someone attempts to redefine namespaces.
                if qn_prefix_str == "xmlns" && qn_localname == "xmlns" {
                    return Err(ParseError::NotWellFormed(String::from(
                        "cannot redefine namespace",
                    )));
                }
                //xml prefix must always be set to http://www.w3.org/XML/1998/namespace
                if qn_prefix_str == "xmlns"
                    && qn_localname == "xml"
                    && val_str != "http://www.w3.org/XML/1998/namespace"
                {
                    return Err(ParseError::NotWellFormed(String::from(
                        "xml namespace URI must be http://www.w3.org/XML/1998/namespace",
                    )));
                }
                // http://www.w3.org/XML/1998/namespace must always be bound to xml
                if qn_prefix_str == "xmlns"
                    && qn_localname != "xml"
                    && val_str == "http://www.w3.org/XML/1998/namespace"
                {
                    return Err(ParseError::NotWellFormed(String::from(
                        "XML namespace must be bound to xml prefix",
                    )));
                }
                // http://www.w3.org/2000/xmlns/ must always be bound to xmlns
                if qn_prefix_str == "xmlns"
                    && qn_localname != "xmlns"
                    && val_str == "http://www.w3.org/2000/xmlns/"
                {
                    return Err(ParseError::NotWellFormed(String::from(
                        "XMLNS namespace must be bound to xmlns prefix",
                    )));
                }
                // Default namespace cannot be http://www.w3.org/XML/1998/namespace
                // Default namespace cannot be http://www.w3.org/2000/xmlns/
                if qn_prefix_str.is_empty()
                    && qn_localname == "xmlns"
                    && (val_str == "http://www.w3.org/XML/1998/namespace"
                        || val_str == "http://www.w3.org/2000/xmlns/")
                {
                    return Err(ParseError::NotWellFormed(String::from(
                        "invalid default namespace",
                    )));
                }

                // XML 1.0 documents cannot redefine an alias to ""
                if qn_prefix_str == "xmlns"
                    && !qn_localname.is_empty()
                    && val_str.is_empty()
                    && state1.xmlversion == *"1.0"
                {
                    return Err(ParseError::NotWellFormed(String::from(
                        "cannot redefine alias to empty",
                    )));
                }

                if qn_prefix_str == "xmlns" {
                    new_namespaces.push(
                        doc.new_namespace(val, Some(qn.local_part()))
                            .expect("unable to create namespace node"),
                    );
                    match new_namespace_prefixes.insert(Some(qn.local_part())) {
                        true => {}
                        false => {
                            return Err(ParseError::NotWellFormed(String::from(
                                "duplicate namespace declaration",
                            )));
                        }
                    }
                    //namespaces.insert(Some(qn.get_localname()), val.to_string());
                    //resnsnodes.insert(Some(qn.get_localname()), val.to_string());
                } else if qn_localname == "xmlns" && !val_str.is_empty() {
                    new_namespaces.push(
                        doc.new_namespace(val, None)
                            .expect("unable to create default namespace node"),
                    );
                    match new_namespace_prefixes.insert(None) {
                        true => {}
                        false => {
                            return Err(ParseError::NotWellFormed(String::from(
                                "duplicate namespace declaration",
                            )));
                        }
                    }
                    //namespaces.insert(None, val.to_string());
                    //resnsnodes.insert(None, val.to_string());
                };
                // If the namespace is set like xmlns="", we remove from the list
                // TODO: improve handling of undeclaring the default namespace
                if qn_localname == "xmlns" && val_str.is_empty() {
                    //namespaces.remove(&None);
                    //resnsnodes.remove(&None);
                };

                //Check if the xml:space attribute is present and if so, does it have
                //"Preserved" or "Default" as its value. We'll actually handle in a future release.
                if qn_prefix_str == "xml"
                    && qn_localname == "space"
                    && !(qn_str == "Default" || qn_str == "Preserve")
                {
                    return Err(ParseError::Validation {
                        row: state1.currentrow,
                        col: state1.currentcol,
                    });
                }
            }
            // Now construct the namespace hashmap, if required
            //state1.namespace.push(namespaces.clone());
            if !new_namespaces.is_empty() {
                // We will build a new namespace hashmap
                let mut new_ns_hm = NamespaceMap::new();
                state1.namespace.iter().for_each(|(old_prefix, old_nsuri)| {
                    new_ns_hm.insert(old_prefix.clone(), old_nsuri.clone());
                });
                new_namespaces.iter().for_each(|nsnode| {
                    let prefix = nsnode.name::<I>().unwrap().local_part();
                    let o = if prefix.to_string().is_empty() {
                        None
                    } else {
                        Some(prefix)
                    };
                    new_ns_hm.insert(o, nsnode.value().to_string());
                });
                state1.namespace = Rc::new(new_ns_hm);
            } // else just reuse the existing hashmap

            //Why loop through the nodes a second time? XML attributes are not in any order, so the
            //namespace declaration can happen after the attribute if it has a namespace prefix.
            // SRB: TODO: partition the nodes vector based on whether the attribute has a prefix (and is not a namespace declaration)
            // Then loop through the prefixed attributes after the namespaces have been processed
            let mut resnodes = vec![];
            //This vec tracks duplicate attrs
            let mut resnodenames = vec![];

            for (mut qn, attrval) in nodes {
                let qn_prefix = qn.prefix().map_or(String::from(""), |s| s);
                let qn_localname = qn.local_part();
                if qn_prefix != "xmlns" && qn_localname != "xmlns" {
                    if qn
                        .resolve(|p| {
                            state1.namespace.get(&p).map_or(
                                Err(Error::new(
                                    ErrorKind::DynamicAbsent,
                                    "no namespace for prefix",
                                )),
                                |r| Ok(r.clone()),
                            )
                        })
                        .is_err()
                    {
                        return Err(ParseError::MissingNameSpace);
                    }
                    if qn_prefix != "xmlns" && !qn_prefix.is_empty() {
                        match qn.namespace_uri() {
                            None => {
                                return Err(ParseError::MissingNameSpace);
                            }
                            Some(u) => {
                                if u.to_string().is_empty() {
                                    return Err(ParseError::MissingNameSpace);
                                }
                            }
                        }
                    }
                    /*
                        We don't return fully completed attributes here because we need to do some
                        DTD checking on the element to manage IDs.
                    */
                    resnodes.push((qn.clone(), attrval));

                    /* Why not just use resnodes.contains()  ? I don't know how to do partial matching */
                    if resnodenames.contains(&(qn.namespace_uri(), qn.local_part())) {
                        return Err(ParseError::NotWellFormed(String::from(
                            "duplicate attributes",
                        )));
                    } else {
                        resnodenames.push((qn.namespace_uri(), qn.local_part()));
                    }
                }
            }
            Ok(((input1, state1), (resnodes, new_namespaces)))
        }
        Err(err) => Err(err),
    }
}
// Attribute ::= Name '=' AttValue
fn attribute<'a, 'i, I: Interner, N: Node>() -> impl Fn(
    ParseInput<'a, 'i, I, N>,
) -> Result<
    (ParseInput<'a, 'i, I, N>, (QualifiedName<'i, I>, String)),
    ParseError,
> {
    move |(input, state)| match tuple6(
        whitespace1(),
        qualname(),
        whitespace0(),
        tag("="),
        whitespace0(),
        attribute_value(),
    )((input, state))
    {
        Ok(((input1, state1), (_, n, _, _, _, s))) => Ok(((input1, state1.clone()), (n, s))),
        Err(e) => Err(e),
    }
}

fn attribute_value<'a, 'i, I: Interner, N: Node>(
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, String), ParseError> {
    move |(input, state)| {
        let parse = alt2(
            delimited(
                tag("'"),
                many0(alt3(
                    map(chardata_unicode_codepoint(), |c| c.to_string()),
                    textreference(),
                    wellformed(take_while(|c| c != '&' && c != '\''), |c| !c.contains('<')),
                )),
                tag("'"),
            ),
            delimited(
                tag("\""),
                many0(alt3(
                    map(chardata_unicode_codepoint(), |c| c.to_string()),
                    textreference(),
                    wellformed(take_while(|c| c != '&' && c != '\"'), |c| !c.contains('<')),
                )),
                tag("\""),
            ),
        )((input, state));

        match parse {
            Err(e) => Err(e),
            Ok(((input1, state1), rn)) => {
                /*  For each character, entity reference, or character reference in the unnormalized
                   attribute value, beginning with the first and continuing to the last, do the following:

                   For a character reference, append the referenced character to the normalized value.
                   For an entity reference, recursively apply step 3 of this algorithm to the replacement text of the entity.
                   For a white space character (#x20, #xD, #xA, #x9), append a space character (#x20) to the normalized value.
                   For another character, append the character to the normalized value.
                */
                let r = if state1.xmlversion == "1.1" {
                    rn.concat()
                        .replace(['\u{85}', '\u{2028}', '\n', '\r', '\t', '\n'], " ")
                        .trim()
                        .to_string()
                } else {
                    rn.concat()
                        .replace(['\n', '\r', '\t', '\n'], " ")
                        .trim()
                        .to_string()
                };
                //NEL character cannot be in attributes.
                if state1.xmlversion == "1.1" && r.find(|c| !is_char11(&c)).is_some() {
                    Err(ParseError::NotWellFormed(r))
                } else if r.find(|c| !is_char10(&c)).is_some() {
                    Err(ParseError::NotWellFormed(r))
                } else if r.contains('\u{0085}') {
                    Err(ParseError::NotWellFormed(r))
                } else {
                    Ok(((input1, state1), r))
                }
            }
        }
    }
}
