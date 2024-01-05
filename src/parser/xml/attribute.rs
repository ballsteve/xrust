use crate::item::NodeType;
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
use crate::parser::{ParseError, ParseInput, ParseResult};
use crate::trees::intmuttree::{NodeBuilder, RNode};
use crate::{Node, Value};
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) fn attributes() -> impl Fn(ParseInput) -> ParseResult<Vec<RNode>> {
    move |input| match many0(attribute())(input) {
        Ok(((input1, mut state1), nodes)) => {
            let n: HashMap<String, String> = HashMap::new();
            let mut namespaces = state1.namespace.last().unwrap_or(&n).clone();
            for node in nodes.clone() {
                //Return error if someone attempts to redefine namespaces.
                if (node.name().get_prefix() == Some("xmlns".to_string()))
                    && (node.name().get_localname() == *"xmlns")
                {
                    return Err(ParseError::NotWellFormed);
                }
                //xml prefix must always be set to http://www.w3.org/XML/1998/namespace
                if (node.name().get_prefix() == Some("xmlns".to_string()))
                    && (node.name().get_localname() == *"xml")
                    && (node.to_string() != *"http://www.w3.org/XML/1998/namespace")
                {
                    return Err(ParseError::NotWellFormed);
                }
                // http://www.w3.org/XML/1998/namespace must always be bound to xml
                if (node.name().get_prefix() == Some("xmlns".to_string()))
                    && (node.name().get_localname() != *"xml")
                    && (node.to_string() == *"http://www.w3.org/XML/1998/namespace")
                {
                    return Err(ParseError::NotWellFormed);
                }
                // http://www.w3.org/2000/xmlns/ must always be bound to xmlns
                if (node.name().get_prefix() == Some("xmlns".to_string()))
                    && (node.name().get_localname() != *"xmlns")
                    && (node.to_string() == *"http://www.w3.org/2000/xmlns/")
                {
                    return Err(ParseError::NotWellFormed);
                }
                // Default namespace cannot be http://www.w3.org/XML/1998/namespace
                // Default namespace cannot be http://www.w3.org/2000/xmlns/
                if (node.name().get_prefix().is_none())
                    && (node.name().get_localname() == *"xmlns")
                    && (node.to_string() == *"http://www.w3.org/XML/1998/namespace"
                        || node.to_string() == *"http://www.w3.org/2000/xmlns/")
                {
                    return Err(ParseError::NotWellFormed);
                }

                // XML 1.0 documents cannot redefine an alias to ""
                if (node.name().get_prefix() == Some("xmlns".to_string()))
                    && (node.name().get_localname() != "")
                    && (node.to_string() == *"")
                    && state1.xmlversion == *"1.0"
                {
                    return Err(ParseError::NotWellFormed);
                }

                if (node.name().get_prefix() == Some("xmlns".to_string()))
                    || (node.name().get_localname() == *"xmlns")
                {
                    namespaces.insert(node.name().get_localname(), node.to_string());
                };

                //Check if the xml:space attribute is present and if so, does it have
                //"Preserved" or "Default" as its value. We'll actually handle in a future release.
                if node.name().get_prefix() == Some("xml".to_string())
                    && node.name().get_localname() == *"space"
                    && !(node.to_string() == "Default" || node.to_string() == "Preserve")
                {
                    return Err(ParseError::Validation {
                        row: state1.currentrow,
                        col: state1.currentcol,
                    });
                }
            }
            state1.namespace.push(namespaces.clone());
            //Why loop through the nodes a second time? XML attributes are not in any order, so the
            //namespace declaration can happen after the attribute if it has a namespace prefix.
            let mut resnodes = vec![];
            let mut resnodenames = vec![];
            for node in nodes {
                if node.name().get_prefix() != Some("xmlns".to_string())
                    && node.name().get_localname() != *"xmlns"
                {
                    if let Some(ns) = node.name().get_prefix() {
                        if ns == *"xml" {
                            node.set_nsuri("http://www.w3.org/XML/1998/namespace".to_string())
                        } else {
                            match namespaces.get(&*ns) {
                                None => return Err(ParseError::MissingNameSpace),
                                Some(nsuri) => node.set_nsuri(nsuri.clone()),
                            }
                        }
                    }
                    /* Why not just use resnodes.contains()  ? I don't know how to do partial matching */
                    if resnodenames
                        .contains(&(node.name().get_nsuri(), node.name().get_localname()))
                    {
                        return Err(ParseError::NotWellFormed);
                    } else {
                        resnodenames.push((node.name().get_nsuri(), node.name().get_localname()));
                        resnodes.push(node);
                    }
                }
            }
            Ok(((input1, state1), resnodes))
        }
        Err(err) => Err(err),
    }
}
// Attribute ::= Name '=' AttValue
fn attribute() -> impl Fn(ParseInput) -> ParseResult<RNode> {
    map(
        tuple6(
            whitespace1(),
            qualname(),
            whitespace0(),
            tag("="),
            whitespace0(),
            attribute_value(),
        ),
        |(_, n, _, _, _, s)| {
            NodeBuilder::new(NodeType::Attribute)
                .name(n)
                .value(Rc::new(Value::String(s)))
                .build()
        },
    )
}

fn attribute_value() -> impl Fn(ParseInput) -> ParseResult<String> {
    move |(input, state)| {
        let parse = alt2(
            delimited(
                tag("'"),
                many0(alt3(
                    map(
                        wellformed(chardata_unicode_codepoint(), |c| c != &'<'),
                        |c| c.to_string(),
                    ),
                    textreference(),
                    wellformed(take_while(|c| c != '&' && c != '\''), |c| !c.contains('<')),
                )),
                tag("'"),
            ),
            delimited(
                tag("\""),
                many0(alt3(
                    map(
                        wellformed(chardata_unicode_codepoint(), |c| c != &'<'),
                        |c| c.to_string(),
                    ),
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
                let r = rn
                    .concat()
                    .replace(['\n', '\r', '\t', '\n'], " ")
                    .trim()
                    .to_string();
                //NEL character cannot be in attributes.
                if state1.xmlversion == "1.1" && r.find(|c| !is_char11(&c)).is_some() {
                    Err(ParseError::NotWellFormed)
                } else if r.find(|c| !is_char10(&c)).is_some() {
                    Err(ParseError::NotWellFormed)
                } else if r.contains('\u{0085}') {
                    Err(ParseError::NotWellFormed)
                } else {
                    Ok(((input1, state1), r))
                }
            }
        }
    }
}
