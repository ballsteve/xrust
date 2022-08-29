#[macro_export]
macro_rules! item_node_tests_a (
    ( $x:expr ) => {

	fn anode_push_content_generic<D: RWDocument>(mut d: D) {
	    let n = d.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element node");
	    d.push_content(n)
		.expect("unable to add node");
	    assert_eq!(d.to_xml(), "<Test></Test>")
	}

	#[test]
	fn anode_push_content() {
	    anode_push_content_generic($x())
	}
    }
);

#[macro_export]
macro_rules! item_node_tests_b (
    ( $x:expr , $y:ty , $z:ty ) => {
	use xrust::item::{Document, Node, NodeType};

	// This test expects the document to have a single toplevel element
	// TODO: filter nodes to get elements and check there is only one
	fn item_node_type_generic<D: Document, N: Node>(d: D) {
	    match d.child_iter().nth(0) {
		Some(c) => {
		    assert_eq!(c.node_type(), NodeType::Element)
		}
		None => panic!("no toplevel element")
	    }
	}
	fn item_node_root_generic<D: Document, N: Node>(d: D) {
	    match d.root_element() {
		Some(c) => {
		    assert_eq!(c.node_type(), NodeType::Element)
		}
		None => panic!("no toplevel element")
	    }
	}
	fn item_node_name_generic<D: Document, N: Node>(d: D) {
	    match d.child_iter().nth(0) {
		Some(c) => {
		    assert_eq!(c.node_type(), NodeType::Element);
		    assert_eq!(c.name().to_string(), "Test")
		}
		None => panic!("no toplevel element")
	    }
	}

	fn item_node_value_generic<D: Document, N: Node>(d: D) {
	    match d.child_iter().nth(0) {
		Some(c) => {
		    assert_eq!(c.node_type(), NodeType::Element);
		    assert_eq!(c.name().to_string(), "Test");
		    let mut it = c.child_iter();
		    match it.next() {
			Some(t) => {
			    assert_eq!(t.node_type(), NodeType::Text);
			    assert_eq!(t.value().to_string(), "foobar");
			    match it.next() {
				Some(_) => panic!("unexpected child node"),
				None => assert!(true)
			    }
			}
			None => panic!("root element does not have child node")
		    }
		}
		None => panic!("no toplevel element")
	    }
	}
	fn item_node_to_string_doc_generic<D: Document, N: Node>(d: D) {
	    assert_eq!(d.to_string(), "foobar")
	}
	fn item_node_to_xml_doc_generic<D: Document, N: Node>(d: D) {
	    assert_eq!(d.to_xml(), "<Test>foobar</Test>")
	}

	#[test]
	fn item_node_root() {
	    item_node_root_generic::<$y, $z>($x(QualifiedName::new(None, None, String::from("Test")), Value::from("foobar")))
	}

	#[test]
	fn item_node_type() {
	    item_node_type_generic::<$y, $z>($x(QualifiedName::new(None, None, String::from("Test")), Value::from("foobar")))
	}

	#[test]
	fn item_node_name() {
	    item_node_name_generic::<$y, $z>($x(QualifiedName::new(None, None, String::from("Test")), Value::from("foobar")))
	}

	#[test]
	fn item_node_value() {
	    item_node_value_generic::<$y, $z>($x(QualifiedName::new(None, None, String::from("Test")), Value::from("foobar")))
	}

	#[test]
	fn item_node_to_string_doc() {
	    item_node_to_string_doc_generic::<$y, $z>($x(QualifiedName::new(None, None, String::from("Test")), Value::from("foobar")))
	}

	#[test]
	fn item_node_to_xml_doc() {
	    item_node_to_xml_doc_generic::<$y, $z>($x(QualifiedName::new(None, None, String::from("Test")), Value::from("foobar")))
	}

//	#[test]
//	fn item_node_child_iter() {
//	    item_node_child_iter_generic::<$y, $z>($x(QualifiedName::new(None, None, String::from("Test")), Value::from("foobar")))
//	}
    }
);
