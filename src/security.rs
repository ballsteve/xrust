//! Support for security policies.
//!
//! A security policy allows a module to limit, or constrain, access to a resource.
//!
//! # Security Features
//! The resource is named, using a [QName], and the module will call into the in-force policy to retrieve the limitation set on the resource.
//! The limitation is returned as a [SecurityResult].
//! The module may provide [ActualParameters] to the feature, refer to the module's documentation for details.
//!
//! ```rust
//! # use std::rc::Rc;
//! use xrust::security::{SecurityResult, Policy, Feature};
//! use xrust::{Error, ErrorKind, Node};
//! use xrust::item::Item;
//! use xrust::value::Value;
//! use xrust::transform::Transform;
//! use xrust::transform::callable::ActualParameters;
//! use qualname::{QName, NcName};
//!
//! fn get_feature<N: Node>(policy: &Policy<N>) -> Result<Option<String>, Error> {
//!    match policy.get(
//!       &QName::from_local_name(NcName::try_from("my_security_feature").unwrap()),
//!       ActualParameters::Named(vec![
//!          (QName::from_local_name(NcName::try_from("input").unwrap()),
//!           Transform::Literal(Item::Value(Rc::new(Value::from("value")))))
//!       ])
//!    )? {
//!        SecurityResult::NotPermitted => Err(Error::new(ErrorKind::NotPermitted, "access denied")),
//!        SecurityResult::Permitted(None) => Ok(None),
//!        SecurityResult::Permitted(Some(v)) => Ok(Some(v)),
//!    }
//! }
//! ```
//!
//! If a policy does not define a limit or constraint for a resource,
//! then the module will define a default value. The module should set a default that has minimal security implications for the application.
//! Most likely this will be to deny access to the resource.
//!
//! # Security Policies
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
//!
//! # Serialisation
//! Security policies may be represented as an XML document.
//! See the From trait implementation for [Policy].

#![allow(rustdoc::bare_urls)]

use std::collections::HashMap;

use crate::item::{Node, NodeType, SequenceTrait};
use crate::transform::Transform;
use crate::transform::callable::ActualParameters;
use crate::transform::context::{Context, StaticContextBuilder};
use crate::xdmerror::{Error, ErrorKind};
use crate::xslt::to_transform;
use qualname::{NamespaceUri, NcName, QName};

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

