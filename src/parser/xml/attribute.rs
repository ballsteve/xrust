use crate::item::Node;
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
use crate::parser::xml::qname::qualname_to_parts;
use crate::parser::xml::reference::textreference;
use crate::parser::{ParseError, ParseInput, StaticState};
use crate::value::{ID, Value};
use qualname::{NamespaceDeclaration, NamespacePrefix, NamespaceUri, NcName, QName};
use std::rc::Rc;

/// Parse all of the attributes in an element's start tag.
/// Returns (attribute nodes, namespace declaration nodes).
pub(crate) fn attributes<'a, N: Node, L>() -> impl Fn(
    ParseInput<'a, N>,
    &mut StaticState<L>,
) -> Result<
    (ParseInput<'a, N>, (Vec<N>, Vec<N>)),
    ParseError,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |input, ss| match many0(attribute())(input, ss) {
        Ok(((input1, mut state1), attrs)) => {
            // First separate namespace declarations from other attributes
            let (ns_decls, attr_list): (
                Vec<((Option<String>, String), String)>,
                Vec<((Option<String>, String), String)>,
            ) = attrs
                .into_iter()
                .partition(|((prefix, local_part), value)| {
                    match (prefix.as_deref(), local_part.as_str(), value) {
                        (Some("xmlns"), _, _) => true,
                        (None, "xmlns", _) => true,
                        _ => false,
                    }
                });

            let doc = state1.doc.clone().unwrap().clone();

            // Create namespace declaration nodes
            // and update in-scope namespace map
            // TODO: use try_collect()
            let mut nsd_vec: Vec<N> = vec![];
            let _ = ns_decls
                .iter()
                .try_for_each(|((prefix, local_part), value)| {
                    match (prefix.as_deref(), local_part.as_str(), value.as_str()) {
                        (Some("xmlns"), "xmlns", "http://www.w3.org/2000/xmlns/") => {
                            Err(ParseError::NotWellFormed(
                                "namespace definition not allowed".to_string(),
                            ))
                        }
                        (Some("xmlns"), "xmlns", _) => Err(ParseError::NotWellFormed(
                            String::from("namespace prefix \"xmlns\" not allowed"),
                        )),
                        (Some("xmlns"), "xml", "http://www.w3.org/XML/1998/namespace") => {
                            state1.in_scope_namespaces.push(
                                NamespaceDeclaration::new(
                                    Some(NamespacePrefix::try_from("xml").unwrap()),
                                    NamespaceUri::try_from("http://www.w3.org/XML/1998/namespace")
                                        .unwrap(),
                                )
                                .map_err(|_| {
                                    ParseError::NotWellFormed(
                                        "unable to create namespace declaration".to_string(),
                                    )
                                })?,
                            );
                            nsd_vec.push(
                                doc.new_namespace(
                                    NamespaceUri::try_from("http://www.w3.org/XML/1998/namespace")
                                        .unwrap(),
                                    Some(NamespacePrefix::try_from("xml").unwrap()),
                                    true,
                                )
                                .map_err(|_| ParseError::MissingNameSpace)?,
                            );
                            Ok(())
                        }
                        (Some("xmlns"), "xml", _) => Err(ParseError::NotWellFormed(String::from(
                            "XML namespace URI must be http://www.w3.org/XML/1998/namespace",
                        ))),
                        (Some("xmlns"), _, "http://www.w3.org/XML/1998/namespace") => {
                            Err(ParseError::NotWellFormed(String::from(
                                "XML namespace must be bound to xml prefix",
                            )))
                        }
                        (None, "xmlns", "http://www.w3.org/XML/1998/namespace")
                        | (None, "xmlns", "http://www.w3.org/2000/xmlns/") => Err(
                            ParseError::NotWellFormed(String::from("invalid default namespace")),
                        ),
                        (Some("xmlns"), p, "") => {
                            if state1.xmlversion == *"1.0" {
                                Err(ParseError::NotWellFormed(String::from(
                                    "cannot redefine alias to empty",
                                )))
                            } else {
                                // Descope the namespace
                                // First find the URI for this prefix. If not found then error.
                                let prefix = Some(NamespacePrefix::try_from(p).map_err(|_| {
                                    ParseError::NotWellFormed(String::from(
                                        "invalid namespace prefix",
                                    ))
                                })?);
                                if let Some(nsuri) = state1.in_scope_namespaces.namespace_uri(&prefix) {
                                    if state1.in_scope_namespaces.pop_prefix(&prefix).is_none() {
                                        return Err(ParseError::NotWellFormed(String::from("unable to descope namespace: not found in in-scope namespaces")))
                                    }
                                    nsd_vec.push(
                                        doc.new_namespace(
                                            nsuri,
                                            prefix,
                                            false,
                                        )
                                        .map_err(|_| ParseError::MissingNameSpace)?,
                                    );
                                } else {
                                    return Err(ParseError::NotWellFormed(String::from("unable to descope namespace: it has not been declared")))
                                }
                                Ok(())
                            }
                        }
                        (Some("xmlns"), p, v) => {
                            if v.is_empty() {
                                // A descoping declaration
                                nsd_vec.push(
                                    doc.new_namespace(
                                        NamespaceUri::try_from(v).unwrap(),
                                        Some(NamespacePrefix::try_from(p).map_err(|_| {
                                            ParseError::NotWellFormed(String::from(
                                                "invalid namespace prefix",
                                            ))
                                        })?),
                                        false,
                                    )
                                    .map_err(|_| ParseError::MissingNameSpace)?,
                                );
                            } else {
                                state1.in_scope_namespaces.push(
                                    NamespaceDeclaration::new(
                                        Some(NamespacePrefix::try_from(p).map_err(|_| {
                                            ParseError::NotWellFormed(String::from(
                                                "invalid namespace prefix",
                                            ))
                                        })?),
                                        NamespaceUri::try_from(v).unwrap(),
                                    )
                                    .map_err(|_| {
                                        ParseError::NotWellFormed(
                                            "unable to create namespace declaration".to_string(),
                                        )
                                    })?,
                                );
                                nsd_vec.push(
                                    doc.new_namespace(
                                        NamespaceUri::try_from(v).unwrap(),
                                        Some(NamespacePrefix::try_from(p).map_err(|_| {
                                            ParseError::NotWellFormed(String::from(
                                                "invalid namespace prefix",
                                            ))
                                        })?),
                                        true,
                                    )
                                    .map_err(|_| ParseError::MissingNameSpace)?,
                                );
                            }
                            Ok(())
                        }
                        (None, "xmlns", v) => {
                            if v.is_empty() {
                                // Undeclare the default namespace
                                nsd_vec.push(
                                    doc.new_namespace(
                                        NamespaceUri::try_from(v).unwrap(),
                                        None,
                                        false,
                                    )
                                    .map_err(|_| ParseError::MissingNameSpace)?,
                                );
                            } else {
                                state1.in_scope_namespaces.push(
                                    NamespaceDeclaration::new(
                                        None,
                                        NamespaceUri::try_from(v).unwrap(),
                                    )
                                    .map_err(|_| {
                                        ParseError::NotWellFormed(
                                            "unable to create namespace declaration".to_string(),
                                        )
                                    })?,
                                );
                                nsd_vec.push(
                                    doc.new_namespace(
                                        NamespaceUri::try_from(v).unwrap(),
                                        None,
                                        true,
                                    )
                                    .map_err(|_| ParseError::MissingNameSpace)?,
                                );
                            }
                            Ok(())
                        }
                        _ => Err(ParseError::NotWellFormed(String::from(
                            "unable to define namespace",
                        ))),
                    }
                })?;

            // Now process the normal attributes
            // TODO: use try_collect()
            let mut attr_vec: Vec<N> = vec![];
            let _ = attr_list
                .iter()
                .try_for_each(|((prefix, local_part), value)| {
                    match (prefix.as_deref(), local_part.as_str(), value.as_str()) {
                        // Sanity checks
                        // Check if the xml:space attribute is present and if so, does it have
                        // "Preserved" or "Default" as its value. We'll actually handle in a future release.
                        (Some("xml"), "space", "Default") => {
                            // Make the attribute
                            attr_vec.push(
                                doc.new_attribute(
                                    QName::new_from_parts(
                                        NcName::try_from("space").unwrap(),
                                        state1.in_scope_namespaces.namespace_uri(&Some(
                                            NamespacePrefix::try_from("xml").unwrap(),
                                        )),
                                    ),
                                    Rc::new(Value::from("Default")),
                                )
                                .map_err(|_| {
                                    ParseError::NotWellFormed(String::from(
                                        "unable to create attribute",
                                    ))
                                })?,
                            );
                            Ok(())
                        }
                        (Some("xml"), "space", "Preserve") => {
                            // Make the attribute
                            attr_vec.push(
                                doc.new_attribute(
                                    QName::new_from_parts(
                                        NcName::try_from("space").unwrap(),
                                        state1.in_scope_namespaces.namespace_uri(&Some(
                                            NamespacePrefix::try_from("xml").unwrap(),
                                        )),
                                    ),
                                    Rc::new(Value::from("Preserve")),
                                )
                                .map_err(|_| {
                                    ParseError::NotWellFormed(String::from(
                                        "unable to create attribute",
                                    ))
                                })?,
                            );
                            Ok(())
                        }
                        (Some("xml"), "space", _) => Err(ParseError::Validation {
                            row: state1.currentrow,
                            col: state1.currentcol,
                        }),
                        (Some("xml"), "id", val) => {
                            attr_vec.push(
                                doc.new_attribute(
                                    QName::new_from_parts(
                                        NcName::try_from("id").unwrap(),
                                        state1.in_scope_namespaces.namespace_uri(&Some(
                                            NamespacePrefix::try_from("xml").unwrap(),
                                        )),
                                    ),
                                    Rc::new(Value::from(ID::try_from(val).map_err(|_| {
                                        ParseError::IDError(String::from("not a valid ID value"))
                                    })?)),
                                )
                                .map_err(|_| {
                                    ParseError::NotWellFormed(String::from(
                                        "unable to create attribute",
                                    ))
                                })?,
                            );
                            Ok(())
                        }
                        (Some(p), lp, v) => {
                            // lookup namespace uri
                            attr_vec.push(
                                doc.new_attribute(
                                    QName::new_from_parts(
                                        NcName::try_from(lp).map_err(|_| {
                                            ParseError::NotWellFormed(String::from(
                                                "local part not valid",
                                            ))
                                        })?,
                                        Some(
                                            state1
                                                .in_scope_namespaces
                                                .namespace_uri(&Some(
                                                    NamespacePrefix::try_from(p).map_err(|_| {
                                                        ParseError::NotWellFormed(String::from(
                                                            "unable to resolve namespace prefix",
                                                        ))
                                                    })?,
                                                ))
                                                .ok_or(ParseError::NotWellFormed(String::from(
                                                    "no namespace declaration for prefix",
                                                )))?,
                                        ),
                                    ),
                                    Rc::new(Value::from(v)),
                                )
                                .map_err(|_| {
                                    ParseError::NotWellFormed(String::from(
                                        "unable to create attribute",
                                    ))
                                })?,
                            );
                            Ok(())
                        }
                        _ => {
                            // unprefixed name
                            // Make the attribute
                            attr_vec.push(
                                doc.new_attribute(
                                    QName::from_local_name(
                                        NcName::try_from(local_part.as_str()).map_err(|_| {
                                            ParseError::NotWellFormed(String::from(
                                                "local part not valid",
                                            ))
                                        })?,
                                    ),
                                    Rc::new(Value::from(value.as_str())),
                                )
                                .map_err(|_| {
                                    ParseError::NotWellFormed(String::from(
                                        "unable to create attribute",
                                    ))
                                })?,
                            );
                            Ok(())
                        }
                    }
                })?;

            Ok(((input1, state1), (attr_vec, nsd_vec)))
        }
        Err(err) => Err(err),
    }
}
// Attribute ::= Name '=' AttValue
fn attribute<'a, N: Node, L>() -> impl Fn(
    ParseInput<'a, N>,
    &mut StaticState<L>,
) -> Result<
    (ParseInput<'a, N>, ((Option<String>, String), String)),
    ParseError,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| match tuple6(
        whitespace1(),
        qualname_to_parts(),
        whitespace0(),
        tag("="),
        whitespace0(),
        attribute_value(),
    )((input, state), ss)
    {
        Ok(((input1, state1), (_, n, _, _, _, s))) => Ok(((input1, state1.clone()), (n, s))),
        Err(e) => Err(e),
    }
}

