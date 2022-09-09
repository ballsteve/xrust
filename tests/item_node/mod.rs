#[macro_export]
macro_rules! item_node_mutable_tests (
    ( $x:expr ) => {
	use xrust::item::MNode;

	fn mnode_push_content_generic<D: MNode>(mut d: D) {
	    let n = d.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element node");
	    d.push(n)
		.expect("unable to add node");
	    assert_eq!(d.to_xml(), "<Test></Test>")
	}

	#[test]
	fn mnode_push_content() {
	    mnode_push_content_generic($x())
	}
    }
);

#[macro_export]
macro_rules! item_node_immutable_tests (
    ( $x:expr , $y:ty ) => {
	use xrust::item::{INode, NodeType};

	// This test expects the document to have a single toplevel element
	// TODO: filter nodes to get elements and check there is only one
	fn item_inode_type_generic<I: INode>(d: I) {
	    assert_eq!(d.node_type(), NodeType::Document)
	}
	fn item_inode_name_generic<I: INode>(d: I) {
	    match d.child_iter().nth(0) {
		Some(c) => {
		    assert_eq!(c.node_type(), NodeType::Element);
		    assert_eq!(c.name().to_string(), "Test")
		}
		None => panic!("no toplevel element")
	    }
	}

	fn item_inode_value_generic<I: INode>(d: I) {
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
	fn item_inode_to_string_doc_generic<I: INode>(d: I) {
	    assert_eq!(d.to_string(), "foobar")
	}
	fn item_inode_to_xml_doc_generic<I: INode>(d: I) {
	    assert_eq!(d.to_xml(), "<Test>foobar</Test>")
	}

	#[test]
	fn item_inode_type() {
	    item_inode_type_generic::<$y>($x(QualifiedName::new(None, None, String::from("Test")), Value::from("foobar")))
	}

	#[test]
	fn item_inode_name() {
	    item_inode_name_generic::<$y>($x(QualifiedName::new(None, None, String::from("Test")), Value::from("foobar")))
	}

	#[test]
	fn item_inode_value() {
	    item_inode_value_generic::<$y>($x(QualifiedName::new(None, None, String::from("Test")), Value::from("foobar")))
	}

	#[test]
	fn item_inode_to_string_doc() {
	    item_inode_to_string_doc_generic::<$y>($x(QualifiedName::new(None, None, String::from("Test")), Value::from("foobar")))
	}

	#[test]
	fn item_inode_to_xml_doc() {
	    item_inode_to_xml_doc_generic::<$y>($x(QualifiedName::new(None, None, String::from("Test")), Value::from("foobar")))
	}
    }
);
