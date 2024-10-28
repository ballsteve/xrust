//! # Callables
//! Sequence constructors that are invoked by stylesheet code, such as named templates and functions.
//! The difference between them is that named templates have named parameters,
//! whereas functions have positional parameters.

// TODO: tunneling parameters

use crate::item::Node;
use crate::qname::QualifiedName;
use crate::transform::context::StaticContext;
use crate::transform::{NamespaceMap, Transform};
use crate::{Context, Error, ErrorKind, Sequence};
use std::collections::HashMap;
use url::Url;

/// A user-defined callable object, such as a function or named template.
#[derive(Clone, Debug)]
pub struct Callable<N: Node>
{
    pub(crate) body: Transform<N>,
    pub(crate) parameters: FormalParameters<N>,
    // TODO: return type
}

impl<N: Node> Callable<N>
{
    pub fn new(body: Transform<N>, parameters: FormalParameters<N>) -> Self {
        Callable { body, parameters }
    }
}

/// A custom extension function.
pub struct ExtFunction<N: Node>
{
    pub(crate) callback: Box<dyn FnMut(&Context<N>) -> Result<Sequence<N>, Error>>,
    pub(crate) parameters: FormalParameters<N>,
}

impl<N: Node> ExtFunction<N>
{
    pub fn new(
        callback: Box<dyn FnMut(&Context<N>) -> Result<Sequence<N>, Error>>,
        parameters: FormalParameters<N>
    ) -> Self {
        ExtFunction { callback, parameters }
    }
}

// TODO: parameter type ("as" attribute)
#[derive(Clone, Debug)]
pub enum FormalParameters<N: Node> {
    Named(Vec<(QualifiedName, Option<Transform<N>>)>), // parameter name, default value
    Positional(Vec<QualifiedName>),
}
#[derive(Clone, Debug)]
pub enum ActualParameters<N: Node> {
    Named(Vec<(QualifiedName, Transform<N>)>), // parameter name, value
    Positional(Vec<Transform<N>>),
}

/// Invoke a callable component
pub(crate) fn invoke<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    qn: &QualifiedName,
    a: &ActualParameters<N>,
    ns: &NamespaceMap,
) -> Result<Sequence<N>, Error> {
    let mut qnr = (*qn).clone();
    qnr.resolve(|p| {
        ns.get(&p).map_or(
            Err(Error::new(
                ErrorKind::DynamicAbsent,
                "no namespace for prefix",
            )),
            |r| Ok(r.clone()),
        )
    })?;
    // Callable transforms have precedence over application callbacks
    match ctxt.callables.get(&qnr) {
        Some(t) => {
            let newctxt = make_new_context(ctxt, stctxt, &t.parameters, a)?;
            newctxt.dispatch(stctxt, &t.body)
        }
        None => {
            let fp = stctxt.ext_formals.get(&qnr).unwrap().clone();
            let newctxt = make_new_context(ctxt, stctxt, &fp, a)?;
            if let Some(e) = stctxt.extensions.get_mut(&qnr) {
                if let Some(g) = e {
                    g(&newctxt)
                } else {
                    Err(Error::new(
                        ErrorKind::TypeError,
                        format!("extension function \"{}\" is built-in", qn),
                    ))
                }
            } else {
                Err(Error::new(
                    ErrorKind::Unknown,
                    format!("unknown extension function \"{}\"", qn),
                ))
            }
        },
    }
}

fn make_new_context<N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    p: &FormalParameters<N>,
    a: &ActualParameters<N>,
) -> Result<Context<N>, Error> {
    match &p {
        FormalParameters::Named(v) => {
            let mut newctxt = ctxt.clone();
            // Put the actual parameters in a HashMap for easy access
            let mut actuals = HashMap::new();
            if let ActualParameters::Named(av) = a {
                av.iter().try_for_each(|(a_name, a_value)| {
                    actuals.insert(a_name, ctxt.dispatch(stctxt, a_value)?);
                    Ok(())
                })?
            } else {
                return Err(Error::new(ErrorKind::TypeError, "argument mismatch"));
            }
            // Match each actual parameter to a formal parameter by name
            v.iter().try_for_each(|(name, dflt)| {
                match actuals.get(name) {
                    Some(val) => {
                        newctxt.var_push(name.to_string(), val.clone());
                        Ok(())
                    }
                    None => {
                        // Use default value
                        if let Some(d) = dflt {
                            newctxt.var_push(name.to_string(), ctxt.dispatch(stctxt, d)?)
                        } else {
                            newctxt.var_push(name.to_string(), vec![])
                        }
                        Ok(())
                    }
                }
            })?;
            Ok(newctxt)
        }
        FormalParameters::Positional(v) => {
            if let ActualParameters::Positional(av) = a {
                // Make sure number of parameters are equal, then set up variables by position
                if v.len() == av.len() {
                    let mut newctxt = ctxt.clone();
                    v.iter().zip(av.iter()).try_for_each(|(qn, t)| {
                        newctxt.var_push(qn.to_string(), ctxt.dispatch(stctxt, t)?);
                        Ok(())
                    })?;
                    Ok(newctxt)
                } else {
                    Err(Error::new(ErrorKind::TypeError, "argument mismatch"))
                }
            } else {
                Err(Error::new(ErrorKind::TypeError, "argument mismatch"))
            }
        }
    }
}