fn attribute_value<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| {
        let parse = alt2(
            delimited(
                tag("'"),
                many0(alt3(
                    map(chardata_unicode_codepoint(), |c| c.to_string()),
                    textreference(),
                    wellformed(
                        take_while(|c| c != '&' && c != '\''),
                        |c| !c.contains('<'),
                        "'<' not allowed in attribute value",
                    ),
                )),
                tag("'"),
            ),
            delimited(
                tag("\""),
                many0(alt3(
                    map(chardata_unicode_codepoint(), |c| c.to_string()),
                    textreference(),
                    wellformed(
                        take_while(|c| c != '&' && c != '\"'),
                        |c| !c.contains('<'),
                        "'<' not allowed in attribute value",
                    ),
                )),
                tag("\""),
            ),
        )((input, state), ss);

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
                let r = if state1.xmlversion.as_str() == "1.1" {
                    rn.concat()
                        .replace(['\u{85}', '\u{2028}', '\n', '\r', '\t', '\n'], " ")
                        .trim()
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .join(" ")
                } else {
                    rn.concat()
                        .replace(['\n', '\r', '\t', '\n'], " ")
                        .trim()
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .join(" ")
                };
                //NEL character cannot be in attributes.
                if state1.xmlversion.as_str() == "1.1" && r.find(|c| !is_char11(&c)).is_some() {
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
