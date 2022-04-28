use std::any::Any;
use crate::value::Value;
use crate::item::{
    Document,
    Node,
    AncestorIterator,
    ChildIterator,
    DocChildIterator,
    NodeType,
    OutputDefinition,
};
use crate::qname::QualifiedName;
use crate::xdmerror::{Error, ErrorKind};
use generational_arena::{Arena, Index};
use std::collections::HashMap;

/// A Tree, using an Arena Allocator.
/// A node in the tree is a [Leaf], which is just an arena Index.
/// Nodes can be detached, but not deleted
#[derive(Clone)]
pub struct Tree<N: Node> {
    a: Arena<NodeContent>,
    root: Option<N>,
    // TODO: prologue, epilogue, XML declaration, DTD
}

impl<N> Tree<N>
where
    N: Node
{
    pub fn new() -> Self {
        Tree {
            a: Arena::new(),
	    root: None,
        }
    }
}
impl<N> Tree<N>
where
    N: Node
{
    /// Return the [NodeContent] given a [Leaf]
    fn any_get(&self, r: &dyn Any) -> Option<&NodeContent> {
	r.downcast_ref::<Leaf>()
	    .and_then(|l| self.a.get(l.0))
    }
    fn get(&self, i: Leaf) -> Option<&NodeContent> {
	self.a.get(i.0)
    }
    /// Return a mutable [NodeContent] given a [Leaf]
    fn any_get_mut(&mut self, r: &dyn Any) -> Option<&mut NodeContent> {
	r.downcast_ref::<Leaf>()
	    .and_then(|l| self.a.get_mut(l.0))
    }
    fn get_mut(&mut self, i: Leaf) -> Option<&mut NodeContent> {
	self.a.get_mut(i.0)
    }
}

