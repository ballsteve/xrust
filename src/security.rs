//! Support for security policies.
//!
//! A security policy allows a module to limit, or constrain, access to a resource.
//!
//! # Security Features
//! The resource is named, using a [QName], and the module will call into the in-force policy to determine the limitation set on the resource.
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
//! Resource constraints are specified with a template.
//! Templates use the same syntax as XSLT templates.
//! The template is evaluated when a module wants to grant access to a resource.
//! The parameters provided by the module, see above, are passed to the template.
//! The template must return a single element, one of:
//! ```xslt
//! Q{http://gitlab.gnome.org/World/Rust/markup-rs/Security}not-permitted
//! Q{http://gitlab.gnome.org/World/Rust/markup-rs/Security}permitted
//! ```
//!
//! These elements map, respectively, to:
//! * SecurityResult::NotPermitted
//! * SecurityResult::Permitted(Option<String>)
//!
//! If the 'permitted' element does not contain content, then None is set in the SecurityResult::Permitted value.
//! Otherwise a Some is inserted with the string value of the content.
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
//!    Transform::LiteralElement(
//!      QName::new_from_parts(
//!        NcName::try_from("permitted").unwrap(),
//!        NamespaceUri::try_from("http://gitlab.gnome.org/World/Rust/markup-rs/Security").unwrap(),
//!      )),
//!      Box::new(Transform::Empty),
//!    ),
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
use crate::transform::context::{ContextBuilder, StaticContextBuilder};
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
    pub fn get<F>(
        &self,
        f: &QName,
        a: ActualParameters<N>,
        make_doc: F,
    ) -> Result<SecurityResult, Error>
    where
        F: Fn() -> N,
    {
        // If there is no in-force security policy then all features are not permitted
        if self.in_force.is_none() {
            return Ok(SecurityResult::NotPermitted);
        }

        // Does the in-force security policy have the requested feature?
        // If not then it is not permitted
        if let Some(p) = self.policies.get(&self.in_force.as_ref().unwrap()) {
            p.get(f, a, make_doc)
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
    pub fn get<F>(
        &self,
        name: &QName,
        a: ActualParameters<N>,
        make_doc: F,
    ) -> Result<SecurityResult, Error>
    where
        F: Fn() -> N,
    {
        self.features
            .get(name)
            .map_or_else(|| Ok(SecurityResult::NotPermitted), |f| f.get(a, make_doc))
    }
}

/// Build a [Policy] from an XML document.
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
/// The content is an XSLT template. It must result in a single element node, optionally with content. See above for the interpretation of the element.
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
//impl<N: Node> From<N> for Policy<N> {
pub fn try_from_document<N: Node>(doc: N) -> Result<Policy<N>, Error> {
    // doc must be a document-type node
    // TODO: make the QNames constants
    let secnsuri =
        NamespaceUri::try_from("http://gitlab.gnome.org/World/Rust/markup-rs/Security").unwrap();
    if let Some(top) = doc.first_child() {
        if !top.name().is_some_and(|qn| {
            qn == QName::new_from_parts(NcName::try_from("policy").unwrap(), Some(secnsuri.clone()))
        }) {
            return Err(Error::new(
                ErrorKind::TypeError,
                "not a security policy document",
            ));
        }
        let name = top
            .get_attribute(&QName::from_local_name(NcName::try_from("name").unwrap()))
            .to_string();
        if name != "" {
            // Resolve qualified name to a QName using the doc's namespaces
            let mut policy = Policy::new(top.to_qname(name)?);

            // Content is feature elements, skipping over white space
            let fname =
                QName::new_from_parts(NcName::try_from("feature").unwrap(), Some(secnsuri.clone()));
            top.child_iter()
                .filter(|c| c.name().is_some_and(|n| n == fname))
                .try_for_each(|f| {
                    let feat_name = f
                        .get_attribute(&QName::from_local_name(NcName::try_from("name").unwrap()))
                        .to_string();
                    if feat_name != "" {
                        // Content is the template to evaluate
                        let mut body: Vec<Transform<N>> = vec![];
                        // attribute sets are not used in this context
                        let attr_sets: HashMap<QName, Vec<Transform<N>>> = HashMap::new();

                        // Strip whitespace
                        let feat_children: Vec<N> = f
                            .child_iter()
                            .filter(|fc| {
                                fc.node_type() != NodeType::Text
                                    || !fc.value().to_string().trim().is_empty()
                            })
                            .collect();

                        // Compile template
                        feat_children.into_iter().try_for_each(|d| {
                            body.push(to_transform(d, &attr_sets)?);
                            Ok::<(), Error>(())
                        })?;
                        if body.len() != 1 {
                            return Err(Error::new(
                                ErrorKind::TypeError,
                                "template must result in a single node",
                            ));
                        }
                        policy.add(top.to_qname(feat_name)?, Feature(body.remove(0)));
                    } else {
                        return Err(Error::new(
                            ErrorKind::DynamicAbsent,
                            "feature must have a name",
                        ));
                    }
                    Ok(())
                })?;
            Ok(policy)
        } else {
            Err(Error::new(
                ErrorKind::DynamicAbsent,
                "name attribute is required",
            ))
        }
    } else {
        Err(Error::new(ErrorKind::DynamicAbsent, "empty document"))
    }
}

/// A security feature. These limit or constrain acccess to a resource.
/// Access to a resource may, or may not, be permitted.
/// If access is permitted, then it may also be constrained so some maximum value.
/// This value is computed dynamically using a [Transform].
/// The transformation is not allowed to access external resources.
#[derive(Clone, Debug)]
pub struct Feature<N: Node>(Transform<N>);

impl<N: Node> Feature<N> {
    /// Create a Feature
    pub fn new(t: Transform<N>) -> Self {
        Feature(t)
    }

    /// Evaluate the template to determine whether this security feature is permitted,
    /// and if so then to what limit, i.e. a maximum value.
    pub fn get<F>(&self, a: ActualParameters<N>, make_doc: F) -> Result<SecurityResult, Error>
    where
        F: Fn() -> N,
    {
        // The template is a callable.
        // If the transformation results in an error then that it propagated back via the result
        let mut stctxt = StaticContextBuilder::new()
            .message(|_| Ok(()))
            .parser(|_| {
                Err(Error::new(
                    ErrorKind::StaticBadFunction,
                    "external resources are not allowed",
                ))
            })
            .fetcher(|_: &_| {
                Err(Error::new(
                    ErrorKind::StaticBadFunction,
                    "external resources are not allowed",
                ))
            })
            .build();
        let rd = make_doc();
        let mut ctxt = ContextBuilder::new().result_document(rd).build();
        if let ActualParameters::Named(ap) = a {
            ap.iter().try_for_each(|(an, av)| {
                ctxt.var_push(an.to_string(), ctxt.dispatch(&mut stctxt, av)?);
                Ok(())
            })?
        }
        // TODO: make these constants
        let np = QName::new_from_parts(
            NcName::try_from("not-permitted").unwrap(),
            Some(
                NamespaceUri::try_from("http://gitlab.gnome.org/World/Rust/markup-rs/Security")
                    .unwrap(),
            ),
        );
        let p = QName::new_from_parts(
            NcName::try_from("permitted").unwrap(),
            Some(
                NamespaceUri::try_from("http://gitlab.gnome.org/World/Rust/markup-rs/Security")
                    .unwrap(),
            ),
        );

        // Now evaluate the template. It must result in a single element node.
        let r = ctxt.dispatch(&mut stctxt, &self.0)?;
        if r.len() == 1 {
            if r[0].is_element_node() {
                if r[0].name().unwrap() == np {
                    Ok(SecurityResult::NotPermitted)
                } else if r[0].name().unwrap() == p {
                    let content = r[0].to_string();
                    if content.is_empty() {
                        Ok(SecurityResult::Permitted(None))
                    } else {
                        Ok(SecurityResult::Permitted(Some(content)))
                    }
                } else {
                    Err(Error::new(
                        ErrorKind::TypeError,
                        "result must be a permitted or not-permitted element",
                    ))
                }
            } else {
                Err(Error::new(
                    ErrorKind::DynamicAbsent,
                    "result must be an element",
                ))
            }
        } else {
            Err(Error::new(
                ErrorKind::DynamicAbsent,
                "result must be a single element",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::Item;
    use crate::trees::smite::RNode;
    use crate::value::Value;
    use std::rc::Rc;

    #[test]
    fn feature_get_np() {
        let f = Feature(Transform::LiteralElement(
            QName::new_from_parts(
                NcName::try_from("not-permitted").unwrap(),
                Some(
                    NamespaceUri::try_from("http://gitlab.gnome.org/World/Rust/markup-rs/Security")
                        .unwrap(),
                ),
            ),
            Box::new(Transform::Empty),
        ));
        assert_eq!(
            f.get(ActualParameters::Named(vec![]), RNode::new_document)
                .expect("unable to determine status of security feature"),
            SecurityResult::NotPermitted
        )
    }

    #[test]
    fn feature_get_unlimited() {
        let f = Feature(Transform::LiteralElement(
            QName::new_from_parts(
                NcName::try_from("permitted").unwrap(),
                Some(
                    NamespaceUri::try_from("http://gitlab.gnome.org/World/Rust/markup-rs/Security")
                        .unwrap(),
                ),
            ),
            Box::new(Transform::Empty),
        ));
        assert_eq!(
            f.get(ActualParameters::Named(vec![]), RNode::new_document)
                .expect("unable to determine status of security feature"),
            SecurityResult::Permitted(None)
        )
    }

    #[test]
    fn feature_get_limited() {
        let f = Feature(Transform::LiteralElement(
            QName::new_from_parts(
                NcName::try_from("permitted").unwrap(),
                Some(
                    NamespaceUri::try_from("http://gitlab.gnome.org/World/Rust/markup-rs/Security")
                        .unwrap(),
                ),
            ),
            Box::new(Transform::Literal(Item::Value(Rc::new(Value::from(1234))))),
        ));
        assert_eq!(
            f.get(ActualParameters::Named(vec![]), RNode::new_document)
                .expect("unable to determine status of security feature"),
            SecurityResult::Permitted(Some(String::from("1234")))
        )
    }
}