/// Build a [Policy] from an XML document.
/// This will panic if an error is found in the document.
///
/// A security policy document has Q{http://gitlab.gnome.org/World/Rust/markup-rs/Security}policy as its toplevel element.
/// The policy element must have a name attribute.
///
/// The policy element may have one or more Q{http://gitlab.gnome.org/World/Rust/markup-rs/Security}feature child elements.
/// Each feature element must have a name attribute which has the qualified name of a security feature.
/// The feature element must contain either a Q{http://gitlab.gnome.org/World/Rust/markup-rs/Security}Permitted or {http://gitlab.gnome.org/World/Rust/markup-rs/Security}not-permitted element.
/// Which element is present determines whether the security feature is permitted or not.
///
/// The Q{http://gitlab.gnome.org/World/Rust/markup-rs/Security}permitted may contain child content.
/// If there is no content then the feature is permitted, but has no value.
/// If there is content then it is evaluated to determine the value for the feature.
/// The content is an XSLT template. Only XSLT elements that do not create nodes may be used. For example, use xsl:sequence rather than xsl:value-of.
///
/// Example security policy document:
///
/// ```xml
/// <sec:policy name="my-policy" xmlns:sec='http://gitlab.gnome.org/World/Rust/markup-rs/Security'
///    xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
///   <sec:feature name="feature1">
///     <sec:not-permitted/>
///   </sec:feature>
///   <sec:feature name="feature2">
///     <sec:permitted/>
///   </sec:feature>
///   <sec:feature name="feature3">
///     <sec:permitted>
///       <xsl:sequence select='42'/>
///     </sec:permitted>
///   </sec:feature>
///   <sec:feature name="feature4">
///     <sec:permitted>42</sec:permitted>
///   </sec:feature>
/// </sec:policy>
/// ```
///
/// TODO: a TryFrom version.
impl<N: Node> From<N> for Policy<N> {
    //type Error = Error;
    fn from(doc: N) -> Self {
        // doc must be a document-type node
        let secnsuri =
            NamespaceUri::try_from("http://gitlab.gnome.org/World/Rust/markup-rs/Security")
                .unwrap();
        if let Some(top) = doc.first_child() {
            if !top.name().is_some_and(|qn| {
                qn == QName::new_from_parts(
                    NcName::try_from("policy").unwrap(),
                    Some(secnsuri.clone()),
                )
            }) {
                panic!("not a security policy document")
                /*return Err(Error::new(
                    ErrorKind::TypeError,
                    "not a security policy document",
                ));*/
            }
            // TODO: support name as a QName
            let name = top
                .get_attribute(&QName::from_local_name(NcName::try_from("name").unwrap()))
                .to_string();
            if name != "" {
                let mut policy = Policy::new(QName::from_local_name(
                    NcName::try_from(name.as_str()).unwrap(),
                ));

                // Content is feature elements, skipping over white space
                let fname = QName::new_from_parts(
                    NcName::try_from("feature").unwrap(),
                    Some(secnsuri.clone()),
                );
                let pname = QName::new_from_parts(
                    NcName::try_from("permitted").unwrap(),
                    Some(secnsuri.clone()),
                );
                let npname = QName::new_from_parts(
                    NcName::try_from("not-permitted").unwrap(),
                    Some(secnsuri.clone()),
                );
                top.child_iter()
                    .filter(|c| c.name().is_some_and(|n| n == fname))
                    .for_each(|f| {
                        let feat_name = f
                            .get_attribute(&QName::from_local_name(
                                NcName::try_from("name").unwrap(),
                            ))
                            .to_string();
                        if feat_name != "" {
                            // Check that there is only one child element
                            let fc: Vec<N> = f
                                .child_iter()
                                .skip_while(|c| c.node_type() != NodeType::Element)
                                .take(1)
                                .collect();
                            if fc.is_empty() {
                                panic!("feature missing element")
                            } else {
                                // TODO: support QName for feature name
                                if fc[0].name().unwrap() == npname {
                                    policy.add(
                                        QName::from_local_name(
                                            NcName::try_from(feat_name.as_str()).unwrap(),
                                        ),
                                        Feature::NotPermitted,
                                    );
                                } else if fc[0].name().unwrap() == pname {
                                    let feat_children: Vec<N> = fc[0].child_iter().filter(|fc| fc.node_type() != NodeType::Text || !fc.value().to_string().trim().is_empty()).collect();
                                    if feat_children.is_empty() {
                                        policy.add(
                                            QName::from_local_name(
                                                NcName::try_from(feat_name.as_str()).unwrap(),
                                            ),
                                            Feature::Permitted(None),
                                        );
                                    } else {
                                        let mut body: Vec<Transform<N>> = vec![];
                                        // attribute sets are not used in this context
                                        let attr_sets: HashMap<QName, Vec<Transform<N>>> = HashMap::new();

                                        feat_children.into_iter().try_for_each(|d| {
                                            body.push(to_transform(d, &attr_sets)?);
                                            Ok::<(), Error>(())
                                        }).expect("unable to compile transformation");

                                        policy.add(
                                            QName::from_local_name(
                                                NcName::try_from(feat_name.as_str()).unwrap(),
                                            ),
                                            Feature::Permitted(Some(Transform::SequenceItems(body))),
                                        );
                                    }
                                } else {
                                    panic!("wrong element in feature: must be permitted or not-permitted")
                                }
                            }
                        } else {
                            panic!("feature must have a name")
                        }
                    });
                policy
            } else {
                panic!("name attribute is required")
            }
        } else {
            panic!("empty document")
            //Err(Error::new(ErrorKind::DynamicAbsent, "empty document"))
        }
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