impl<N> Document<N> for Tree<N>
where
    N: Node
{
    type Node = Leaf;

    fn to_string(&self, _n: Option<N>) -> String {
	String::from("not implemented yet (document)")
    }
    fn to_xml(&self, n: Option<N>) -> String {
	n.map_or(
	    self.root.map_or(
		String::from(""),
		|r| self.to_xml(Some(r))
	    ),
	    |m| {
		let nc = self.any_get(m.as_any()).unwrap();	// TODO: Don't Panic
		match nc.node_type() {
		    NodeType::Element => {
			let mut result = String::from("<");
			let name = nc.name().unwrap();
			result.push_str(name.to_string().as_str());
			result.push_str(">");
//			let children = (*self).child_iter(m);
//			loop {
//			    match children.next(self) {
//				Some(c) => {
//				    result.push_str(self.to_xml(Some(c)).as_str())
//				}
//				None => break,
//			    }
//			}
			result.push_str("</");
			result.push_str(name.to_string().as_str());
			result.push_str(">");
			result
		    }
		    NodeType::Text => {
			nc.value().unwrap().to_string()
		    }
		    _ => {
			// TODO
			String::from("-- not implemented --")
		    }
		}
	    }
	)
    }
    fn to_xml_with_options(&self, _n: Option<N>, _od: &OutputDefinition) -> String {
	String::from("not implemented yet")
    }
    fn to_json(&self, _n: Option<N>) -> String {
	String::from("not implemented yet")
    }

    fn to_int(&self, n: Option<N>) -> Result<i64, Error> {
	// Convert to a string, then try parsing that as an integer
	n.map_or(
	    self.root.map_or(
		Result::Err(Error::new(ErrorKind::Unknown, String::from("document has no root element"))),
		|r| self.to_int(Some(r))
	    ),
	    |m| {
		self.to_string(Some(m)).parse::<i64>()
		    .map_err(|e| Error::new(ErrorKind::Unknown, e.to_string()))
	    }
	)
    }
    fn to_double(&self, n: Option<N>) -> f64 {
	// Convert to a string, then try parsing that as a double
	n.map_or(
	    self.root.map_or(
		f64::NAN,
		|r| self.to_double(Some(r))
	    ),
	    |m| {
		match self.to_string(Some(m)).parse::<f64>() {
		    Ok(f) => f,
		    Err(_) => f64::NAN,
		}
	    }
	)
    }
    fn to_name(&self, n: Option<N>) -> QualifiedName {
	n.map_or(
	    QualifiedName::new(None, None, String::from("")),
	    |m| self.any_get(m.as_any())
		.map_or(
		    QualifiedName::new(None, None, String::from("")),
		    |o| o.name().map_or(
			QualifiedName::new(None, None, String::from("")),
			|p| p,
		    )
		),
	)
    }

    fn node_type(&self, n: N) -> NodeType {
	self.any_get(n.as_any())
	    .map_or(
		NodeType::Unknown,
		|m| m.node_type(),
	    )
    }

    fn get_root_element(&self) -> Option<N> {
	self.root.as_ref().map(|r| r.clone())
    }
    fn set_root_element(&mut self, n: N) -> Result<(), Error> {
	if let Some(r) = self.any_get(n.as_any()) {
	    if r.node_type() == NodeType::Element {
		// TODO: check if the Tree already has a root element
		self.root = Some(n);
		Ok(())
	    } else {
		Result::Err(Error::new(
                    ErrorKind::Unknown,
                    String::from("cannot set the root element to a non-element type node"),
		))
	    }
	} else {
	    Result::Err(Error::new(
                ErrorKind::Unknown,
                String::from("invalid index"),
            ))
	}
    }

    fn new_element(&mut self, name: QualifiedName) -> Result<N, Error> {
	Ok(
	    Leaf::from(
		self.a
		    .insert(NodeBuilder::new(NodeType::Element).name(name).build())
	    )
	)
    }
    fn new_text(&mut self, c: Value) -> Result<N, Error> {
	Ok(
	    Leaf::from(
		self.a
		    .insert(NodeBuilder::new(NodeType::Text).value(c).build())
	    )
	)
    }
    fn new_attribute(&mut self, name: QualifiedName, v: Value) -> Result<N, Error> {
	Ok(
	    Leaf::from(
		self.a.insert(
		    NodeBuilder::new(NodeType::Attribute)
			.name(name)
			.value(v)
			.build(),
		)
	    )
	)
    }
    fn new_comment(&mut self, v: Value) -> Result<N, Error> {
        Ok(
	    Leaf::from(
		self.a
		    .insert(NodeBuilder::new(NodeType::Comment).value(v).build())
	    )
	)
    }
    fn new_processing_instruction(&mut self, name: QualifiedName, v: Value) -> Result<N, Error> {
        Ok(
	    Leaf::from(
		self.a.insert(
		    NodeBuilder::new(NodeType::ProcessingInstruction)
			.name(name)
			.value(v)
			.build(),
		)
	    )
	)
    }

    fn append_child(&mut self, p: N, c: N) -> Result<(), Error> {
        // TODO: p and c must be Indexes for this Tree
	// TODO: Don't Panic

	// Check that p is an element and that c is not an attribute
        if self.get(p).unwrap().node_type() != NodeType::Element {
            return Result::Err(Error::new(
                ErrorKind::Unknown,
                String::from("must be an element"),
            ));
        }
        if self.get(c).unwrap().node_type() == NodeType::Attribute {
            return Result::Err(Error::new(
                ErrorKind::Unknown,
                String::from("cannot append an attribute as a child"),
            ));
        }

	// TODO: detach c from wherever it currently is located

	// p will now be c's parent
	self.get_mut(c).unwrap().parent = Some(p);

	// Push c onto p's child list
        let x = self.get_mut(p).children.take();
        self.get_mut(p).children = Some(x.map_or(vec![c], |mut y| {
            y.push(c);
            y
        }));

        Ok(())
    }
    fn insert_before(&mut self, _child: N, _insert: N) -> Result<(), Error> {
        return Result::Err(Error::new(
            ErrorKind::NotImplemented,
            String::from("not yet implemented"),
        ));
    }

//    fn ancestor_iter<D: Document<N>>(&self, n: N) -> Box<dyn AncestorIterator<D, N, Item = N>> {
//	Box::new(Ancestors::new(n))
//    }
    fn parent(&self, n: N) -> Option<N> {
	None
//	self.ancestor_iter(n).next(self).map(|p| p)
    }
//    fn child_iter<D: Document<N>>(&self, n: N) -> Box<dyn ChildIterator<D, N, Item = N>> {
//	Box::new(Children::new(n))
//    }
//    fn doc_child_iter<D: Document<N>>(&self) -> Box<dyn DocChildIterator<D, N, Item = N>> {
	// TODO: support prologue and epilogue
//	Box::new(DocChildren::new())
//    }
    //fn descend_iter(&self, i: NodeRef) -> Descendants {
	//Descendants::new(self, i.1)
    //}
}

/// A node in the [Tree]. This is just a wrapper for Index, so that methods can be defined.
#[derive(Copy, Clone)]
struct Leaf(Index);

impl From<Index> for Leaf {
    fn from(i: Index) -> Self {
	Leaf(i)
    }
}

impl Node for Leaf {
    // Need the Tree to be able to find the NodeContent
    fn node_type(&self) -> NodeType {
	NodeType::Unknown
    }
}

