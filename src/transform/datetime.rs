//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use std::rc::Rc;

#[allow(unused_imports)]
use chrono::{DateTime, Datelike, FixedOffset, Local, Timelike};
use url::Url;

use crate::item::{Item, Node, Sequence, SequenceTrait};
use crate::parser::datetime::parse as picture_parse;
use crate::transform::context::{Context, StaticContext};
use crate::transform::Transform;
use crate::value::{Value, ValueData};
use crate::xdmerror::{Error, ErrorKind};

/// XPath current-date-time function.
pub fn current_date_time<N: Node>(_ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::from(Local::now())))])
}

/// XPath current-date function.
pub fn current_date<N: Node>(_ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::new_date(
        Local::now().date_naive(),
    )))])
}

/// XPath current-time function.
pub fn current_time<N: Node>(_ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::new_time(Local::now())))])
}

/// XPath format-date-time function.
/// NB. language, calendar, and place are not implemented.
pub fn format_date_time<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    value: &Transform<N>,
    picture: &Transform<N>,
    _language: &Option<Box<Transform<N>>>,
    _calendar: &Option<Box<Transform<N>>>,
    _place: &Option<Box<Transform<N>>>,
) -> Result<Sequence<N>, Error> {
    let dt = ctxt.dispatch(stctxt, value)?;
    let pic = picture_parse::<N>(&ctxt.dispatch(stctxt, picture)?.to_string())?;
    match dt.len() {
        0 => Ok(vec![]), // Empty value returns empty sequence
        1 => {
            match &dt[0] {
                Item::Value(d) => match d.value {
                    ValueData::DateTime(i) => Ok(vec![Item::Value(Rc::new(Value::from(
                        i.format(&pic).to_string(),
                    )))]),
                    ValueData::String(ref s) => {
                        // Try and coerce into a DateTime value
                        match DateTime::<FixedOffset>::parse_from_rfc3339(s.as_str()) {
                            Ok(j) => Ok(vec![Item::Value(Rc::new(Value::from(
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
                },
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
pub fn format_date<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    value: &Transform<N>,
    picture: &Transform<N>,
    _language: &Option<Box<Transform<N>>>,
    _calendar: &Option<Box<Transform<N>>>,
    _place: &Option<Box<Transform<N>>>,
) -> Result<Sequence<N>, Error> {
    let dt = ctxt.dispatch(stctxt, value)?;
    let pic = picture_parse::<N>(&ctxt.dispatch(stctxt, picture)?.to_string())?;
    match dt.len() {
        0 => Ok(vec![]), // Empty value returns empty sequence
        1 => {
            match &dt[0] {
                Item::Value(d) => match d.value {
                    ValueData::Date(i) => Ok(vec![Item::Value(Rc::new(Value::from(
                        i.format(&pic).to_string(),
                    )))]),
                    ValueData::String(ref s) => {
                        // Try and coerce into a DateTime value
                        let a = format!("{}T00:00:00Z", s);
                        match DateTime::<FixedOffset>::parse_from_rfc3339(a.as_str()) {
                            Ok(j) => Ok(vec![Item::Value(Rc::new(Value::from(
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
                },
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
pub fn format_time<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    value: &Transform<N>,
    picture: &Transform<N>,
    _language: &Option<Box<Transform<N>>>,
    _calendar: &Option<Box<Transform<N>>>,
    _place: &Option<Box<Transform<N>>>,
) -> Result<Sequence<N>, Error> {
    let dt = ctxt.dispatch(stctxt, value)?;
    let pic = picture_parse::<N>(&ctxt.dispatch(stctxt, picture)?.to_string())?;
    match dt.len() {
        0 => Ok(vec![]), // Empty value returns empty sequence
        1 => {
            match &dt[0] {
                Item::Value(d) => match d.value {
                    ValueData::Time(i) => Ok(vec![Item::Value(Rc::new(Value::from(
                        i.format(&pic).to_string(),
                    )))]),
                    ValueData::String(ref s) => {
                        // Try and coerce into a DateTime value
                        let a = format!("1900-01-01T{}Z", s);
                        match DateTime::<FixedOffset>::parse_from_rfc3339(a.as_str()) {
                            Ok(j) => Ok(vec![Item::Value(Rc::new(Value::from(
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
                },
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
