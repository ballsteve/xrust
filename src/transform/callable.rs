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
use std::rc::Rc;
use std::collections::HashMap;
use url::Url;

#[derive(Clone, Debug)]
pub enum CallType<N: Node> {
    Transform(Transform<N>),
    ApplicationCallback(Rc<QualifiedName>),
}
#[derive(Clone, Debug)]
pub struct Callable<N: Node> {
    pub(crate) body: CallType<N>,
    pub(crate) parameters: FormalParameters<N>,
    // TODO: return type
}

impl<N: Node> Callable<N> {
    pub fn new(body: CallType<N>, parameters: FormalParameters<N>) -> Self {
        Callable { body, parameters }
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
    J: FnMut(&Vec<Transform<N>>) -> Result<Sequence<N>, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H, J>,
    qn: &QualifiedName,
    a: &ActualParameters<N>,
    ns: &NamespaceMap,
) -> Result<Sequence<N>, Error> {
    let mut qnr = qn.clone();
    qnr.resolve(|p| ns.get(&p).map_or(
        Err(Error::new(ErrorKind::DynamicAbsent, "no namespace for prefix")),
        |r| Ok(r.clone())
    ))?;
    match ctxt.callables.get(&qnr) {
        Some(t) => {
            match &t.parameters {
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
                    newctxt.dispatch(stctxt, &t.body)
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
                            match &t.body {
                                CallType::Transform(u) => newctxt.dispatch(stctxt, &u),
                                CallType::ApplicationCallback(qn) => {
                                    stctxt.extensions.get(&qn).map_or(
                                        Err(Error::new(ErrorKind::StaticAbsent, format!("unable to find extension function \"{}\"", qn.to_string()))),
                                        |f| {
                                            if let Some(g) = f {
                                                g(&vec![]) // we don't need this coz the actual arguments are now variables
                                            } else {
                                                Err(Error::new(ErrorKind::TypeError, format!("function \"{}\" is not an extension function", qn.to_string())))
                                            }
                                        }
                                    )
                                }
                            }
                        } else {
                            Err(Error::new(ErrorKind::TypeError, "argument mismatch"))
                        }
                    } else {
                        Err(Error::new(ErrorKind::TypeError, "argument mismatch"))
                    }
                }
            }
        }
        None => Err(Error::new(
            ErrorKind::Unknown,
            format!("unknown callable \"{}\"", qn),
        )),
    }
}
