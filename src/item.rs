//! # xrust::item
//!
//! Sequence Item module.
//! An Item is a Node, Function or Atomic Value.
//!
//! Nodes are implemented as a trait.

use std::rc::Rc;
use crate::value::{Value, Operator};
use crate::forest::{NodeType, Node, Forest};
use crate::qname::QualifiedName;
use crate::output::OutputDefinition;
use crate::xdmerror::{Error, ErrorKind};

/// In XPath, the Sequence is the fundamental data structure.
/// It is an ordered collection of [Item]s.
/// The Rust impementation is a Vector of reference counted [Item]s.
///
/// See [SequenceTrait] for methods.
pub type Sequence = Vec<Rc<Item>>;

pub trait SequenceTrait {
    /// Return the string value of the [Sequence].
    fn to_string(&self, d: Option<&Forest>) -> String;
    /// Return a XML formatted representation of the [Sequence].
    fn to_xml(&self, d: Option<&Forest>) -> String;
    /// Return a XML formatted representation of the [Sequence], controlled by the supplied output definition.
    fn to_xml_with_options(&self, od: &OutputDefinition, d: Option<&Forest>) -> String;
    /// Return a JSON formatted representation of the [Sequence].
    fn to_json(&self, d: Option<&Forest>) -> String;
    /// Return the Effective Boolean Value of the [Sequence].
    fn to_bool(&self) -> bool;
    /// Convert the [Sequence] to an integer. The [Sequence] must be a singleton value.
    fn to_int(&self) -> Result<i64, Error>;
    /// Push a [Node] to the [Sequence]
    fn push_node(&mut self, n: Node);
    /// Push a [Value] to the [Sequence]
    fn push_value(&mut self, v: Value);
    /// Push an [Item] to the [Sequence]
    fn push_item(&mut self, i: &Rc<Item>);
}

impl SequenceTrait for Sequence {
    /// Returns the string value of the Sequence.
    fn to_string(&self, d: Option<&Forest>) -> String {
	let mut r = String::new();
	for i in self {
	    r.push_str(i.to_string(d).as_str())
	}
	r
    }
    /// Renders the Sequence as XML
    fn to_xml(&self, d: Option<&Forest>) -> String {
	let mut r = String::new();
	for i in self {
	    r.push_str(i.to_xml(d).as_str())
	}
	r
    }
    /// Renders the Sequence as XML
    fn to_xml_with_options(&self, od: &OutputDefinition, d: Option<&Forest>) -> String {
	let mut r = String::new();
	for i in self {
	    r.push_str(i.to_xml_with_options(od, d).as_str())
	}
	r
    }
    /// Renders the Sequence as JSON
    fn to_json(&self, d: Option<&Forest>) -> String {
	let mut r = String::new();
	for i in self {
	    r.push_str(i.to_json(d).as_str())
	}
	r
    }
    /// Push a Document's [Node] on to the [Sequence]
    fn push_node(&mut self, n: Node) {
	self.push(Rc::new(Item::Node(n)));
    }
    /// Push a [Value] on to the [Sequence]
    fn push_value(&mut self, v: Value) {
	self.push(Rc::new(Item::Value(v)));
    }
  //fn new_function(&self, f: Function) -> Sequence {
  //}
    /// Push an [Item] on to the [Sequence]. This clones the Item.
    fn push_item(&mut self, i: &Rc<Item>) {
	self.push(Rc::clone(i));
    }

    /// Calculate the effective boolean value of the Sequence
    fn to_bool(&self) -> bool {
	if self.len() == 0 {
	    false
	} else {
	    match *self[0] {
		Item::Node(..) => true,
		_ => {
		    if self.len() == 1 {
			(*self[0]).to_bool()
		    } else {
			false // should be a type error
		    }
		}
	    }
	}
    }

    /// Convenience routine for integer value of the [Sequence]. The Sequence must be a singleton; i.e. be a single item.
    fn to_int(&self) -> Result<i64, Error> {
	if self.len() == 1 {
	    self[0].to_int()
	} else {
	    Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error: sequence is not a singleton")})
	}
    }
}

impl From<Node> for Sequence {
    fn from(n: Node) -> Self {
	vec![Rc::new(Item::Node(n))]
    }
}
impl From<Value> for Sequence {
    fn from(v: Value) -> Self {
	vec![Rc::new(Item::Value(v))]
    }
}
impl From<Item> for Sequence {
    fn from(i: Item) -> Self {
	vec![Rc::new(i)]
    }
}

/// An Item in a [Sequence]. Can be a [Node], Function or [Value].
///
/// [Node]s are dynamic trait objects. [Node]s can only exist in the context of a [Document].
///
/// Functions are not yet implemented.
//#[derive(Clone)]
pub enum Item {
    /// A [Node]
    Node(Node),

    /// Functions are not yet supported
    Function,

    /// A scalar value
    Value(Value),
}

