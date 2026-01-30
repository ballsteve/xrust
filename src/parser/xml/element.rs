use crate::item::{Node, NodeType};
use crate::parser::combinators::alt::{alt2, alt4};
use crate::parser::combinators::many::many0nsreset;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::{tuple2, tuple5, tuple10};
use crate::parser::combinators::wellformed::wellformed;
use crate::parser::combinators::whitespace::whitespace0;
use crate::parser::common::is_ncnamechar;
use crate::parser::xml::attribute::attributes;
use crate::parser::xml::chardata::chardata;
use crate::parser::xml::misc::{comment, processing_instruction};
use crate::parser::xml::qname::qualname_to_parts;
use crate::parser::xml::reference::reference;
use crate::parser::{ParseError, ParseInput, StaticState};
use crate::value::{ID, IDREF, Value, ValueBuilder, ValueData};
use crate::xmldecl::{AttType, DefaultDecl};
use qualname::{NamespacePrefix, NamespaceUri, NcName, QName};
use std::rc::Rc;
use std::sync::LazyLock;

// static QNames
static XMLID: LazyLock<QName> = LazyLock::new(|| {
    QName::new_from_parts(
        NcName::try_from("id").unwrap(),
        Some(NamespaceUri::try_from("http://www.w3.org/XML/1998/namespace").unwrap()),
    )
});