#[derive(Clone, Default)]
pub struct NodeContent {
    t: NodeType,
    name: Option<QualifiedName>,
    v: Option<Value>,
    parent: Option<Index>, // The root node has no parent
    attributes: Option<HashMap<QualifiedName, Index>>, // non-empty nodes could just have empty HashMap, but I'm guessing an Option consumes less memory than a HashMap
    children: Option<Vec<Index>>, // non-element nodes could just have empty vector. See above for memory issue.
}

impl NodeContent {
    pub fn new(t: NodeType) -> Self {
        NodeContent {
	    t,
            ..Default::default()
        }
    }
    pub fn node_type(&self) -> NodeType {
	self.t
    }
    pub fn name(&self) -> &Option<QualifiedName> {
        &self.name
    }
    pub fn value(&self) -> &Option<Value> {
	&self.v
    }
}

struct NodeBuilder(NodeContent);

impl NodeBuilder {
    pub fn new(t: NodeType) -> Self {
        NodeBuilder(NodeContent::new(t))
    }
    pub fn name(mut self, qn: QualifiedName) -> Self {
        self.0.name = Some(qn);
        self
    }
    // Q: what to do if the node already has a value?
    // This implementation drops the previous value
    pub fn value(mut self, v: Value) -> Self {
        self.0.v = Some(v);
        self
    }
    pub fn build(self) -> NodeContent {
        self.0
    }
}

pub struct Ancestors {
    cur: Leaf,
}

impl Ancestors {
    fn new(cur: Leaf) -> Ancestors {
	Ancestors{cur}
    }
}

impl<D: Document<N>, N: Node> AncestorIterator<D, N> for Ancestors {
    type Item = N;

    fn next(&mut self, d: D) -> Option<Self::Item> {
	if let Some(c) = d.get(self.cur) {
	    c.parent.map(|p| {
		self.cur = p;
		p
	    })
	} else {
	    // not a valid Index for the arena
	    None
	}
    }
}

// This iterator assumes that the child list doesn't change
pub struct Children {
    parent: Leaf,
    cur: usize,
}

impl Children {
    fn new(parent: Leaf) -> Children {
	Children{parent: parent, cur: 0}
    }
}

impl<D: Document<N>, N: Node> ChildIterator<D, N> for Children {
    type Item = N;

    fn next(&mut self, d: D) -> Option<Self::Item> {
	if let Some(p) = d.get(self.parent) {
	    if p.children.is_none() {
		None
	    } else if self.cur < p.children.as_ref().unwrap().len() {
		self.cur += 1;
		Some(p.children.as_ref().unwrap()[self.cur - 1])
	    } else {
		None
	    }
	} else {
	    // parent is not a valid Index for the arena
	    None
	}
    }
}

// This iterator assumes that the child list doesn't change
pub struct DocChildren {
    cur: usize,
}

impl DocChildren {
    fn new() -> DocChildren {
	DocChildren{cur: 0}
    }
}

impl<D: Document<N>, N: Node> DocChildIterator<D, N> for DocChildren {
    type Item = N;

    // TODO: support prologoue and epilogue
    fn next(&mut self, d: D) -> Option<Self::Item> {
	d.root.map(|r| r)
    }
}

// Return all of the descendant nodes, not including self
// This iterator assumes that none of the child lists change
// NB. tried simply returning the iterator for the vector, but it is dynamically sized
// TODO: this implementation eagerly evaluates the descendants. This needs to be reimplemented as a lazy evaluator. The next item is either:
//	the first child
//	the next sibling
//	the next() of the parent
pub struct Descendants {
    cur: Vec<Index>,
    idx: Option<usize>,
}

impl Descendants {
    fn new<N: Node>(t: &Tree<N>, parent: Index) -> Descendants {
	Descendants{cur: gather_nodes(t, parent), idx: None}
    }
}

//impl Iterator for Descendants {
//    type Item = Index;
//
//    fn next(&mut self) -> Option<Self::Item> {
//	let n = self.idx.map_or(0, |i| i + 1);
//	self.idx = Some(n);
//	self.cur.get(self.idx.unwrap()).cloned()
//    }
//}

