//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use std::rc::Rc;

#[allow(unused_imports)]
use chrono::{DateTime, Datelike, FixedOffset, Local, Timelike};

use crate::xdmerror::{Error, ErrorKind};
use crate::parsepicture::parse as picture_parse;
use crate::value::Value;
use crate::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use crate::transform::Transform;
use crate::transform::context::{Context, ContextBuilder};

/// XPath current-date-time function.
pub fn current_date_time<N: Node>(
    _ctxt: &Context<N>,
) -> Result<Sequence<N>, Error> {
    Ok(vec![Rc::new(Item::Value(Value::DateTime(Local::now())))])
}

/// XPath current-date function.
pub fn current_date<N: Node>(
    _ctxt: &Context<N>,
) -> Result<Sequence<N>, Error> {
        Ok(vec![Rc::new(Item::Value(Value::Date(
            Local::now().date_naive(),
        )))])
}

/// XPath current-time function.
pub fn current_time<N: Node>(
    _ctxt: &Context<N>,
) -> Result<Sequence<N>, Error> {
    Ok(vec![Rc::new(Item::Value(Value::Time(Local::now())))])
}

/// XPath format-date-time function.
/// NB. language, calendar, and place are not implemented.
pub fn format_date_time<N: Node>(
    ctxt: &Context<N>,
    value: &Transform<N>,
    picture: &Transform<N>,
    _language: Option<&Transform<N>>,
    _calendar: Option<&Transform<N>>,
    _place: Option<&Transform<N>>,
) -> Result<Sequence<N>, Error> {
        let dt = ctxt.dispatch(value)?;
        let pic = picture_parse(&ctxt.dispatch(picture)?.to_string())?;
        match dt.len() {
            0 => Ok(vec![]), // Empty value returns empty sequence
            1 => {
                match *dt[0] {
                    Item::Value(Value::DateTime(i)) => Ok(vec![Rc::new(Item::Value(
                        Value::String(i.format(&pic).to_string()),
                    ))]),
                    Item::Value(Value::String(ref s)) => {
                        // Try and coerce into a DateTime value
                        match DateTime::<FixedOffset>::parse_from_rfc3339(s.as_str()) {
                            Ok(j) => Ok(vec![Rc::new(Item::Value(Value::String(
                                j.format(&pic).to_string(),
                            )))]),
                            _ => Err(Error::new(
                                ErrorKind::TypeError,
                                String::from("unable to determine date value"),
                            )),
                        }
                    }
                    _ => Err(Error::new(
                        ErrorKind::TypeError,
                        String::from("not a dateTime value"),
                    )),
                }
            }
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("not a singleton sequence"),
            )),
        }
}

/// XPath format-date function.
/// NB. language, calendar, and place are not implemented.
pub fn format_date<N: Node>(
    ctxt: &Context<N>,
    value: &Transform<N>,
    picture: &Transform<N>,
    _language: Option<&Transform<N>>,
    _calendar: Option<&Transform<N>>,
    _place: Option<&Transform<N>>,
) -> Result<Sequence<N>, Error> {
        let dt = ctxt.dispatch(value)?;
        let pic = picture_parse(&ctxt.dispatch(picture)?.to_string())?;
        match dt.len() {
            0 => Ok(vec![]), // Empty value returns empty sequence
            1 => {
                match *dt[0] {
                    Item::Value(Value::Date(i)) => Ok(vec![Rc::new(Item::Value(Value::String(
                        i.format(&pic).to_string(),
                    )))]),
                    Item::Value(Value::String(ref s)) => {
                        // Try and coerce into a DateTime value
                        let a = format!("{}T00:00:00Z", s);
                        match DateTime::<FixedOffset>::parse_from_rfc3339(a.as_str()) {
                            Ok(j) => Ok(vec![Rc::new(Item::Value(Value::String(
                                j.date_naive().format(&pic).to_string(),
                            )))]),
                            _ => Err(Error::new(
                                ErrorKind::TypeError,
                                String::from("unable to determine date value"),
                            )),
                        }
                    }
                    _ => Err(Error::new(
                        ErrorKind::TypeError,
                        String::from("not a date value"),
                    )),
                }
            }
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("not a singleton sequence"),
            )),
        }
}

/// XPath format-time function.
/// NB. language, calendar, and place are not implemented.
pub fn format_time<N: Node>(
    ctxt: &Context<N>,
    value: &Transform<N>,
    picture: &Transform<N>,
    _language: Option<&Transform<N>>,
    _calendar: Option<&Transform<N>>,
    _place: Option<&Transform<N>>,
) -> Result<Sequence<N>, Error> {
        let dt = ctxt.dispatch(value)?;
        let pic = picture_parse(&ctxt.dispatch(picture)?.to_string())?;
        match dt.len() {
            0 => Ok(vec![]), // Empty value returns empty sequence
            1 => {
                match *dt[0] {
                    Item::Value(Value::Time(i)) => Ok(vec![Rc::new(Item::Value(Value::String(
                        i.format(&pic).to_string(),
                    )))]),
                    Item::Value(Value::String(ref s)) => {
                        // Try and coerce into a DateTime value
                        let a = format!("1900-01-01T{}Z", s);
                        match DateTime::<FixedOffset>::parse_from_rfc3339(a.as_str()) {
                            Ok(j) => Ok(vec![Rc::new(Item::Value(Value::String(
                                j.format(&pic).to_string(),
                            )))]),
                            _ => Err(Error::new(
                                ErrorKind::TypeError,
                                String::from("unable to determine time value"),
                            )),
                        }
                    }
                    _ => Err(Error::new(
                        ErrorKind::TypeError,
                        String::from("not a time value"),
                    )),
                }
            }
            _ => Err(Error::new(
                ErrorKind::TypeError,
                String::from("not a singleton sequence"),
            )),
        }
}
