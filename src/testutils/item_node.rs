#[macro_export]
macro_rules! item_node_tests (
    ( $x:expr, $y:expr, $z:expr ) => {
	use std::cmp::Ordering;

	#[test]
	fn node_push_content() {
	    let mut d = $x();
	    let n = d.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element node");
	    d.push(n)
		.expect("unable to add node");
	    assert_eq!(d.to_xml(), "<Test></Test>")
	}

	// This test expects the document to have a single toplevel element
	// TODO: filter nodes to get elements and check there is only one
	#[test]
	fn item_node_type() {
	    assert_eq!(
		$y(Rc::new(QualifiedName::new(None, None, String::from("Test"))), Value::from("foobar")).node_type(),
		NodeType::Document
	    )
	}

	#[test]
	fn item_node_name() {
	    let d = $y(Rc::new(QualifiedName::new(None, None, String::from("Test"))), Value::from("foobar"));
	    match d.child_iter().nth(0) {
			Some(c) => {
		    	assert_eq!(c.node_type(), NodeType::Element);
		    	assert_eq!(c.name().to_string(), "Test")
			}
			None => panic!("no toplevel element")
	    }
	}

	#[test]
	fn item_node_value() {
	    let d = $y(Rc::new(QualifiedName::new(None, None, String::from("Test"))), Value::from("foobar"));
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

	#[test]
	fn item_node_pop() {
	    let mut d = $x();
	    let mut new = d.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element node");
	    d.push(new.clone())
		.expect("unable to add node");
	    assert_eq!(d.to_xml(), "<Test></Test>");
	    let mut e = d.new_element(Rc::new(QualifiedName::new(None, None, String::from("Foo"))))
		.expect("unable to create element node");
	    new.push(e.clone())
		.expect("unable to add node");
	    let mut f = d.new_element(Rc::new(QualifiedName::new(None, None, String::from("Bar"))))
		.expect("unable to create element node");
	    e.push(f)
		.expect("unable to add node");
	    assert_eq!(d.to_xml(), "<Test><Foo><Bar></Bar></Foo></Test>");
	    e.pop()
		.expect("unable to remove node");
	    assert_eq!(d.to_xml(), "<Test></Test>")
	}

	#[test]
	fn item_node_insert_before() {
	    let mut d = $x();
	    let mut new = d.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element node");
	    d.push(new.clone())
		.expect("unable to add node");
	    assert_eq!(d.to_xml(), "<Test></Test>");
	    let mut e = d.new_element(Rc::new(QualifiedName::new(None, None, String::from("Foo"))))
		.expect("unable to create element node");
	    new.push(e.clone())
		.expect("unable to add node");
	    let mut f = d.new_element(Rc::new(QualifiedName::new(None, None, String::from("Bar"))))
		.expect("unable to create element node");
	    e.push(f.clone())
		.expect("unable to add node");
	    assert_eq!(d.to_xml(), "<Test><Foo><Bar></Bar></Foo></Test>");
	    let g = d.new_element(Rc::new(QualifiedName::new(None, None, String::from("Inserted"))))
		.expect("unable to create element node");
	    f.insert_before(g)
		.expect("unable to insert element");
	    assert_eq!(d.to_xml(), "<Test><Foo><Inserted></Inserted><Bar></Bar></Foo></Test>")
	}

	#[test]
	fn item_node_to_string_doc() {
	    let d = $y(Rc::new(QualifiedName::new(None, None, String::from("Test"))), Value::from("foobar"));
	    assert_eq!(d.to_string(), "foobar")
	}

	#[test]
	fn item_node_to_xml_doc() {
	    let d = $y(Rc::new(QualifiedName::new(None, None, String::from("Test"))), Value::from("foobar"));
	    assert_eq!(d.to_xml(), "<Test>foobar</Test>")
	}

	#[test]
	fn item_node_parent() {
	    let mut sd = $x();
	    let mut t = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1 = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Level-1"))))
		.expect("unable to create element");
	    t.push(l1.clone())
		.expect("unable to append child");
	    let l2 = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Level-2"))))
		.expect("unable to create element");
	    l1.push(l2.clone())
		.expect("unable to append child");
	    assert_eq!(l2.parent().unwrap().name().to_string(), "Level-1")
	}

	#[test]
	fn item_node_ancestor() {
	    let mut sd = $x();
	    let mut t = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1 = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Level-1"))))
		.expect("unable to create element");
	    t.push(l1.clone())
		.expect("unable to append child");
	    let mut l2 = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Level-2"))))
		.expect("unable to create element");
	    l1.push(l2.clone())
		.expect("unable to append child");
	    let leaf = sd.new_text(Rc::new(Value::from("leaf node")))
		.expect("unable to create text node");
	    l2.push(leaf.clone())
		.expect("unable to append child");
	    let mut aiter = leaf.ancestor_iter();
	    assert_eq!(aiter.next().unwrap().name().to_string(), "Level-2");
	    assert_eq!(aiter.next().unwrap().name().to_string(), "Level-1");
	    assert_eq!(aiter.next().unwrap().name().to_string(), "Test");
	    assert_eq!(aiter.next().unwrap().node_type(), NodeType::Document);
	    match aiter.next() {
		None => {},
		_ => panic!("iterator should have no more items")
	    }
	}
	#[test]
	fn item_node_owner_doc() {
	    let mut sd = $x();
	    let mut t = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1 = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Level-1"))))
		.expect("unable to create element");
	    t.push(l1.clone())
		.expect("unable to append child");
	    let mut l2 = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Level-2"))))
		.expect("unable to create element");
	    l1.push(l2.clone())
		.expect("unable to append child");
	    let leaf = sd.new_text(Rc::new(Value::from("leaf node")))
		.expect("unable to create text node");
	    l2.push(leaf.clone())
		.expect("unable to append child");
	    let od = leaf.owner_document();
	    assert_eq!(od.node_type(), NodeType::Document);
	}
	#[test]
	fn item_node_owner_doc_root() {
	    let mut sd = $x();
	    let mut t = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1 = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Level-1"))))
		.expect("unable to create element");
	    t.push(l1.clone())
		.expect("unable to append child");
	    let mut l2 = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Level-2"))))
		.expect("unable to create element");
	    l1.push(l2.clone())
		.expect("unable to append child");
	    let leaf = sd.new_text(Rc::new(Value::from("leaf node")))
		.expect("unable to create text node");
	    l2.push(leaf.clone())
		.expect("unable to append child");
	    let od = sd.owner_document();
	    assert_eq!(od.node_type(), NodeType::Document);
	}

	#[test]
	fn item_node_children() {
	    let mut sd = $x();
	    let mut t = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1 = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Level-1"))))
		.expect("unable to create element");
	    t.push(l1.clone())
		.expect("unable to append child");
	    let mut l2 = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Level-2"))))
		.expect("unable to create element");
	    t.push(l2.clone())
		.expect("unable to append child");
	    let leaf = sd.new_text(Rc::new(Value::from("leaf node")))
		.expect("unable to create text node");
	    t.push(leaf.clone())
		.expect("unable to append child");
	    let mut citer = t.child_iter();
	    assert_eq!(citer.next().unwrap().name().to_string(), "Level-1");
	    assert_eq!(citer.next().unwrap().name().to_string(), "Level-2");
	    assert_eq!(citer.next().unwrap().value().to_string(), "leaf node");
	    match citer.next() {
		None => {},
		_ => panic!("iterator should have no more items")
	    }
	}

	#[test]
	fn item_node_first_child() {
	    let mut sd = $x();
	    let mut t = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1 = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Level-1"))))
		.expect("unable to create element");
	    t.push(l1.clone())
		.expect("unable to append child");
	    let mut l2 = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Level-2"))))
		.expect("unable to create element");
	    t.push(l2.clone())
		.expect("unable to append child");
	    let leaf = sd.new_text(Rc::new(Value::from("leaf node")))
		.expect("unable to create text node");
	    t.push(leaf.clone())
		.expect("unable to append child");
	    match t.first_child() {
		Some(f) => assert_eq!(f.name().to_string(), "Level-1"),
		None => panic!("no first child")
	    }
	}

	#[test]
	fn item_node_descend() {
	    let mut sd = $x();
	    let mut t = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1a = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("A"))))
		.expect("unable to create element");
	    t.push(l1a.clone())
		.expect("unable to append child");
	    let mut l1b = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("B"))))
		.expect("unable to create element");
	    t.push(l1b.clone())
		.expect("unable to append child");

	    let mut l2aa = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("A"))))
		.expect("unable to create element");
	    l1a.push(l2aa.clone())
		.expect("unable to append child");
	    l2aa.push(
		sd.new_text(Rc::new(Value::from("AA")))
		    .expect("unable to create text")
	    ).expect("unable to append text");
	    let mut l2ab = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("B"))))
		.expect("unable to create element");
	    l1a.push(l2ab.clone())
		.expect("unable to append child");
	    l2ab.push(
		sd.new_text(Rc::new(Value::from("AB")))
		    .expect("unable to create text")
	    ).expect("unable to append text");

	    let mut l2ba = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("A"))))
		.expect("unable to create element");
	    l1b.push(l2ba.clone())
		.expect("unable to append child");
	    l2ba.push(
		sd.new_text(Rc::new(Value::from("BA")))
		    .expect("unable to create text")
	    ).expect("unable to append text");
	    let mut l2bb = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("B"))))
		.expect("unable to create element");
	    l1b.push(l2bb.clone())
		.expect("unable to append child");
	    l2bb.push(
		sd.new_text(Rc::new(Value::from("BB")))
		    .expect("unable to create text")
	    ).expect("unable to append text");

	    let mut diter = t.descend_iter();
	    assert_eq!(diter.next().unwrap().name().to_string(), "A");
	    assert_eq!(diter.next().unwrap().name().to_string(), "A");
	    assert_eq!(diter.next().unwrap().value().to_string(), "AA");
	    assert_eq!(diter.next().unwrap().name().to_string(), "B");
	    assert_eq!(diter.next().unwrap().value().to_string(), "AB");
	    assert_eq!(diter.next().unwrap().name().to_string(), "B");
	    assert_eq!(diter.next().unwrap().name().to_string(), "A");
	    assert_eq!(diter.next().unwrap().value().to_string(), "BA");
	    assert_eq!(diter.next().unwrap().name().to_string(), "B");
	    assert_eq!(diter.next().unwrap().value().to_string(), "BB");
	    match diter.next() {
		None => {},
		_ => panic!("iterator should have no more items")
	    }
	}

	#[test]
	fn item_node_next() {
	    let mut sd = $x();
	    let mut t = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1 = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Level-1"))))
		.expect("unable to create element");
	    t.push(l1.clone())
		.expect("unable to append child");
	    let mut l2 = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Level-2"))))
		.expect("unable to create element");
	    t.push(l2.clone())
		.expect("unable to append child");
	    let leaf = sd.new_text(Rc::new(Value::from("leaf node")))
		.expect("unable to create text node");
	    t.push(leaf.clone())
		.expect("unable to append child");
	    let mut niter = l1.next_iter();
	    assert_eq!(niter.next().unwrap().name().to_string(), "Level-2");
	    assert_eq!(niter.next().unwrap().value().to_string(), "leaf node");
	    match niter.next() {
		None => {},
		_ => panic!("iterator should have no more items")
	    }
	}

	#[test]
	fn item_node_prev() {
	    let mut sd = $x();
	    let mut t = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1 = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Level-1"))))
		.expect("unable to create element");
	    t.push(l1.clone())
		.expect("unable to append child");
	    let mut l2 = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Level-2"))))
		.expect("unable to create element");
	    t.push(l2.clone())
		.expect("unable to append child");
	    let leaf = sd.new_text(Rc::new(Value::from("leaf node")))
		.expect("unable to create text node");
	    t.push(leaf.clone())
		.expect("unable to append child");
	    let mut piter = leaf.prev_iter();
	    assert_eq!(piter.next().unwrap().name().to_string(), "Level-2");
	    assert_eq!(piter.next().unwrap().name().to_string(), "Level-1");
	    match piter.next() {
		None => {},
		_ => panic!("iterator should have no more items")
	    }
	}

	#[test]
	fn item_node_attr() {
	    let mut sd = $x();
	    let mut t = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let a1 = sd.new_attribute(
		Rc::new(QualifiedName::new(None, None, String::from("role"))),
		Rc::new(Value::from("testing"))
	    ).expect("unable to create attribute");
	    t.add_attribute(a1)
		.expect("unable to add attribute");
	    let a2 = sd.new_attribute(
		Rc::new(QualifiedName::new(None, None, String::from("phase"))),
		Rc::new(Value::from("one"))
	    ).expect("unable to create element");
	    t.add_attribute(a2)
		.expect("unable to add attribute");

	    // NB. attributes could be returned in a different order
	    assert!(
		sd.to_xml() == "<Test role='testing' phase='one'></Test>" ||
		    sd.to_xml() == "<Test phase='one' role='testing'></Test>"
	    );
	    let mut aiter = t.attribute_iter();
	    let v = aiter.next().unwrap().name().to_string();
	    if v == "role" {
		assert_eq!(aiter.next().unwrap().name().to_string(), "phase");
	    } else if v == "phase" {
		assert_eq!(aiter.next().unwrap().name().to_string(), "role");
	    } else {
		panic!("unexpected attribute value")
	    }
	    match aiter.next() {
		None => {},
		_ => panic!("iterator should have no more items")
	    }
	}

	#[test]
	fn item_node_shallow_copy_element() {
	    let mut sd = $x();
	    let mut t = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let l = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("content"))))
		.expect("unable to create element");
	    t.push(l)
		.expect("unable to append child");
	    let it = Item::Node(t.clone());
	    let u = it.shallow_copy().expect("unable to shallow copy element");
	    assert_eq!(t.to_xml(), "<Test><content></content></Test>");
	    assert_eq!(u.to_xml(), "<Test></Test>");
	}
		#[test]
		fn item_node_cmp_doc_order_1() {
		let sd = $z();
		let b1: Vec<RNode> = sd.descend_iter().filter(|n| n.get_attribute(&Rc::new(QualifiedName::new(None, None, String::from("id")))).to_string() == String::from("b1")).collect();
		let b9: Vec<RNode> = sd.descend_iter().filter(|n| n.get_attribute(&Rc::new(QualifiedName::new(None, None, String::from("id")))).to_string() == String::from("b9")).collect();
		assert_eq!(b1[0].cmp_document_order(&b9[0]), Ordering::Less)
	}
		#[test]
		fn item_node_cmp_doc_order_2() {
		let sd = $z();
		let b10: Vec<RNode> = sd.descend_iter().filter(|n| n.get_attribute(&Rc::new(QualifiedName::new(None, None, String::from("id")))).to_string() == String::from("b10")).collect();
		let b6: Vec<RNode> = sd.descend_iter().filter(|n| n.get_attribute(&Rc::new(QualifiedName::new(None, None, String::from("id")))).to_string() == String::from("b6")).collect();
		assert_eq!(b10[0].cmp_document_order(&b6[0]), Ordering::Greater)
	}

	#[test]
	fn item_node_partialeq_1_pos() {
	    let mut sd = $x();
	    let mut t = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let a1 = sd.new_attribute(
		Rc::new(QualifiedName::new(None, None, String::from("role"))),
		Rc::new(Value::from("testing"))
	    ).expect("unable to create attribute");
	    t.add_attribute(a1)
		.expect("unable to add attribute");
	    let a2 = sd.new_attribute(
		Rc::new(QualifiedName::new(None, None, String::from("phase"))),
		Rc::new(Value::from("one"))
	    ).expect("unable to create element");
	    t.add_attribute(a2)
		.expect("unable to add attribute");
		t.push(sd.new_text(Rc::new(Value::from("my test document"))).expect("unable to create text node"))
		.expect("unable to add text node");

		// The same document as above, but with attributes in a different order
	    let mut od = $x();
	    let mut u = od.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element");
	    od.push(u.clone())
		.expect("unable to append child");
	    let b1 = od.new_attribute(
		Rc::new(QualifiedName::new(None, None, String::from("role"))),
		Rc::new(Value::from("testing"))
	    ).expect("unable to create attribute");
	    let b2 = od.new_attribute(
		Rc::new(QualifiedName::new(None, None, String::from("phase"))),
		Rc::new(Value::from("one"))
	    ).expect("unable to create element");
	    u.add_attribute(b2)
		.expect("unable to add attribute");
		u.push(od.new_text(Rc::new(Value::from("my test document"))).expect("unable to create text node"))
		.expect("unable to add text node");
	    u.add_attribute(b1)
		.expect("unable to add attribute");

	    assert_eq!(sd == od, true)
	}
	#[test]
	fn item_node_partialeq_1_neg() {
	    let mut sd = $x();
	    let mut t = sd.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let a1 = sd.new_attribute(
		Rc::new(QualifiedName::new(None, None, String::from("role"))),
		Rc::new(Value::from("testing"))
	    ).expect("unable to create attribute");
	    t.add_attribute(a1)
		.expect("unable to add attribute");
	    let a2 = sd.new_attribute(
		Rc::new(QualifiedName::new(None, None, String::from("phase"))),
		Rc::new(Value::from("one"))
	    ).expect("unable to create element");
	    t.add_attribute(a2)
		.expect("unable to add attribute");
		t.push(sd.new_text(Rc::new(Value::from("my test document"))).expect("unable to create text node"))
		.expect("unable to add text node");

		// The same document as above, but with attributes in a different order
	    let mut od = $x();
	    let mut u = od.new_element(Rc::new(QualifiedName::new(None, None, String::from("Test"))))
		.expect("unable to create element");
	    od.push(u.clone())
		.expect("unable to append child");
	    let b1 = od.new_attribute(
		Rc::new(QualifiedName::new(None, None, String::from("role"))),
		Rc::new(Value::from("testing"))
	    ).expect("unable to create attribute");
	    let b2 = od.new_attribute(
		Rc::new(QualifiedName::new(None, None, String::from("phase"))),
		Rc::new(Value::from("one"))
	    ).expect("unable to create element");
	    u.add_attribute(b2)
		.expect("unable to add attribute");
		u.push(od.new_text(Rc::new(Value::from("not the same document"))).expect("unable to create text node"))
		.expect("unable to add text node");
	    u.add_attribute(b1)
		.expect("unable to add attribute");

	    assert_eq!(sd == od, false)
	}
    }
);