// Element ::= EmptyElemTag | STag content ETag
pub(crate) fn element<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, N), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |input, ss| {
        match alt2(
            //Empty element
            map(
                tuple5(
                    tag("<"),
                    qualname_to_parts(),
                    attributes(), //many0(attribute),
                    whitespace0(),
                    tag("/>"),
                ),
                |(_, n, (at, ns), _, _)| ((), n.clone(), (at, ns), (), (), vec![], (), n, (), ()),
            ),
            //tagged element
            wellformed(
                tuple10(
                    tag("<"),
                    qualname_to_parts(),
                    attributes(), //many0(attribute),
                    whitespace0(),
                    tag(">"),
                    content(),
                    tag("</"),
                    qualname_to_parts(),
                    whitespace0(),
                    tag(">"),
                ),
                |(_, n, _a, _, _, _c, _, e, _, _)| n == e,
                "mismatched start and end tags",
            ),
        )(input, ss)
        {
            Err(err) => Err(err),
            Ok((
                (input1, state1),
                (_, (prefix, local_part), (av, namespaces), _, _, c, _, _, _, _),
            )) => {
                // Need to resolve element name to create element node,
                // then we can add namespace declarations.
                // Processing the attribute list updates the in-scope namespaces in the state
                let elementname = if let Some(p) = prefix.clone() {
                    // This is a prefixed name, so the prefix must resolve to a URI
                    // NB. Creating the prefix cannot fail, since it has already been parsed
                    if let Some(u) = state1
                        .in_scope_namespaces
                        .namespace_uri(&Some(NamespacePrefix::try_from(p.as_str()).unwrap()))
                    {
                        QName::new_from_parts(
                            NcName::try_from(local_part.as_str()).unwrap(), // creating NcName cannot fail, since we have already parsed it
                            Some(u),
                        )
                    } else {
                        return Err(ParseError::MissingNameSpace);
                    }
                } else {
                    // This is either an unprefixed name or a name in the default namespace, if one has been defined
                    if let Some(u) = state1.in_scope_namespaces.namespace_uri(&None) {
                        let lp = NcName::try_from(local_part.as_str()).unwrap();
                        QName::new_from_parts(
                            lp, // creating NcName cannot fail, since we have already parsed it
                            Some(u),
                        )
                    } else {
                        QName::from_local_name(NcName::try_from(local_part.as_str()).unwrap())
                    }
                };
                /* SRB: is this possible?
                if state1.xmlversion == "1.1"
                    && elementname.namespace_uri().to_string() == Some("".to_string())
                    && elementname.prefix().to_string().is_some()
                {
                    return Err(ParseError::MissingNameSpace);
                }*/
                let d = state1.doc.clone().unwrap();
                let mut e = d
                    .new_element(elementname.clone())
                    .expect("unable to create element");

                // Looking up the DTD, seeing if there are any attributes we should populate
                // Remember, DTDs don't have namespaces, you need to lookup based on prefix and local name!
                // We generate the attributes in two sweeps:
                // Once for attributes declared on the element and once for the DTD default attribute values.

                let attlist = state1
                    .dtd
                    .attlists
                    .get(&(prefix.clone(), local_part.clone()));

                match attlist {
                    None => {
                        // No Attribute DTD, just insert all attributes.
                        av.into_iter().try_for_each(|a| {
                            e.add_attribute(a).map_err(|_| {
                                ParseError::NotWellFormed(String::from("unable to add attribute"))
                            })
                        })?
                        /*for (attname, attval) in av.into_iter() {
                            //Ordinarily, you'll just treat attributes as CDATA and not normalize, however we need to check xml:id
                            let avalue: String;
                            let a: N;
                            if attname.prefix_to_string() == Some("xml".to_string())
                                && attname.localname_to_string() == "id"
                            {
                                av = attval.trim().replace("  ", " ");
                                if let Ok(avr) = ID::try_from(av) {
                                    a = d
                                        .new_attribute(Rc::new(attname), Rc::new(Value::from(avr)))
                                        .expect("unable to create attribute");
                                } else {
                                    return Err(ParseError::IDError("not an ID".to_string()));
                                }
                            } else {
                                av = attval;
                                a = d
                                    .new_attribute(Rc::new(attname), state1.get_value(&av))
                                    .expect("unable to create attribute");
                            };

                            e.add_attribute(a).expect("unable to add attribute")
                        }*/
                    }
                    Some(atts) => {
                        // Keep track of attributes that are created as defaults
                        let mut created_attrs = vec![];
                        for attnode in av.into_iter() {
                            if attnode.name().unwrap() == *XMLID {
                                let a = d
                                    .new_attribute(attnode.name().unwrap(), attnode.value())
                                    .expect("unable to create xml:id attribute");
                                if e.add_attribute(a).is_err() {
                                    return Err(ParseError::DuplicateAttribute(
                                        attnode.name().unwrap().to_string(),
                                    ));
                                }
                                created_attrs.push(attnode.name().clone());
                            } else {
                                let thisatprefix =
                                    attnode.name().unwrap().namespace_uri().and_then(|ns| {
                                        state1
                                            .in_scope_namespaces
                                            .prefix(&ns)
                                            .map(|p| p.to_string())
                                    });
                                let thisatlocalpart =
                                    attnode.name().unwrap().local_name().to_string();
                                match atts.get(&(thisatprefix, thisatlocalpart)) {
                                    // No DTD found, we just create the value
                                    None => {
                                        // TODO: this is duplicate code to the @xml:id case above.
                                        // Consolidate this with the above code.
                                        let a = d
                                            .new_attribute(attnode.name().unwrap(), attnode.value())
                                            .expect("unable to create attribute");
                                        if e.add_attribute(a).is_err() {
                                            return Err(ParseError::DuplicateAttribute(
                                                attnode.name().unwrap().to_string(),
                                            ));
                                        }
                                        created_attrs.push(attnode.name().clone());
                                    }
                                    Some((atttype, _, _)) => {
                                        //https://www.w3.org/TR/xml11/#AVNormalize
                                        let av = match atttype {
                                            AttType::CDATA => attnode.value().to_string(),
                                            _ => attnode
                                                .value()
                                                .to_string()
                                                .trim()
                                                .replace("  ", " "), // see attribute.rs for better attr value normalisation
                                        };
                                        // Assign IDs only if we are tracking.
                                        let v = match (atttype, state1.id_tracking) {
                                            (AttType::ID, true) => Rc::new(Value::from(
                                                ID::try_from(av.clone())
                                                    .map_err(|_| ParseError::MissingNameSpace)?,
                                            )),
                                            (AttType::IDREF, true) => Rc::new(Value::from(
                                                IDREF::try_from(av.clone())
                                                    .map_err(|_| ParseError::MissingNameSpace)?,
                                            )),
                                            (AttType::IDREFS, true) => Rc::new(
                                                ValueBuilder::new()
                                                    .value(ValueData::IDREFS(
                                                        av.clone().split(' ').try_fold(
                                                            vec![],
                                                            |mut acc, s| {
                                                                acc.push(
                                                                IDREF::try_from(s.to_string())
                                                                    .map_err(|_| {
                                                                        ParseError::MissingNameSpace
                                                                    })?,
                                                            );
                                                                Ok(acc)
                                                            },
                                                        )?,
                                                    ))
                                                    .build(),
                                            ),
                                            (_, _) => Rc::new(Value::from(av.clone())),
                                        };
                                        if atttype == &AttType::NMTOKENS && av.is_empty() {
                                            return Err(ParseError::NotWellFormed(
                                                "NMTOKENs must not be empty".to_string(),
                                            ));
                                        } else if atttype == &AttType::NMTOKENS {
                                            let names = av.split(' ');
                                            for name in names {
                                                let ch = name.chars();
                                                for cha in ch {
                                                    if !(is_ncnamechar(&cha) || cha == ':') {
                                                        return Err(ParseError::NotWellFormed(
                                                            String::from("Invalid NMTOKEN"),
                                                        ));
                                                    }
                                                }
                                            }
                                        }

                                        let a = d
                                            .new_attribute(attnode.name().unwrap(), v)
                                            .expect("unable to create attribute");
                                        if e.add_attribute(a).is_err() {
                                            return Err(ParseError::DuplicateAttribute(
                                                attnode.name().unwrap().to_string(),
                                            ));
                                        }
                                        created_attrs.push(attnode.name().clone());
                                    }
                                }
                            }
                        }
                        if state1.attr_defaults {
                            for ((attprefix, attlocalname), (atttype, defdecl, _)) in atts.iter() {
                                match defdecl {
                                    DefaultDecl::Default(s) | DefaultDecl::FIXED(s) => {
                                        let qn = attprefix.as_ref().map_or_else(
                                            || {
                                                QName::from_local_name(
                                                    NcName::try_from(attlocalname.as_str())
                                                        .unwrap(),
                                                )
                                            },
                                            |ap| {
                                                QName::new_from_parts(
                                                    NcName::try_from(attlocalname.as_str())
                                                        .unwrap(),
                                                    state1.in_scope_namespaces.namespace_uri(
                                                        &Some(
                                                            NamespacePrefix::try_from(ap.as_str())
                                                                .unwrap(),
                                                        ),
                                                    ), // TODO: return error if no namespace found
                                                )
                                            },
                                        );
                                        //https://www.w3.org/TR/xml11/#AVNormalize
                                        if !created_attrs.iter().any(|aqn| *aqn == Some(qn.clone()))
                                        {
                                            let attval = match atttype {
                                                AttType::CDATA => s.clone(),
                                                _ => s.trim().replace("  ", " "),
                                            };
                                            let a = d
                                                .new_attribute(
                                                    qn.clone(),
                                                    Rc::new(Value::from(attval)),
                                                )
                                                .expect("unable to create attribute");
                                            if e.add_attribute(a).is_err() {
                                                return Err(ParseError::DuplicateAttribute(
                                                    qn.to_string(),
                                                ));
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }

                //we've added the IDs and IDRefs, but we need to track all that.
                if state1.id_tracking {
                    for attribute in e.attribute_iter() {
                        if attribute.is_id() {
                            match ss.ids_read.insert(attribute.to_string()) {
                                true => {}
                                false => {
                                    //Value already existed!
                                    return Err(ParseError::IDError(String::from(
                                        "Duplicate ID found",
                                    )));
                                }
                            }
                        }
                        if attribute.is_idrefs() {
                            /*
                            If the IDRef matches a previously loaded ID, we're all good. If not, that ID
                            may exist further along, we'll make a note of it to check when we
                            have completely parsed the document.
                            */
                            if attribute.value().to_string().split_whitespace().count() == 0 {
                                return Err(ParseError::IDError(
                                    "IDREFs cannot be empty".to_string(),
                                ));
                            } else {
                                for idref in attribute.value().to_string().split_whitespace() {
                                    match ss.ids_read.get(idref) {
                                        Some(_) => {}
                                        None => {
                                            ss.ids_pending.insert(idref.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                namespaces
                    .iter()
                    .for_each(|b| e.add_namespace(b.clone()).expect("unable to add namespace"));
                // Add child nodes
                c.iter().for_each(|d| {
                    e.push(d.clone()).expect("unable to add node");
                });
                Ok(((input1, state1.clone()), e))
            }
        }
    }
}

// content ::= CharData? ((element | Reference | CDSect | PI | Comment) CharData?)*
pub(crate) fn content<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, Vec<N>), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| match tuple2(
        opt(chardata()),
        many0nsreset(tuple2(
            alt4(
                map(processing_instruction(), |e| vec![e]),
                map(comment(), |e| vec![e]),
                map(element(), |e| vec![e]),
                reference(),
            ),
            opt(chardata()),
        )),
    )((input, state.clone()), ss)
    {
        Ok((state1, (c, v))) => {
            let mut new: Vec<N> = Vec::new();
            let mut notex: Vec<String> = Vec::new();
            if let Some(s) = c {
                notex.push(s)
            }
            if !v.is_empty() {
                for (w, d) in v {
                    for x in w {
                        match x.node_type() {
                            NodeType::Text => notex.push(x.to_string()),
                            _ => {
                                if !notex.is_empty() {
                                    new.push(
                                        state
                                            .doc
                                            .clone()
                                            .unwrap()
                                            .new_text(Rc::new(Value::from(notex.concat())))
                                            .expect("unable to create text node"),
                                    );
                                    notex.clear();
                                }
                                new.push(x);
                            }
                        }
                    }
                    if let Some(s) = d {
                        notex.push(s)
                    }
                }
            }
            if !notex.is_empty() {
                new.push(
                    state
                        .doc
                        .clone()
                        .unwrap()
                        .new_text(Rc::new(Value::from(notex.concat())))
                        .expect("unable to create text node"),
                );
            }
            Ok((state1, new))
        }

        Err(e) => Err(e),
    }
}
