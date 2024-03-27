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
use crate::parser::xml::qname::qualname;
use crate::parser::xml::reference::textreference;
use crate::parser::{ParseError, ParseInput};
use crate::value::Value;
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) fn attributes<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, Vec<N>), ParseError> {
    move |input| match many0(attribute())(input) {
        Ok(((input1, mut state1), nodes)) => {
            let n: HashMap<String, String> = HashMap::new();
            let mut namespaces = state1.namespace.last().unwrap_or(&n).clone();
            for node in nodes.clone() {
                //Return error if someone attempts to redefine namespaces.
                if (node.name().get_prefix() == Some("xmlns".to_string()))
                    && (node.name().get_localname() == *"xmlns")
                {
                    return Err(ParseError::NotWellFormed(String::from("cannot redefine namespace")));
                }
                //xml prefix must always be set to http://www.w3.org/XML/1998/namespace
                if (node.name().get_prefix() == Some("xmlns".to_string()))
                    && (node.name().get_localname() == *"xml")
                    && (node.to_string() != *"http://www.w3.org/XML/1998/namespace")
                {
                    return Err(ParseError::NotWellFormed(String::from("xml namespace URI must be http://www.w3.org/XML/1998/namespace")));
                }
                // http://www.w3.org/XML/1998/namespace must always be bound to xml
                if (node.name().get_prefix() == Some("xmlns".to_string()))
                    && (node.name().get_localname() != *"xml")
                    && (node.to_string() == *"http://www.w3.org/XML/1998/namespace")
                {
                    return Err(ParseError::NotWellFormed(String::from("XML namespace must be bound to xml prefix")));
                }
                // http://www.w3.org/2000/xmlns/ must always be bound to xmlns
                if (node.name().get_prefix() == Some("xmlns".to_string()))
                    && (node.name().get_localname() != *"xmlns")
                    && (node.to_string() == *"http://www.w3.org/2000/xmlns/")
                {
                    return Err(ParseError::NotWellFormed(String::from("XMLNS namespace must be bound to xmlns prefix")));
                }
                // Default namespace cannot be http://www.w3.org/XML/1998/namespace
                // Default namespace cannot be http://www.w3.org/2000/xmlns/
                if (node.name().get_prefix().is_none())
                    && (node.name().get_localname() == *"xmlns")
                    && (node.to_string() == *"http://www.w3.org/XML/1998/namespace"
                        || node.to_string() == *"http://www.w3.org/2000/xmlns/")
                {
                    return Err(ParseError::NotWellFormed(String::from("invalid default namespace")));
                }

                // XML 1.0 documents cannot redefine an alias to ""
                if (node.name().get_prefix() == Some("xmlns".to_string()))
                    && (node.name().get_localname() != "")
                    && (node.to_string() == *"")
                    && state1.xmlversion == *"1.0"
                {
                    return Err(ParseError::NotWellFormed(String::from("cannot redefine alias to empty")));
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
            // SRB: TODO: partition the nodes vector based on whether the attribute has a prefix (and is not a namespace declaration)
            // Then loop through the prefixed attributes after the namespaces have been processed
            let mut resnodes = vec![];
            let mut resnodenames = vec![];
            for node in nodes {
                if node.name().get_prefix() != Some("xmlns".to_string())
                    && node.name().get_localname() != *"xmlns"
                {
                    //if let Some(ns) = node.name().get_prefix() {
                    //    if ns == *"xml" {
                    //        node.set_nsuri("http://www.w3.org/XML/1998/namespace".to_string())
                    //    } else {
                    //        match namespaces.get(&*ns) {
                    //            None => return Err(ParseError::MissingNameSpace),
                    //            Some(nsuri) => node.set_nsuri(nsuri.clone()),
                    //        }
                    //    }
                    //}
                    /* Why not just use resnodes.contains()  ? I don't know how to do partial matching */
                    if resnodenames
                        .contains(&(node.name().get_nsuri(), node.name().get_localname()))
                    {
                        return Err(ParseError::NotWellFormed(String::from("missing namespace")));
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
fn attribute<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, N), ParseError> {
    move |(input, state)| match tuple6(
            whitespace1(),
            qualname(),
            whitespace0(),
            tag("="),
            whitespace0(),
            attribute_value(),
        )((input, state)) {
        Ok(((input1, state1), (_, n, _, _, _, s))) => {
            Ok(((input1, state1.clone()), state1.doc.unwrap().new_attribute(n, Rc::new(Value::String(s))).expect("unable to create attribute")))
        }
        Err(e) => Err(e)
    }
}

fn attribute_value<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, String), ParseError> {
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