fn gather_nodes<N: Node>(t: &Tree<N>, p: Index) -> Vec<Index> {
    t.get(Leaf::from(p)).map_or(vec![], |n| {
	n.children.as_ref().map_or(vec![], |m| {
	    let mut result: Vec<Index> = vec![];
	    for i in m {
		let mut r: Vec<Index> = vec![*i];
		let mut g = gather_nodes(t, *i);
		r.append(&mut g);
		result.append(&mut r);
	    }
	    result
	})
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn noroot() {
        let n = Tree::<Leaf>::new();
        assert_eq!(n.get_root_element().is_none(), true);
    }
    #[test]
    fn root() {
        let mut n = Tree::<Leaf>::new();
        let m = n.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element");
        n.set_root_element(m);

	let rootnode = n.get(
	    n.get_root_element().unwrap()
	).unwrap();
	let qname = rootnode.name().clone().unwrap();
        assert_eq!(qname.get_localname(), "Test")
    }
    #[test]
    fn build_element() {
	let nb = NodeBuilder::new(NodeType::Element);
	let el = nb.name(QualifiedName::new(None, None, String::from("Test"))).build();
	let n = el.name().clone().unwrap().get_localname();
	assert_eq!(n, "Test")
    }
    #[test]
    fn append_element_child() {
        let mut n = Tree::<Leaf>::new();
        let m = n.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element");
        n.set_root_element(m);
        let e = n.new_element(QualifiedName::new(None, None, String::from("Child"))).expect("unable to create element");
        assert!(n.append_child(m, e).is_ok());
        assert_eq!(n.to_xml(None), "<Test><Child></Child></Test>")
    }
    #[test]
    fn append_text_child() {
        let mut n = Tree::<Leaf>::new();
        let m = n.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element");
        n.set_root_element(m);
        let t = n.new_text(Value::from("this is text")).expect("unable to create text node");
        assert!(n.append_child(m, t).is_ok());
        assert_eq!(n.to_xml(None), "<Test>this is text</Test>")
    }
    #[test]
    fn append_multi() {
        let mut n = Tree::<Leaf>::new();
        let m = n.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element");
	let mi = m.clone();
        n.set_root_element(m);
        (1..3).for_each(|i| {
            let e = n.new_element(QualifiedName::new(None, None, String::from("Level-1"))).expect("unable to create element");
	    let ei = e.clone();
            n.append_child(mi, e)
                .expect("failed to append element child");
            (1..3).for_each(|j| {
                let f = n.new_element(QualifiedName::new(None, None, String::from("Level-2"))).expect("unable to create element");
		let fi = f.clone();
                let g = n.new_text(Value::from(format!("node {}-{}", i, j))).expect("unable to create text node");
                n.append_child(fi, g).expect("unable to add text node");
                n.append_child(ei, f).expect("unable to add Level-2 element");
            });
        });

        assert_eq!(n.to_xml(None), "<Test><Level-1><Level-2>node 1-1</Level-2><Level-2>node 1-2</Level-2></Level-1><Level-1><Level-2>node 2-1</Level-2><Level-2>node 2-2</Level-2></Level-1></Test>")
    }
//    #[test]
//    fn ancestors() {
//        let mut n = Tree::<Leaf>::new();
//        let m = n.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element");
//        n.set_root_element(m);
//        let e = n.new_element(QualifiedName::new(None, None, String::from("Child1"))).expect("unable to create element");
//	let ei = e.clone();
//        assert!(n.append_child(m, ei).is_ok());
//        let f = n.new_element(QualifiedName::new(None, None, String::from("Child2"))).expect("unable to create element");
//	let fi = f.clone();
//        assert!(n.append_child(e, fi.to_node()).is_ok());
//        assert_eq!(n.to_xml(None), "<Test><Child1><Child2></Child2></Child1></Test>");
//	let mut pi = n.ancestor_iter(f);
//	let p = pi.next(&n).unwrap();
//	assert_eq!(n.get(
//	    Leaf::from_node(p.as_any()).expect("unable to translate to Leaf")
//	).unwrap().name().clone().unwrap().get_localname(), "Child1");
//	let q = pi.next(&n).unwrap();
//	assert_eq!(n.get(
//	    Leaf::from_node(q.as_any()).expect("unable to translate to Leaf")
//	).unwrap().name().clone().unwrap().get_localname(), "Test");
//	assert_eq!(pi.next(&n).is_none(), true)
//    }
//    #[test]
//    fn children() {
//        let mut n = Tree::<Leaf>::new();
//        let m = n.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element");
//	let mi = Leaf::from_node(m.as_any()).expect("unable to translate Leaf").clone();
  //      n.set_root_element(m.as_any());
//        let e = n.new_element(QualifiedName::new(None, None, String::from("Child1"))).expect("unable to create element");
//        assert!(n.append_child(mi.to_node(), e).is_ok());
//        let f = n.new_element(QualifiedName::new(None, None, String::from("Child2"))).expect("unable to create element");
//        assert!(n.append_child(mi.to_node(), f).is_ok());
//        let g = n.new_element(QualifiedName::new(None, None, String::from("Child3"))).expect("unable to create element");
//        assert!(n.append_child(mi.to_node(), g).is_ok());

//	let mut it = n.child_iter(m);
//	let c1 = it.next(&n).unwrap();
//	assert_eq!(n.get(
//	    Leaf::from_node(c1.as_any()).expect("unable to translate to Leaf")
//	).unwrap().name().clone().unwrap().get_localname(), "Child1");
//	let c2 = it.next(&n).unwrap();
//	assert_eq!(n.get(
//	    Leaf::from_node(c2.as_any()).expect("unable to translate to Leaf")
//	).unwrap().name().clone().unwrap().get_localname(), "Child2");
//	let c3 = it.next(&n).unwrap();
//	assert_eq!(n.get(
//	    Leaf::from_node(c3.as_any()).expect("unable to translate to Leaf")
//	).unwrap().name().clone().unwrap().get_localname(), "Child3");
//	assert_eq!(it.next(&n).is_none(), true)
//    }
//    #[test]
//    fn descendants_none() {
//        let mut n = Tree::new();
//        let m = n.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element");
//        n.set_root_element(m.as_any());
//	let mut it = n.descend_iter(m);
//	assert_eq!(it.next(n), None)
//    }
//    #[test]
//    fn descendants_many() {
//        let mut n = Tree::new();
//        let m = n.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element");
//        n.set_root_element(m.as_any());
//        let e = n.new_element(QualifiedName::new(None, None, String::from("Child1-1"))).expect("unable to create element");
//        assert!(n.append_child(m, e).is_ok());
//        let f = n.new_element(QualifiedName::new(None, None, String::from("Child1-2"))).expect("unable to create element");
//        assert!(n.append_child(m, f).is_ok());
//        let g = n.new_element(QualifiedName::new(None, None, String::from("Child2-1"))).expect("unable to create element");
//        assert!(n.append_child(e, g).is_ok());
//        let h = n.new_element(QualifiedName::new(None, None, String::from("Child2-2"))).expect("unable to create element");
//        assert!(n.append_child(e, h).is_ok());
//        let i = n.new_element(QualifiedName::new(None, None, String::from("Child2-3"))).expect("unable to create element");
//        assert!(n.append_child(f, i).is_ok());
//        let j = n.new_element(QualifiedName::new(None, None, String::from("Child2-4"))).expect("unable to create element");
//        assert!(n.append_child(f, j).is_ok());
//
//	let mut it = n.descend_iter(m);
//	let d1 = it.next(n).unwrap();
//	assert_eq!(n.get(d1).unwrap().name().clone().unwrap().get_localname(), "Child1-1");
//	let d2 = it.next(n).unwrap();
//	assert_eq!(n.get(d2).unwrap().name().clone().unwrap().get_localname(), "Child2-1");
//	let d3 = it.next(n).unwrap();
//	assert_eq!(n.get(d3).unwrap().name().clone().unwrap().get_localname(), "Child2-2");
//	let d4 = it.next(n).unwrap();
//	assert_eq!(n.get(d4).unwrap().name().clone().unwrap().get_localname(), "Child1-2");
//	let d5 = it.next(n).unwrap();
//	assert_eq!(n.get(d5).unwrap().name().clone().unwrap().get_localname(), "Child2-3");
//	let d6 = it.next(n).unwrap();
//	assert_eq!(n.get(d6).unwrap().name().clone().unwrap().get_localname(), "Child2-4");
//	assert_eq!(it.next(n), None)
//    }

    #[bench]
    fn bench_ga(b: &mut Bencher) {
        b.iter(|| {
            let mut n = Tree::<Leaf>::new();
            let m = n.new_element(QualifiedName::new(None, None, String::from("Test"))).expect("unable to create element");
	    let mi = m.clone();
            n.set_root_element(m);
            (1..3).for_each(|i| {
                let e = n.new_element(QualifiedName::new(None, None, String::from("Level-1"))).expect("unable to create element");
		let ei = e.clone();

                n.append_child(mi, e)
                    .expect("failed to append element child");
                (1..3).for_each(|j| {
                    let f = n.new_element(QualifiedName::new(None, None, String::from("Level-2"))).expect("unable to create element");
		    let fi = f.clone();
                    let g = n.new_text(Value::from(format!("node {}-{}", i, j))).expect("unable to create text node");
                    n.append_child(fi, g).expect("unable to add text node");
                    n.append_child(ei, f).expect("unable to add Level-2 element");
                });
            });
            n
        })
    }
}
