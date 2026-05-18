//! Support for security policies.
//!
//! A security policy allows a module to limit, or constrain, access to a resource.
//! The resource is named, using a [QName], and the module will call into the in-force policy to retrieve the limitation set on the resource.
//! The limitation is returned as a [SecurityResult].
//! The module may provide [ActualParameters] to the feature, refer to the module's documentation for details.
//!
//! ```rust
//! # use std::rc::Rc;
//! use xrust::security::{SecurityResult, Policy, Feature};
//! use xrust::item::Item;
//! use xrust::value::Value;
//! use xrust::transform::Transform;
//! use xrust::transform::callable::ActualParameters;
//! use qualname::{QName, NcName};
//!
//! fn get_feature<N: Node>(policy: &Policy<N>) -> Result<Option<String>, Error> {
//!    match policy.get(
//!       QName::from_local_name(NcName::try_from("my_security_feature").unwrap()),
//!       ActualParameters::Named(vec![
//!          (QName::from_local_name(NcName::try_from("input").unwrap()),
//!           Transform::Literal(Item::Value(Rc::new(Value::from("value")))))
//!       ])
//!    )? {
//!        SecurityResult::NotPermitted => Err(Error::new(ErrorKind::NotPermitted, "access denied")),
//!        SecurityResult::Permitted(None) => Ok(None),
//!        SecurityResult::Permitted(Some(v)) => Some(v),
//!    }
//! }
//! ```
//!
//! If a policy does not define a limit or constraint for a resource,
//! then the module will define a default value. The module should set a default that has minimal security implications for the application.
//! Most likely this will be to deny access to the resource.
//!
//! Security policies are named. Many named policies can be loaded into the system.
//! The application can nominate which policy it wants to be in force ("activated").
//!
//! Resource constraints may be specified either as an absolute value or with a template.
//! Templates use the same syntax as XSLT templates.
//!
//! In this example, a security policy is created with the feature set to "permitted with no limits".
//!
//! ```rust
//! use xrust::security::{Feature, Policy};
//! use xrust::trees::smite::RNode;
//! use qualname::{QName, NcName};
//!
//! let mut policy: Policy<RNode> = Policy::new(QName::from_local_name(
//!    NcName::try_from("test_policy").unwrap(),
//! ));
//! policy.add(
//!    QName::from_local_name(
//!        NcName::try_from("my_security_feature").unwrap(),
//!    ),
//!    Feature::Permitted(None),
//! );
//! ```

use std::collections::HashMap;

use crate::item::{Node, SequenceTrait};
use crate::transform::Transform;
use crate::transform::callable::ActualParameters;
use crate::transform::context::{Context, StaticContextBuilder};
use crate::xdmerror::{Error, ErrorKind};
use qualname::QName;

/// The result of determining the limitation or constraint for a security feature.
/// Permitted means that the application is allowed to access the resource.
/// The contained value is a limit on the usage of the resource.
/// If it is None then there is no limit on resource usage, or the module may impose a default limit.
/// NotPermitted means that the application is not allowed to access the resource at all, or the module may impose a default limit.
#[derive(Clone, Debug, PartialEq)]
pub enum SecurityResult {
    Permitted(Option<String>),
    NotPermitted,
}

/// All of the security policies available for use, indexed by name.
/// One of these policies may be in force (or "active").
#[derive(Clone, Debug)]
pub struct SecurityPolicies<N: Node> {
    policies: HashMap<QName, Policy<N>>,
    in_force: Option<QName>,
}

impl<N: Node> SecurityPolicies<N> {
    /// Create a new set of security policies.
    pub fn new() -> Self {
        Self {
            policies: HashMap::new(),
            in_force: None,
        }
    }
    /// Add a new security policy
    pub fn push(&mut self, policy: Policy<N>) {
        self.policies.insert(policy.name.clone(), policy);
    }
    /// Look up a security policy by name
    pub fn find(&self, name: QName) -> Option<&Policy<N>> {
        self.policies.get(&name)
    }
    /// Make the named security policy the "in force" (or "active") policy.
    pub fn activate(&mut self, name: &QName) -> Option<&Policy<N>> {
        self.policies.get_key_value(name).map(|(_k, v)| {
            self.in_force = Some(name.clone());
            v
        })
    }
    /// Determine whether a feature, in the in-force policy, is permitted.
    /// All parameters must be named, i.e. positional parameters are ignored.
    pub fn get(&self, f: &QName, a: ActualParameters<N>) -> Result<SecurityResult, Error> {
        // If there is no in-force security policy then all features are not permitted
        if self.in_force.is_none() {
            return Ok(SecurityResult::NotPermitted);
        }

        // Does the in-force security policy have the requested feature?
        // If not then it is not permitted
        if let Some(p) = self.policies.get(&self.in_force.as_ref().unwrap()) {
            p.get(f, a)
        } else {
            Ok(SecurityResult::NotPermitted)
        }
    }
}