impl Item {
    /// Gives the string value of an item. All items have a string value.
    pub fn to_string(&self, d: Option<&Forest>) -> String {
	match self {
	    Item::Node(n) => d.map_or(
		String::new(),
		|e| n.to_string(e)
	    ),
	    Item::Function => "".to_string(),
	    Item::Value(v) => v.to_string(),
	}
    }
    /// Serialize as XML
    pub fn to_xml(&self, d: Option<&Forest>) -> String {
	match self {
	    Item::Node(n) => d.map_or(
		String::new(),
		|e| n.to_xml(e)
	    ),
	    Item::Function => "".to_string(),
	    Item::Value(v) => v.to_string(),
	}
    }
    /// Serialize as XML, with options
    pub fn to_xml_with_options(&self, od: &OutputDefinition, d: Option<&Forest>) -> String {
	match self {
	    Item::Node(n) => d.map_or(
		String::new(),
		|e| n.to_xml_with_options(e, od)
	    ),
	    Item::Function => "".to_string(),
	    Item::Value(v) => v.to_string(),
	}
    }
    /// Serialize as JSON
    pub fn to_json(&self, d: Option<&Forest>) -> String {
	match self {
	    Item::Node(n) => d.map_or(
		String::new(),
		|e| n.to_json(e)
	    ),
	    Item::Function => "".to_string(),
	    Item::Value(v) => v.to_string(),
	}
    }

    /// Determine the effective boolean value of the item.
    /// See XPath 2.4.3.
    pub fn to_bool(&self) -> bool {
	match self {
	    Item::Node(..) => true,
	    Item::Function => false,
	    Item::Value(v) => v.to_bool(),
	}
    }

    /// Gives the integer value of the item, if possible.
    pub fn to_int(&self) -> Result<i64, Error> {
	match self {
	    Item::Node(..) => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error: item is a node")}),
	    Item::Function => Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error: item is a function")}),
	    Item::Value(v) => {
		match v.to_int() {
		    Ok(i) => {
			Ok(i)
		    }
		    Err(e) => {
			Result::Err(e)
		    }
		}
	    },
	}
    }

    /// Gives the double value of the item. Returns NaN if the value cannot be converted to a double.
    pub fn to_double(&self) -> f64 {
	match self {
	    Item::Node(..) => f64::NAN,
	    Item::Function => f64::NAN,
	    Item::Value(v) => v.to_double(),
	}
    }

    /// Gives the name of the item. Certain types of Nodes have names, such as element-type nodes. If the item does not have a name returns an empty string.
    pub fn to_name(&self, d: Option<&Forest>) -> QualifiedName {
	match self {
	    Item::Node(n) => d.map_or(
		QualifiedName::new(None, None, "".to_string()),
		|e| n.to_name(e)
	    ),
	    _ => QualifiedName::new(None, None, "".to_string())
	}
    }

    // TODO: atomization
    // fn atomize(&self);

    /// Compare two items.
    pub fn compare(&self, other: &Item, op: Operator, d: Option<&Forest>) -> Result<bool, Error> {
	match self {
	    Item::Value(v) => {
		match other {
		    Item::Value(w) => {
			v.compare(w, op)
		    }
		    Item::Node(..) => {
			v.compare(&Value::String(other.to_string(d)), op)
		    }
		    _ => {
			Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error")})
		    }
		}
	    }
	    Item::Node(..) => {
		other.compare(&Item::Value(Value::String(self.to_string(d))), op, d)
	    }
	    _ => {
		Result::Err(Error{kind: ErrorKind::TypeError, message: String::from("type error")})
	    }
	}
    }

    /// Is this item an element-type node?
    pub fn is_element_node(&self, d: Option<&Forest>) -> bool {
	match self {
	    Item::Node(n) => {
		d.map_or(
		    false,
		    |e| match n.node_type(e) {
			NodeType::Element => true,
			_ => false,
		    }
		)
	    }
	    _ => false,
	}
    }

    /// Gives the type of the item.
    pub fn item_type(&self) -> &'static str {
	match self {
	    Item::Node(..) => "Node",
	    Item::Function => "Function",
	    Item::Value(v) => v.value_type(),
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // to_bool

    #[test]
    fn bool_value_string_empty() {
	assert_eq!(Item::Value(Value::from("")).to_bool(), false)
    }
    #[test]
    fn bool_value_string_nonempty() {
      assert_eq!(Item::Value(Value::from("false")).to_bool(), true)
    }
    #[test]
    fn bool_value_int_zero() {
      assert_eq!(Item::Value(Value::from(0)).to_bool(), false)
    }
    #[test]
    fn bool_value_int_nonzero() {
      assert_eq!(Item::Value(Value::from(42)).to_bool(), true)
    }

    // to_int

    #[test]
    fn int_value_string() {
      match Item::Value(Value::from("1")).to_int() {
        Ok(i) => assert_eq!(i, 1),
	Err(_) => {
	  panic!("to_int() failed")
	}
      }
    }

    // to_double

    #[test]
    fn double_value_string() {
      assert_eq!(Item::Value(Value::from("2.0")).to_double(), 2.0)
    }

    // Sequences

    #[test]
    fn sequence() {
        let _s = Sequence::new();
        assert!(true)
    }
    #[test]
    fn sequence_one() {
        let mut s = Sequence::new();
	s.push_value(Value::from("one"));
	let mut t = Sequence::new();
	t.push_item(&s[0]);
	assert!(Rc::ptr_eq(&s[0], &t[0]))
    }
}