/// A security policy. Security policies contain a number of security [Feature]s.
#[derive(Clone, Debug)]
pub struct Policy<N: Node> {
    name: QName,
    features: HashMap<QName, Feature<N>>,
}

impl<N: Node> Policy<N> {
    /// Create a new security policy
    pub fn new(name: QName) -> Self {
        Self {
            name,
            features: HashMap::new(),
        }
    }
    /// Get the name of the security policy
    pub fn name(&self) -> QName {
        self.name.clone()
    }
    /// Add a [Feature] to the security policy
    pub fn add(&mut self, name: QName, f: Feature<N>) {
        self.features.insert(name, f);
    }
    /// Get a [Feature] of the security policy
    pub fn feature(&self, name: &QName) -> Option<&Feature<N>> {
        self.features.get(name)
    }
    /*
    /// Get all of the [Feature]s of the security policy
    /// TODO: make this an iterator
    pub fn all_features(&self) -> Vec<&Feature<N>> {
        self.features.iter().map(|(_, f)| f).collect()
    }
    */
    /// Resolve the setting of a security [Feature].
    pub fn get(&self, name: &QName, a: ActualParameters<N>) -> Result<SecurityResult, Error> {
        self.features
            .get(name)
            .map_or_else(|| Ok(SecurityResult::NotPermitted), |f| f.get(a))
    }
}

/// A security feature. These limit or constrain acccess to a resource.
/// Access to a resource may, or may not, be permitted.
/// If access is permitted, then it may also be constrained so some maximum value.
/// This value is computed dynamically using a [Transform].
/// If no [Transform] is given then the access to the resource is unlimited.
#[derive(Clone, Debug)]
pub enum Feature<N: Node> {
    Permitted(Option<Transform<N>>),
    NotPermitted,
}

impl<N: Node> Feature<N> {
    /// Determine whether this security feature is permitted,
    /// and if so then to what limit, i.e. a maximum value.
    pub fn get(&self, a: ActualParameters<N>) -> Result<SecurityResult, Error> {
        match self {
            Feature::NotPermitted => Ok(SecurityResult::NotPermitted),
            Feature::Permitted(o) => Ok(SecurityResult::Permitted(if let Some(t) = o {
                // The template is a callable.
                // If the transformation results in an error then that it propegated back via the result
                let mut stctxt = StaticContextBuilder::new()
                    .message(|_| Ok(()))
                    .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
                    .fetcher(|_: &_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
                    .build();
                let mut ctxt = Context::new();
                //let mut actuals = HashMap::new();
                if let ActualParameters::Named(ap) = a {
                    ap.iter().try_for_each(|(an, av)| {
                        ctxt.var_push(an.to_string(), ctxt.dispatch(&mut stctxt, av)?);
                        //actuals.insert(an, ctxt.dispatch(&mut stctxt, av)?);
                        Ok(())
                    })?
                }
                // Now evaluate the template.
                // How to decide whether to return a (Not)Permitted result or a value?
                // If the singleton result is a boolean, then (Not)Permitted otherwise value.
                let r = ctxt.dispatch(&mut stctxt, t)?;
                // TODO: should the return value be the original Sequence?
                Some(r.to_string())
            } else {
                None
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::Item;
    use crate::trees::nullo::Nullo;
    use crate::value::Value;
    use std::rc::Rc;

    #[test]
    fn feature_get_np() {
        let f: Feature<Nullo> = Feature::NotPermitted;
        assert_eq!(
            f.get(ActualParameters::Named(vec![]))
                .expect("unable to determine status of security feature"),
            SecurityResult::NotPermitted
        )
    }

    #[test]
    fn feature_get_unlimited() {
        let f: Feature<Nullo> = Feature::Permitted(None);
        assert_eq!(
            f.get(ActualParameters::Named(vec![]))
                .expect("unable to determine status of security feature"),
            SecurityResult::Permitted(None)
        )
    }

    #[test]
    fn feature_get_limited() {
        let f: Feature<Nullo> = Feature::Permitted(Some(Transform::Literal(Item::Value(Rc::new(
            Value::from(1234),
        )))));
        assert_eq!(
            f.get(ActualParameters::Named(vec![]))
                .expect("unable to determine status of security feature"),
            SecurityResult::Permitted(Some(String::from("1234")))
        )
    }
}
