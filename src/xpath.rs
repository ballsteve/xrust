//! # xdm::xpath
//!
//! An XPath parser as a nom parser combinator.

extern crate nom;
use decimal;
use std::rc::Rc;
use std::collections::HashMap;
use nom:: {
  IResult,
  character::complete::*,
  branch::alt,
  character::complete::{char, none_of},
  sequence::{delimited, pair, tuple},
  multi::{many0, separated_list, separated_nonempty_list},
  combinator::{complete, map, opt, recognize},
  bytes::complete::tag,
};
use crate::item::*;
use crate::xdmerror::*;
use crate::parsecommon::*;
use crate::evaluate::{
    DynamicContext,
    evaluate,
    static_analysis,
    Constructor,
    NameTest, WildcardOrName,
    NodeTest, NodeMatch, KindTest,
    Axis,
    ArithmeticOperator, ArithmeticOperand,
    Function,
    format_constructor,
    StaticContext,
};
use roxmltree::Node;

// Expr ::= ExprSingle (',' ExprSingle)* ;
// we need to unpack each primary_expr
fn expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    separated_nonempty_list(
      tuple((multispace0, tag(","), multispace0)),
      expr_single
    ),
    |v| {
      let mut s = Vec::new();
      for i in v {
        for j in i {
          s.push(j)
	}
      }
      s
    }
  )
  (input)
}

// ExprSingle ::= ForExpr | LetExpr | QuantifiedExpr | IfExpr | OrExpr
fn expr_single(input: &str) -> IResult<&str, Vec<Constructor>> {
  alt((
    or_expr,
    let_expr,
    for_expr,
    if_expr,
    // TODO: other branches
  ))
  (input)
}

// IfExpr ::= 'if' '(' Expr ')' 'then' ExprSingle 'else' ExprSingle
fn if_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map(
    tuple((
      tag("if"),
      multispace0,
      tag("("),
      multispace0,
      expr,
      multispace0,
      tag(")"),
      multispace0,
      tag("then"),
      multispace0,
      expr_single,
      multispace0,
      tag("else"),
      multispace0,
      expr_single,
    )),
    |(_, _, _, _, i, _, _, _, _, _, t, _, _, _, e)| {
      vec![
        Constructor::Switch(
	  vec![
	    i, t,
	  ],
	  e
	)
      ]
    }
  )
  (input)
}

// ForExpr ::= SimpleForClause 'return' ExprSingle
fn for_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map(
    tuple((
      simple_for_clause,
      tuple((multispace0, tag("return"), multispace0)),
      expr_single
    )),
    |(f, _, e)| {
      vec![Constructor::Loop(f, e)]
    }
  )
  (input)
}

// SimpleForClause ::= 'for' SimpleForBinding (',' SimpleForBinding)*
// SimpleForBinding ::= '$' VarName 'in' ExprSingle
fn simple_for_clause(input: &str) -> IResult<&str, Vec<Constructor>> {
  map(
    tuple((
      tag("for"),
      multispace0,

      separated_nonempty_list(
        tuple((multispace0, tag(","), multispace0)),
	tuple((
	  tag("$"),
	  qname,
	  multispace0,
          tag("in"),
	  multispace0,
	  expr_single,
	)),
      )
    )),
    |(_, _, b)| {
      b.iter()
        .map(|(_, v, _, _, _, e)| Constructor::VariableDeclaration(get_nt_localname(v), e.to_vec()))
	.collect()
    }
  )
  (input)
}

// LetExpr ::= SimpleLetClause 'return' ExprSingle
fn let_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map(
    tuple((
      simple_let_clause,
      tuple((multispace0, tag("return"), multispace0)),
      expr_single
    )),
    |(mut l, _, mut e)| {
      // Variable declaration
      // expression
      l.append(&mut e);
      l
    }
  )
  (input)
}

// SimpleLetClause ::= 'let' SimpleLetBinding (',' SimpleLetBinding)*
// SimpleLetBinding ::= '$' VarName ':=' ExprSingle
fn simple_let_clause(input: &str) -> IResult<&str, Vec<Constructor>> {
  map(
    tuple((
      tag("let"),
      multispace0,

      separated_nonempty_list(
        tuple((multispace0, tag(","), multispace0)),
	tuple((
	  tag("$"),
	  qname,
	  multispace0,
          tag(":="),
	  multispace0,
	  expr_single,
	)),
      )
    )),
    |(_, _, b)| {
      b.iter()
        .map(|(_, v, _, _, _, e)| Constructor::VariableDeclaration(get_nt_localname(v), e.to_vec()))
	.collect()
    }
  )
  (input)
}

fn get_nt_localname(nt: &NodeTest) -> String {
  match nt {
    NodeTest::Name(NameTest{name: Some(WildcardOrName::Name(localpart)), ns: None, prefix: None}) => localpart.to_string(),
    _ => String::from("invalid qname")
  }
}

// OrExpr ::= AndExpr ('or' AndExpr)*
fn or_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    separated_nonempty_list(
      tuple((multispace0, tag("or"), multispace0)),
      and_expr
    ),
    |v: Vec<Vec<Constructor>>| {
      if v.len() == 1 {
        let mut s = Vec::new();
        for i in v {
          for j in i {
            s.push(j)
	  }
	}
	s
      } else {
        vec![Constructor::Or(v)]
      }
    }
  )
  (input)
}

// AndExpr ::= ComparisonExpr ('and' ComparisonExpr)*
fn and_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    separated_nonempty_list(
      tuple((multispace0, tag("and"), multispace0)),
      comparison_expr
    ),
    |v: Vec<Vec<Constructor>>| {
      if v.len() == 1 {
        let mut s = Vec::new();
        for i in v {
          for j in i {
            s.push(j)
	  }
	}
	s
      } else {
        vec![Constructor::And(v)]
      }
    }
  )
  (input)
}

// ComparisonExpr ::= StringConcatExpr ( (ValueComp | GeneralComp | NodeComp) StringConcatExpr)?
fn comparison_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    pair (
      stringconcat_expr,
      opt(
        pair(
	  alt((
	    tuple((multispace0, tag("="), multispace0)),
	    tuple((multispace0, tag("!="), multispace0)),
	    tuple((multispace0, tag("<"), multispace0)),
	    tuple((multispace0, tag("<="), multispace0)),
	    tuple((multispace0, tag(">"), multispace0)),
	    tuple((multispace0, tag(">="), multispace0)),
	    tuple((multispace0, tag("eq"), multispace0)),
	    tuple((multispace0, tag("ne"), multispace0)),
	    tuple((multispace0, tag("lt"), multispace0)),
	    tuple((multispace0, tag("le"), multispace0)),
	    tuple((multispace0, tag("gt"), multispace0)),
	    tuple((multispace0, tag("ge"), multispace0)),
	    tuple((multispace0, tag("is"), multispace0)),
	    tuple((multispace0, tag("<<"), multispace0)),
	    tuple((multispace0, tag(">>"), multispace0)),
	  )),
	  stringconcat_expr,
	)
      ),
    ),
    |(v, o)| {
      match o {
        None => v,
	Some(((_a, b, _c), t)) => {
	  match b {
	    "=" => vec![Constructor::GeneralComparison(Operator::Equal, vec![v, t])],
	    "!=" => vec![Constructor::GeneralComparison(Operator::NotEqual, vec![v, t])],
	    "<" => vec![Constructor::GeneralComparison(Operator::LessThan, vec![v, t])],
	    "<=" => vec![Constructor::GeneralComparison(Operator::LessThanEqual, vec![v, t])],
	    ">" => vec![Constructor::GeneralComparison(Operator::GreaterThan, vec![v, t])],
	    ">=" => vec![Constructor::GeneralComparison(Operator::GreaterThanEqual, vec![v, t])],
	    "eq" => vec![Constructor::ValueComparison(Operator::Equal, vec![v, t])],
	    "ne" => vec![Constructor::ValueComparison(Operator::NotEqual, vec![v, t])],
	    "lt" => vec![Constructor::ValueComparison(Operator::LessThan, vec![v, t])],
	    "le" => vec![Constructor::ValueComparison(Operator::LessThanEqual, vec![v, t])],
	    "gt" => vec![Constructor::ValueComparison(Operator::GreaterThan, vec![v, t])],
	    "ge" => vec![Constructor::ValueComparison(Operator::GreaterThanEqual, vec![v, t])],
	    "is" => vec![Constructor::ValueComparison(Operator::Is, vec![v, t])],	//
	    "<<" => vec![Constructor::ValueComparison(Operator::Before, vec![v, t])],	// TODO: use appropriate constructor
	    ">>" => vec![Constructor::ValueComparison(Operator::After, vec![v, t])],	//
	    _ => vec![] // Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not a valid comparison operator")}),
	  }
	},
      }
    }
  )
  (input)
}

// StringConcatExpr ::= RangeExpr ( '||' RangeExpr)*
fn stringconcat_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    separated_nonempty_list(
      tuple((multispace0, tag("||"), multispace0)),
      range_expr
    ),
    |v| {
      if v.len() == 1 {
        let mut s = Vec::new();
      	for i in v {
            for j in i {
              s.push(j)
	    }
        }
        s
      } else {
        vec![Constructor::Concat(v)]
      }
    }
  )
  (input)
}

// RangeExpr ::= AdditiveExpr ( 'to' AdditiveExpr)?
fn range_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    pair(
      additive_expr,
      opt(
        tuple((
	  tuple((multispace0, tag("to"), multispace0)),
	  additive_expr,
	))
      )
    ),
    |(v, o)| {
      match o {
        None => v,
	Some((_t, u)) => {
          vec![Constructor::Range(vec![v, u])]
	}
      }
    }
  )
  (input)
}

// For additive and multiplicative expressions,
// passing the expression to be operated upon to the evaluator
// is quite awkward.
// TODO: find a better way

// AdditiveExpr ::= MultiplicativeExpr ( ('+' | '-') MultiplicativeExpr)*
fn additive_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    pair(
      multiplicative_expr,
      many0(
        tuple((
          alt((
            tuple((multispace0, tag("+"), multispace0)),
	    tuple((multispace0, tag("-"), multispace0)),
          )),
          multiplicative_expr,
	))
      )
    ),
    |(a, b)| {
      if b.len() == 0 {
        a
      } else {
        // The arguments to the constructor are the items to be summed
	// These are pair-wise items: first is the operator as a string literal,
	// second is the value
	let mut r: Vec<ArithmeticOperand> = Vec::new();

	r.push(ArithmeticOperand{op: ArithmeticOperator::Noop, operand: a});

	for ((_x, c, _y), d) in b {
	  r.push(ArithmeticOperand{op: ArithmeticOperator::from(c), operand: d});
	}
        vec![Constructor::Arithmetic(r)]
      }
    }
  )
  (input)
}
// MultiplicativeExpr ::= UnionExpr ( ('*' | 'div' | 'idiv' | 'mod') UnionExpr)*
fn multiplicative_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    pair(
      union_expr,
      many0(
        tuple((
	  alt((
	    tuple((multispace0, tag("*"), multispace0)),
	    tuple((multispace0, tag("div"), multispace0)),
	    tuple((multispace0, tag("idiv"), multispace0)),
	    tuple((multispace0, tag("mod"), multispace0)),
	  )),
	  union_expr,
	))
      )
    ),
    |(a, b)| {
      if b.len() == 0 {
        a
      } else {
        // The arguments to the constructor are the items to be summed
	// These are pair-wise items: first is the operator as a string literal,
	// second is the value
	let mut r: Vec<ArithmeticOperand> = Vec::new();

	r.push(ArithmeticOperand{op: ArithmeticOperator::Noop, operand: a});

	for ((_x, c, _y), d) in b {
	  r.push(ArithmeticOperand{op: ArithmeticOperator::from(c), operand: d});
	}
        vec![Constructor::Arithmetic(r)]
      }
    }
  )
  (input)
}

// UnionExpr ::= IntersectExceptExpr ( ('union' | '|') IntersectExceptExpr)*
fn union_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    separated_nonempty_list(
      alt((
        tuple((multispace0, tag("union"), multispace0)),
        tuple((multispace0, tag("|"), multispace0)),
      )),
      intersectexcept_expr
    ),
    |v| {
      if v.len() == 1 {
        let mut s = Vec::new();
      	for i in v {
            for j in i {
              s.push(j)
	    }
        }
        s
      } else {
        vec![Constructor::NotImplemented]
      }
    }
  )
  (input)
}

// IntersectExceptExpr ::= InstanceOfExpr ( ('intersect' | 'except') InstanceOfExpr)*
fn intersectexcept_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    pair(
      instanceof_expr,
      many0(
        tuple((
	  alt((
	    tuple((multispace0, tag("intersect"), multispace0)),
	    tuple((multispace0, tag("except"), multispace0)),
	  )),
	  instanceof_expr,
	))
      )
    ),
    |(a, b)| {
      if b.len() == 0 {
        a
      } else {
        // The arguments to the intersectexcept function are the sequences to be operated upon.
	// These are pair-wise items: first is the operator as a string literal,
	// second is the value
//	let mut r = Vec::new();

//        r.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String("".to_string()))))]);
//	r.push(a);

//	for ((_x, c, _y), d) in b {
//	  r.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String(c.to_string()))))]);
//	  r.push(d);
//	}
//        vec![SequenceConstructor::new(cons_intersectexcept).set_args(Some(r))]
        vec![Constructor::NotImplemented]
      }
    }
  )
  (input)
}

// InstanceOfExpr ::= TreatExpr ( 'instance' 'of' SequenceType)?
fn instanceof_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    pair(
      treat_expr,
      opt(
        tuple((multispace0, tag("instance"), multispace0, tag("of"), multispace0, sequencetype_expr)),
      )
    ),
    |(u, v)| {
      match v {
        None => {
	  u
	}
	Some(_t) => {
	  //let mut r = Vec::new();
	  //r.push(u);
	  //let (_a, _b, _c, _d, _e, st) = t;
	  //r.push(st);
	  //vec![SequenceConstructor::new(cons_instanceof).set_args(Some(r))]
          vec![Constructor::NotImplemented]
	}
      }
    }
  )
  (input)
}

// SequenceType ::= ( 'empty-sequence' '(' ')' | (ItemType OccurrenceIndicator?)
// TODO: implement this parser fully
fn sequencetype_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    tag("empty-sequence()"),
    |_v| {
      Vec::new()
    }
  )
  (input)
}

// TreatExpr ::= CastableExpr ( 'treat' 'as' SequenceType)?
fn treat_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    pair(
      castable_expr,
      opt(
        tuple((multispace0, tag("treat"), multispace0, tag("as"), multispace0, sequencetype_expr)),
      )
    ),
    |(u, v)| {
      match v {
        None => {
	  u
	}
	Some(_t) => {
	  //let mut r = Vec::new();
	  //r.push(u);
	  //let (_a, _b, _c, _d, _e, st) = t;
	  //r.push(st);
	  //vec![SequenceConstructor::new(cons_treat).set_args(Some(r))]
          vec![Constructor::NotImplemented]
	}
      }
    }
  )
  (input)
}

// CastableExpr ::= CastExpr ( 'castable' 'as' SingleType)?
fn castable_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    pair(
      cast_expr,
      opt(
        tuple((multispace0, tag("castable"), multispace0, tag("as"), multispace0, singletype_expr)),
      )
    ),
    |(u, v)| {
      match v {
        None => {
	  u
	}
	Some(_t) => {
	  //let mut r = Vec::new();
	  //r.push(u);
	  //let (_a, _b, _c, _d, _e, st) = t;
	  //r.push(st);
	  //vec![SequenceConstructor::new(cons_castable).set_args(Some(r))]
          vec![Constructor::NotImplemented]
	}
      }
    }
  )
  (input)
}

// SingleType ::= SimpleTypeName '?'?
// SimpleTypeName ::= TypeName
// TypeName ::= EQName
// EQName ::= QName | URIQualifiedName
// URIQualifiedName ::= BracedURILiteral NCName
// QName ::= PrefixedName | UnprefixedName
// PrefixedName ::= Prefix ':' LocalPart
// UnprefixedName ::= LocalPart
// Prefix ::= NCName
// LocalPart ::= NCName
// NCName ::= Name - (Char* ':' Char*)
// Char ::= #x9 | #xA |#xD | [#x20-#xD7FF] | [#xE000-#xFFFD | [#x10000-#x10FFFF]
// TODO: implement this parser fully
fn singletype_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    pair(
      qname,
      opt(
        tuple((multispace0, tag("?"), multispace0)),
      )
    ),
    |(_u, _v)| {
      Vec::new()
    }
  )
  (input)
}

// CastExpr ::= ArrowExpr ( 'cast' 'as' SingleType)?
fn cast_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    pair(
      arrow_expr,
      opt(
        tuple((multispace0, tag("cast"), multispace0, tag("as"), multispace0, singletype_expr)),
      )
    ),
    |(u, v)| {
      match v {
        None => {
	  u
	}
	Some(_t) => {
	  //let mut r = Vec::new();
	  //r.push(u);
	  //let (_a, _b, _c, _d, _e, st) = t;
	  //r.push(st);
	  //vec![SequenceConstructor::new(cons_cast).set_args(Some(r))]
          vec![Constructor::NotImplemented]
	}
      }
    }
  )
  (input)
}

// ArrowExpr ::= UnaryExpr ( '=>' ArrowFunctionSpecifier ArgumentList)*
fn arrow_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    pair (
      unary_expr,
      many0(
        tuple((
	  multispace0,
	  tag("=>"),
	  multispace0,
	  arrowfunctionspecifier,
	  multispace0,
	  opt(argumentlist)
	))
      )
    ),
    |(u, v)| {
      if v.len() == 0 {
        u
      } else {
        //vec![SequenceConstructor::new(cons_arrow)]
        vec![Constructor::NotImplemented]
      }
    }
  )
  (input)
}

// ArrowFunctionSpecifier ::= EQName | VarRef | ParenthesizedExpr
// TODO: finish this parser with EQName and VarRef
fn arrowfunctionspecifier(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    alt((
      qname_expr,
      parenthesized_expr
    )),
    |_v| {
      Vec::new()
    }
  )
  (input)
}
fn qname_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    qname,
    |q| {
      match q {
        NodeTest::Name(NameTest{name: Some(WildcardOrName::Name(localpart)), ns: None, prefix: None}) => {
	  vec![Constructor::Literal(Value::StringOwned(localpart))]
	}
        _ => {
      	  vec![Constructor::Literal(Value::String("invalid qname"))]
	}
      }
    }
  )
  (input)
}

// ArgumentList ::= '(' (Argument (',' Argument)*)? ')'
// TODO: finish this parser with actual arguments
fn argumentlist(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    tag("()"),
    //tuple((
      //tag("("),
      //multispace0,
      //tag(")"),
    //)),
    |_v| {
      Vec::new()
    }
  )
  (input)
}

// UnaryExpr ::= ('-' | '+')* ValueExpr
fn unary_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    pair (
      many0(
        alt((
	  tag("-"),
	  tag("+"),
	))
      ),
      value_expr,
    ),
    |(u, v)| {
      if u.len() == 0 {
        v
      } else {
        //let mut a = Vec::new();
	//for i in u {
	  //a.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String(String::from(i)))))]);
	//}
	//a.push(v);
        //vec![SequenceConstructor::new(cons_unary).set_args(Some(a))]
        vec![Constructor::NotImplemented]
      }
    }
  )
  (input)
}

// ValueExpr (SimpleMapExpr) ::= PathExpr ('!' PathExpr)*
fn value_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    pair(
      path_expr,
      many0(
        tuple((
	  tag("!"),
	  path_expr,
	))
      )
    ),
    |(u, v)| {
      if v.len() == 0 {
        u
      } else {
        //let mut s = Vec::new();
	//s.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String(String::from("")))))]);
	//s.push(u);
	//for (a, b) in v {
	  //s.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String(String::from(a)))))]);
	  //s.push(b);
	//}
        //vec![SequenceConstructor::new(cons_simplemap).set_args(Some(s))]
        vec![Constructor::NotImplemented]
      }
    }
  )
  (input)
}

// PathExpr ::= ('/' RelativePathExpr?) | ('//' RelativePathExpr) | RelativePathExpr
fn path_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  alt((
    absolute_descendant_expr,
    absolute_path_expr,
    relativepath_expr,
  ))
  (input)
}
// ('/' RelativePathExpr?)
fn absolute_path_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map(
    pair(
      tag("/"),
      opt(relativepath_expr),
    ),
    |(_u, v)| {
      match v {
        Some(a) => {
	  if a.len() == 1 {
	    match &a[0] {
	      Constructor::Path(w) => {
	        let mut x = vec![vec![Constructor::Root]];
		for y in w {
		  x.push(y.to_vec())
		}
	        vec![Constructor::Path(x)]
	      }
	      _ => {
		vec![Constructor::Path(vec![vec![Constructor::Root], a])]
	      }
	    }
	  } else {
	    // Error
	    println!("absolute_path_expr: relativepath_expr returned more than one constructor");
	    vec![]
	  }
	}
	None => {
	  vec![Constructor::Root]
	}
      }
    }
  )
  (input)
}
// ('//' RelativePathExpr)
fn absolute_descendant_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map(
    pair(
      tag("//"),
      relativepath_expr,
    ),
    |(_u, _v)| {
      vec![Constructor::Root,
	Constructor::NotImplemented]
	// TODO: process v to implement descendant-or-self
    }
  )
  (input)
}

// RelativePathExpr ::= StepExpr (('/' | '//') StepExpr)*
fn relativepath_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    pair (
      step_expr,
      many0(
        tuple((
	  alt((
	    tuple((multispace0, tag("//"), multispace0)),
	    tuple((multispace0, tag("/"), multispace0)),
	  )),
	  step_expr,
	))
      )
    ),
    |(a, b)| {
      if b.len() == 0 {
        a
      } else {
        let mut r = Vec::new();

        r.push(a);

	for ((_x, c, _y), d) in b {
	  match c {
	    "/" => {
	      r.push(d)
	    }
	    _ => {
	      // Insert a descendant-or-self::* step
	      r.push(vec![Constructor::Step(NodeMatch{axis: Axis::DescendantOrSelf, nodetest: NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Wildcard)})}, vec![])]);
	      r.push(d)
	    }
	  }
	}

        vec![Constructor::Path(r)]
      }
    }
  )
  (input)
}
// For debugging: a version of the above function that steps through the parsing
fn relativepath_expr_dbg(newinput: &str) -> IResult<&str, Vec<Constructor>> {
  let myin = newinput;
  //println!("relpath: starting with \"{}\"", myin);
  let (myin, a) = step_expr(myin)?;
  let mut r = Vec::new();

  r.push(vec![Constructor::Literal(Value::String(""))]);
  r.push(a);

  //println!("relpath: parsed first step. input=\"{}\"", myin);

  loop {
    //println!("looking for delimiter in \"{}\"", myin);
    if myin.len() == 0 {
      //println!("no more input");
      break
    }
    let (myin, (_x, c, _y)) = alt((
      tuple((multispace0, tag("//"), multispace0)),
      tuple((multispace0, tag("/"), multispace0)),
    ))(myin)?;
    //println!("got delimiter \"{}\", remaining input \"{}\"", c, myin);
    r.push(vec![Constructor::Literal(Value::String(c))]);

    let (_myin, d) = step_expr(myin)?;
    //println!("got next step");
    r.push(d);
    break;
  }

  //println!("relpath: finished");
  Ok((myin, vec![Constructor::NotImplemented]))
}

// StepExpr ::= PostfixExpr | AxisStep
fn step_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  //println!("step_expr: input \"{}\"", input);
  alt((
    postfix_expr, // These two return different objects; we need to switch between them
    axisstep      // TODO: define an enum that allows us to do the switch
  ))
  (input)
}

// AxisStep ::= (ReverseStep | ForwardStep) PredicateList
// TODO: predicates
fn axisstep(input: &str) -> IResult<&str, Vec<Constructor>> {
  map(
    pair(
      alt((
        reversestep,
    	forwardstep
      )),
      predicate_list
    ),
    |(a, p)| {
      if p.is_empty() {
        a
      } else {
        // Insert predicates into step
	if a.len() == 1 {
	  match &a[0] {
	    Constructor::Step(nm, _) => {
	      vec![Constructor::Step(nm.clone(), p)]
	    }
	    _ => {
	      // error
	      vec![]
	    }
	  }
	} else {
	  // error
	  vec![]
	}
      }
    }
  )
  (input)
}

// PredicateList ::= Predicate*
fn predicate_list(input: &str) -> IResult<&str, Vec<Vec<Constructor>>> {
  many0(predicate)
  (input)
}

// Predicate ::= '[' Expr ']'
fn predicate(input: &str) -> IResult<&str, Vec<Constructor>> {
  map(
    tuple((
      tag("["),
      expr,
      tag("]"),
    )),
    |(_, e, _)| {
      e
    }
  )
  (input)
}

// ForwardStep ::= (ForwardAxis NodeTest) | AbbrevForwardStep
// TODO: abbreviated step
fn forwardstep(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    pair(
      forwardaxis,
      nodetest
    ),
    |(a, n)| {
      vec![Constructor::Step(
        NodeMatch{axis: Axis::from(a), nodetest: n},
        vec![]
      )]
    }
  )
  (input)
}
// ReverseStep ::= (ReverseAxis NodeTest) | AbbrevReverseStep
// TODO: abbreviated step
fn reversestep(input: &str) -> IResult<&str, Vec<Constructor>> {
  map (
    pair(
      reverseaxis,
      nodetest
    ),
    |(a, n)| {
      vec![Constructor::Step(
        NodeMatch{axis: Axis::from(a), nodetest: n},
	vec![]
      )]
    }
  )
  (input)
}

// ForwardAxis ::= ('child' | 'descendant' | 'attribute' | 'self' | 'descendant-or-self' | 'following-sibling' | 'following' | 'namespace') '::'
fn forwardaxis(input: &str) -> IResult<&str, &str> {
  map (
    pair(
      alt((
        tag("child"),
        tag("descendant-or-self"),
        tag("descendant"),
        tag("attribute"),
        tag("self"),
        tag("following-sibling"),
        tag("following"),
        tag("namespace"),
      )),
      tag("::"),
    ),
    |(a, _b)| {
      a
    }
  )
  (input)
}
// ReverseAxis ::= ('parent' | 'ancestor' | 'ancestor-or-self' | 'preceding' | 'preceding-sibling') '::'
fn reverseaxis(input: &str) -> IResult<&str, &str> {
  map (
    pair(
      alt((
        tag("parent"),
        tag("ancestor-or-self"),
        tag("ancestor"),
        tag("preceding-sibling"),
        tag("preceding"),
      )),
      tag("::"),
    ),
    |(a, _b)| {
      a
    }
  )
  (input)
}

fn qname(input: &str) -> IResult<&str, NodeTest> {
  alt((
    prefixed_name,
    unprefixed_name,
  ))
  (input)
}
fn unprefixed_name(input: &str) -> IResult<&str, NodeTest> {
  map (
    ncname,
    |localpart| {
      NodeTest::Name(NameTest{ns: None, prefix: None, name: Some(WildcardOrName::Name(String::from(localpart)))})
    }
  )
  (input)
}
fn prefixed_name(input: &str) -> IResult<&str, NodeTest> {
  map (
    tuple((
      ncname,
      tag(":"),
      ncname
    )),
    |(prefix, _, localpart)| {
      NodeTest::Name(NameTest{ns: None, prefix: Some(String::from(prefix)), name: Some(WildcardOrName::Name(String::from(localpart)))})
    }
  )
  (input)
}

// NodeTest ::= KindTest | NameTest
fn nodetest(input: &str) -> IResult<&str, NodeTest> {
  alt((
    kindtest,
    nametest,
  ))
  (input)
}

// KindTest ::= DocumentTest | ElementTest | AttributeTest | SchemaElementTest | SchemaAttributeTest | PITest | CommentTest | TextTest | NamespaceNodeTest | AnyKindTest
fn kindtest(input: &str) -> IResult<&str, NodeTest> {
  alt((
    documenttest,
    elementtest,
    attributetest,
    schemaelementtest,
    schemaattributetest,
    pitest,
    commenttest,
    texttest,
    namespacenodetest,
    anykindtest,
  ))
  (input)
}
// DocumentTest ::= 'document-node' '(' (ElementTest | SchemaElementTest)? ')'
// TODO: capture the element test
fn documenttest(input: &str) -> IResult<&str, NodeTest> {
  map(
    tuple((
      tag("document-node"),
      multispace0,
      tag("("),
      opt(alt((elementtest, schemaelementtest))),
      multispace0,
      tag(")"),
    )),
    |(_, _, _, _t, _, _)| {
      NodeTest::Kind(KindTest::DocumentTest)
    }
  )
  (input)
}
// ElementTest ::= 'element' '(' (ElementNameOrWildcard (',' TypeName '?'?)?)? ')'
// TODO: capture element name or wildcard, typename
fn elementtest(input: &str) -> IResult<&str, NodeTest> {
  map(
    tuple((
      tag("element"),
      multispace0,
      tag("("),
      multispace0,
      tag(")"),
    )),
    |(_, _, _, _, _)| {
      NodeTest::Kind(KindTest::ElementTest)
    }
  )
  (input)
}
// AttributeTest ::= 'attribute' '(' (AttribNameOrWildcard (',' TypeName)?)? ')'
// TODO: capture attribnameOrWildcard and typename
fn attributetest(input: &str) -> IResult<&str, NodeTest> {
  map(
    tuple((
      tag("attribute"),
      multispace0,
      tag("("),
      multispace0,
      tag(")"),
    )),
    |(_, _, _, _, _)| {
      NodeTest::Kind(KindTest::AttributeTest)
    }
  )
  (input)
}
// SchemaElementTest ::= 'schema-element' '(' ElementDeclaration ')'
// TODO: capture elementDeclaration
fn schemaelementtest(input: &str) -> IResult<&str, NodeTest> {
  map(
    tuple((
      tag("schema-element"),
      multispace0,
      tag("("),
      multispace0,
      tag(")"),
    )),
    |(_, _, _, _, _)| {
      NodeTest::Kind(KindTest::SchemaElementTest)
    }
  )
  (input)
}
// SchemaAttributeTest ::= 'schema-attribute' '(' AttributeDeclaration ')'
// TODO: capture attribute declaration
fn schemaattributetest(input: &str) -> IResult<&str, NodeTest> {
  map(
    tuple((
      tag("schema-attribute"),
      multispace0,
      tag("("),
      multispace0,
      tag(")"),
    )),
    |(_, _, _, _, _)| {
      NodeTest::Kind(KindTest::SchemaAttributeTest)
    }
  )
  (input)
}
// PITest ::= 'processing-instruction' '(' (NCName | StringLiteral)? ')'
// TODO: capture PI name
fn pitest(input: &str) -> IResult<&str, NodeTest> {
  map(
    tuple((
      tag("processing-instruction"),
      multispace0,
      tag("("),
      multispace0,
      tag(")"),
    )),
    |(_, _, _, _, _)| {
      NodeTest::Kind(KindTest::PITest)
    }
  )
  (input)
}
// CommentTest ::= 'comment' '(' ')'
fn commenttest(input: &str) -> IResult<&str, NodeTest> {
  map(
    tuple((
      tag("comment"),
      multispace0,
      tag("("),
      multispace0,
      tag(")"),
    )),
    |(_, _, _, _, _)| {
      NodeTest::Kind(KindTest::CommentTest)
    }
  )
  (input)
}
// TextTest ::= 'text' '(' ')'
fn texttest(input: &str) -> IResult<&str, NodeTest> {
  map(
    tuple((
      tag("text"),
      multispace0,
      tag("("),
      multispace0,
      tag(")"),
    )),
    |(_, _, _, _, _)| {
      NodeTest::Kind(KindTest::TextTest)
    }
  )
  (input)
}
// NamespaceNodeTest ::= 'namespace-node' '(' ')'
fn namespacenodetest(input: &str) -> IResult<&str, NodeTest> {
  map(
    tuple((
      tag("namespace-node"),
      multispace0,
      tag("("),
      multispace0,
      tag(")"),
    )),
    |(_, _, _, _, _)| {
      NodeTest::Kind(KindTest::NamespaceNodeTest)
    }
  )
  (input)
}
// AnyKindTest := 'node' '(' ')'
fn anykindtest(input: &str) -> IResult<&str, NodeTest> {
  map(
    tuple((
      tag("node"),
      multispace0,
      tag("("),
      multispace0,
      tag(")"),
    )),
    |(_, _, _, _, _)| {
      NodeTest::Kind(KindTest::AnyKindTest)
    }
  )
  (input)
}
// NameTest ::= EQName | Wildcard
// TODO: allow EQName rather than QName
fn nametest(input: &str) -> IResult<&str, NodeTest> {
  alt((
    qname,
    wildcard,
  ))
  (input)
}

// Wildcard ::= '*' | (NCName ':*') | ('*:' NCName) | (BracedURILiteral '*')
// TODO: more specific wildcards
fn wildcard(input: &str) -> IResult<&str, NodeTest> {
  map(
    tag("*"),
    |_w| {
      NodeTest::Name(NameTest{ns: Some(WildcardOrName::Wildcard), prefix: None, name: Some(WildcardOrName::Wildcard)})
    }
  )
  (input)
}

// PostfixExpr ::= PrimaryExpr (Predicate | ArgumentList | Lookup)*
// TODO: predicates, arg list, lookup
fn postfix_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  primary_expr(input)
}

// PrimaryExpr ::= Literal | VarRef | ParenthesizedExpr | ContextItemExpr | FunctionCall | FunctionItemExpr | MapConstructor | ArrayConstructor | UnaryLookup
// TODO: finish this parser
fn primary_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  alt((
    literal,
    context_item,
    parenthesized_expr,
    function_call,
    variable_reference,
  ))
  (input)
}

// VarRef ::= '$' VarName
fn variable_reference(input: &str) -> IResult<&str, Vec<Constructor>> {
  map(
    pair(
      tag("$"),
      qname
    ),
    |(_, v)| {
      vec![Constructor::VariableReference(get_nt_localname(&v))]
    }
  )
  (input)
}

// FunctionCall ::= EQName ArgumentList
fn function_call(input: &str) -> IResult<&str, Vec<Constructor>> {
  map(
    pair(
      qname,
      arglist,
    ),
    |(n, a)| {
      match n {
        NodeTest::Name(NameTest{name: Some(WildcardOrName::Name(localpart)), ns: None, prefix: None}) => {
      	  vec![Constructor::FunctionCall(
            Function::new(localpart, vec![], None),
	    a
      	  )]
	}
	_ => {
      	  vec![Constructor::Literal(Value::String("invalid qname"))]
	}
      }
    }
  )
  (input)
}

// ArgumentList ::= '(' (Argument (',' Argument)*)? ')'
fn arglist(input: &str) -> IResult<&str, Vec<Vec<Constructor>>> {
  map(
    tuple((
      tag("("),
      separated_list(
        tuple((multispace0, tag(","), multispace0)),
      	argument,
      ),
      tag(")"),
    )),
    |(_, a, _)| {
      a
    }
  )
  (input)
}

// Argument ::= ExpreSingle | ArgumentPlaceHolder
// TODO: ArgumentPlaceHolder
fn argument(input: &str) -> IResult<&str, Vec<Constructor>> {
  expr_single(input)
}

// Literal ::= NumericLiteral | StringLiteral
fn literal(input: &str) -> IResult<&str, Vec<Constructor>> {
  alt((
    numeric_literal ,
    string_literal
  ))
  (input)
}

// NumericLiteral ::= IntegerLiteral | DecimalLiteral | DoubleLiteral
fn numeric_literal(input: &str) -> IResult<&str, Vec<Constructor>> {
  alt((
    double_literal,
    decimal_literal,
    integer_literal,
  ))
  (input)
}
// IntegerLiteral ::= Digits
fn integer_literal(input: &str) -> IResult<&str, Vec<Constructor>> {
  map(digit1, |s: &str| {
    let n = s.parse::<i64>().unwrap();
    vec![Constructor::Literal(Value::Integer(n))]
  })
  (input)
}
// DecimalLiteral ::= ('.' Digits) | (Digits '.' [0-9]*)
// Construct a double, but if that fails fall back to decimal
fn decimal_literal(input: &str) -> IResult<&str, Vec<Constructor>> {
  map(
    alt((
      recognize(complete(pair(tag("."), digit1))),
      recognize(complete(tuple((digit1, tag("."), digit0)))),
    )),
    |s: &str| {
      let n = s.parse::<f64>();
      let i = match n {
        Ok(m) => Value::Double(m),
	Err(_) => Value::Decimal(decimal::d128!(s)),
      };
      vec![Constructor::Literal(i)]
    }
  )
  (input)
}
// DoubleLiteral ::= (('.' Digits) | (Digits ('.' [0-9]*)?)) [eE] [+-]? Digits
// Construct a double
fn double_literal(input: &str) -> IResult<&str, Vec<Constructor>> {
  map(
    recognize(
      tuple((
        alt((
          recognize(complete(pair(tag("."), digit1))),
          recognize(complete(tuple((digit1, tag("."), digit0)))),
        )),
	one_of("eE"),
	opt(one_of("+-")),
	digit1
      ))
    ),
    |s: &str| {
      let n = s.parse::<f64>();
      let i = match n {
        Ok(m) => Value::Double(m),
	Err(_) => panic!("unable to convert to double"),
      };
      vec![Constructor::Literal(i)]
    }
  )
  (input)
}

// StringLiteral ::= double- or single-quote delimited with double-delimiter escape
fn string_literal_double(input: &str) -> IResult<&str, String> {
  delimited(
    char('"'),
    map(
      many0(
        alt((
          map(
            tag("\"\""),
            |_| '"'
	  )
	  ,
	  none_of("\"")
        ))
      ),
      |v| v.iter().collect::<String>()
    ),
    char('"')
  )
  (input)
}
fn string_literal_single(input: &str) -> IResult<&str, String> {
  delimited(
    char('\''),
    map(
      many0(
        alt((
          map(
            tag("''"),
            |_| '\''
	  )
          ,
	  none_of("'")
        ))
      ),
      |v| v.iter().collect::<String>()
    ),
    char('\'')
  )
  (input)
}
fn string_literal(input: &str) -> IResult<&str, Vec<Constructor>> {
  map(
    alt((
      string_literal_double ,
      string_literal_single
    )),
    |s| vec![Constructor::Literal(Value::StringOwned(s))]
  )
  (input)
}
// ContextItemExpr ::= '.'
fn context_item(input: &str) -> IResult<&str, Vec<Constructor>> {
  map(
    tag("."),
    |_| vec![Constructor::ContextItem]
  )
  (input)
}
// ParenthesizedExpr ::= '(' Expr? ')'
fn parenthesized_expr(input: &str) -> IResult<&str, Vec<Constructor>> {
  delimited(
    tag("("),
    map(
      opt(expr),
      |e| match e {
        Some(v) => v,
        None => Vec::new()
      }
    ),
    tag(")")
  )
  (input)
}

pub fn parse(e: &str) -> Result<Vec<Constructor>, Error> {
  match expr(e) {
    Ok((rest, value)) => {
      if rest == "" {
        Result::Ok(value)
      } else {
        Result::Err(Error{kind: ErrorKind::Unknown, message: String::from(format!("extra characters after expression: \"{}\"", rest))})
      }
    },
    Err(nom::Err::Error(c)) => Result::Err(Error{kind: ErrorKind::Unknown, message: format!("parser error: {:?}", c)}),
    Err(nom::Err::Incomplete(_)) => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("incomplete input")}),
    Err(nom::Err::Failure(_)) => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("unrecoverable parser error")}),
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Parses to a singleton integer sequence constructor
    #[test]
    fn nomxpath_parse_int() {
        let e = parse("1").expect("failed to parse expression \"1\"");
	if e.len() == 1 {
	  let s = evaluate(&DynamicContext::new(), None, None, &e).expect("unable to evaluate sequence constructor");
	  if s.len() == 1 {
	    assert_eq!(s[0].to_int().unwrap(), 1)
	  } else {
	    panic!("sequence is not a singleton")
	  }
	} else {
	  panic!("constructor is not a single item")
	}
    }
    // Parses to a singleton double/decimal sequence constructor
    #[test]
    fn nomxpath_parse_decimal() {
        let e = parse("1.2").expect("failed to parse expression \"1.2\"");
	if e.len() == 1 {
	  let s = evaluate(&DynamicContext::new(), None, None, &e).expect("unable to evaluate sequence constructor");
	  if s.len() == 1 {
	    assert_eq!(s[0].to_double(), 1.2)
	  } else {
	    panic!("sequence is not a singleton")
	  }
	} else {
	  panic!("constructor is not a single item")
	}
    }
    // Parses to a singleton double sequence constructor
    #[test]
    fn nomxpath_parse_double() {
        let e = parse("1.2e2").expect("failed to parse expression \"1.2e2\"");
	if e.len() == 1 {
	  let s = evaluate(&DynamicContext::new(), None, None, &e).expect("unable to evaluate sequence constructor");
	  if s.len() == 1 {
	     assert_eq!(s[0].to_double(), 120.0)
	  } else {
	    panic!("sequence is not a singleton")
	  }
	} else {
	  panic!("constructor is not a single item")
	}
    }
    //#[test]
    //fn nomxpath_parse_double() {
        //assert_eq!(parse("2.0").unwrap(), Box::new(Value::Double(2.0)));
    //}
    // Parses to a singleton string
    #[test]
    fn nomxpath_parse_string_apos() {
        let e = parse("'abc'").expect("failed to parse expression \"'abc'\"");
	if e.len() == 1 {
	  let s = evaluate(&DynamicContext::new(), None, None, &e).expect("unable to evaluate sequence constructor");
	  if s.len() == 1 {
	     assert_eq!(s[0].to_string(), "abc")
	  } else {
	    panic!("sequence is not a singleton")
	  }
	} else {
	  panic!("constructor is not a single item")
	}
    }
    // Parses to a singleton string
    #[test]
    fn nomxpath_parse_string_apos_esc() {
        let e = parse("'abc''def'").expect("failed to parse expression \"'abc''def'\"");
	if e.len() == 1 {
	  let s = evaluate(&DynamicContext::new(), None, None, &e).expect("unable to evaluate sequence constructor");
	  if s.len() == 1 {
	     assert_eq!(s[0].to_string(), "abc'def")
	  } else {
	    panic!("sequence is not a singleton")
	  }
	} else {
	  panic!("constructor is not a single item")
	}
    }
    // Parses to a singleton string
    #[test]
    fn nomxpath_parse_string_quot() {
        let e = parse(r#""abc""#).expect("failed to parse expression \"\"abc\"\"");
	if e.len() == 1 {
	  let s = evaluate(&DynamicContext::new(), None, None, &e).expect("unable to evaluate sequence constructor");
	  if s.len() == 1 {
	     assert_eq!(s[0].to_string(), "abc")
	  } else {
	    panic!("sequence is not a singleton")
	  }
	} else {
	  panic!("constructor is not a single item")
	}
    }
    // Parses to a singleton string
    #[test]
    fn nomxpath_parse_string_quot_esc() {
        let e = parse(r#""abc""def""#).expect("failed to parse expression \"\"abc\"\"def\"\"");
	if e.len() == 1 {
	  let s = evaluate(&DynamicContext::new(), None, None, &e).expect("unable to evaluate sequence constructor");
	  if s.len() == 1 {
	     assert_eq!(s[0].to_string(), r#"abc"def"#)
	  } else {
	    panic!("sequence is not a singleton")
	  }
	} else {
	  panic!("constructor is not a single item")
	}
    }
    #[test]
    fn nomxpath_parse_literal_sequence() {
        let e = parse("1,'abc',2").expect("failed to parse \"1,'abc',2\"");
	if e.len() == 3 {
	  let s = evaluate(&DynamicContext::new(), None, None, &e).expect("unable to evaluate sequence constructor");
	  if s.len() == 3 {
	     assert_eq!(s[0].to_int().unwrap(), 1);
	     assert_eq!(s[1].to_string(), r#"abc"#);
	     assert_eq!(s[2].to_int().unwrap(), 2);
	  } else {
	    panic!("sequence does not have 3 items")
	  }
	} else {
	  panic!("constructor does not have 3 items")
	}
    }
    #[test]
    fn nomxpath_parse_literal_seq_ws() {
        let e = parse("1 , 'abc', 2").expect("failed to parse \"1 , 'abc', 2\"");
	if e.len() == 3 {
	  let s = evaluate(&DynamicContext::new(), None, None, &e).expect("unable to evaluate sequence constructor");
	  if s.len() == 3 {
	     assert_eq!(s[0].to_int().unwrap(), 1);
	     assert_eq!(s[1].to_string(), r#"abc"#);
	     assert_eq!(s[2].to_int().unwrap(), 2);
	  } else {
	    panic!("sequence does not have 3 items")
	  }
	} else {
	  panic!("constructor does not have 3 items")
	}
    }

    // Parses to a singleton context item sequence constructor
    #[test]
    fn nomxpath_parse_context_item() {
        let e = parse(".").expect("failed to parse expression \".\"");
	if e.len() == 1 {
	  let ctxt = vec![Rc::new(Item::Value(Value::String("foobar")))];
	  let s = evaluate(&DynamicContext::new(), Some(ctxt), Some(0), &e).expect("unable to evaluate sequence constructor");
	  if s.len() == 1 {
	    assert_eq!(s[0].to_string(), "foobar")
	  } else {
	    panic!("sequence is not a singleton")
	  }
	} else {
	  panic!("constructor is not a singleton")
	}
    }

    #[test]
    fn nomxpath_parse_empty() {
        let e = parse("()").expect("failed to parse expression \"()\"");
	assert_eq!(e.len(), 0)
    }
    #[test]
    fn nomxpath_parse_singleton_paren() {
        let e = parse("(1)").expect("failed to parse expression \"(1)\"");
	if e.len() == 1 {
	  let s = evaluate(&DynamicContext::new(), None, None, &e).expect("unable to evaluate sequence constructor");
	  if s.len() == 1 {
	     assert_eq!(s[0].to_int().unwrap(), 1)
	  } else {
	    panic!("sequence is not a singleton")
	  }
	} else {
	  panic!("constructor is not a single item")
	}
    }

    #[test]
    fn nomxpath_parse_union() {
        let e = parse("'a' | 'b'").expect("failed to parse expression \"'a' | 'b'\"");
	if e.len() == 1 {
	  assert!(true) // TODO: check the sequence constructor
	} else {
	  panic!("sequence is not a singleton")
	}
    }

    #[test]
    fn nomxpath_parse_intersectexcept() {
        let e = parse("'a' intersect 'b' except 'c'").expect("failed to parse expression \"'a' intersect 'b' except 'c'\"");
	if e.len() == 1 {
	  assert!(true) // TODO: check the sequence constructor
	} else {
	  panic!("sequence is not a singleton")
	}
    }

    #[test]
    fn nomxpath_parse_instanceof() {
        let e = parse("'a' instance of empty-sequence()").expect("failed to parse expression \"'a' instance of empty-sequence()\"");
	if e.len() == 1 {
	  assert!(true) // TODO: check the sequence constructor
	} else {
	  panic!("sequence is not a singleton")
	}
    }

    #[test]
    fn nomxpath_parse_treat() {
        let e = parse("'a' treat as empty-sequence()").expect("failed to parse expression \"'a' treat as empty-sequence()\"");
	if e.len() == 1 {
	  assert!(true) // TODO: check the sequence constructor
	} else {
	  panic!("sequence is not a singleton")
	}
    }

    #[test]
    fn nomxpath_parse_castable() {
        let e = parse("'a' castable as type").expect("failed to parse expression \"'a' castable as type\"");
	if e.len() == 1 {
	  assert!(true) // TODO: check the sequence constructor
	} else {
	  panic!("sequence is not a singleton")
	}
    }

    #[test]
    fn nomxpath_parse_cast() {
        let e = parse("'a' cast as type").expect("failed to parse expression \"'a' cast as type\"");
	if e.len() == 1 {
	  assert!(true) // TODO: check the sequence constructor
	} else {
	  panic!("sequence is not a singleton")
	}
    }

    #[test]
    fn nomxpath_parse_arrow() {
        let e = parse("'a' => spec()").expect("failed to parse expression \"'a' => spec()\"");
	if e.len() == 1 {
	  assert!(true) // TODO: check the sequence constructor
	} else {
	  panic!("sequence is not a singleton")
	}
    }

    #[test]
    fn nomxpath_parse_unary() {
        let e = parse("+'a'").expect("failed to parse expression \"+'a'\"");
	if e.len() == 1 {
	  assert!(true) // TODO: check the sequence constructor
	} else {
	  panic!("sequence is not a singleton")
	}
    }

    #[test]
    fn nomxpath_parse_simplemap() {
        let e = parse("'a'!'b'").expect("failed to parse expression \"'a'!'b'\"");
	if e.len() == 1 {
	  assert!(true) // TODO: check the sequence constructor
	} else {
	  panic!("sequence is not a singleton")
	}
    }

    #[test]
    fn xnode_root() {
      let d = roxmltree::Document::parse("<Level1><Level2>one</Level2><Level2>two</Level2><Level2>three</Level2></Level1>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap()))];
      let c = parse("/").expect("unable to parse XPath \"/\"");
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &c)
        .expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_string(), "<Level1><Level2>one</Level2><Level2>two</Level2><Level2>three</Level2></Level1>");
      } else {
        panic!("sequence does not have 1 item: \"{}\"", e.to_string())
      }
    }
    #[test]
    fn nomxpath_parse_root_step_1() {
        let _e = parse("/child::a").expect("failed to parse expression \"/child::a\"");
	assert!(true) // TODO: check the sequence constructor
    }
    #[test]
    fn xnode_step_1() {
      let d = roxmltree::Document::parse("<Level1><Level2>one</Level2><Level2>two</Level2><Level2>three</Level2></Level1>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap()))];
      let c = parse("/child::*").expect("failed to parse expression \"/child::*\"");
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &c)
        .expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_string(), "<Level1><Level2>one</Level2><Level2>two</Level2><Level2>three</Level2></Level1>");
      } else {
        println!("parsed constructor:\n{}\n", format_constructor(&c, 0));
        panic!("sequence does not have 1 item, it has {}: \"{}\"", e.len(), e.to_string())
      }
    }
    #[test]
    fn nomxpath_parse_root_step_2() {
        let _e = parse("/child::a/child::b").expect("failed to parse expression \"/child::a/child::b\"");
	assert!(true) // TODO: check the sequence constructor
    }
    #[test]
    fn xnode_step_nodetest_pos() {
      let d = roxmltree::Document::parse("<Level1/>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap()))];
      let c = parse("/child::Level1").expect("failed to parse expression \"/child::Level1\"");
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &c)
        .expect("evaluation failed");
      if e.len() == 1 {
        assert_eq!(e[0].to_string(), "<Level1/>");
      } else {
        println!("parsed constructor:\n{}\n", format_constructor(&c, 0));
        panic!("sequence does not have 1 item, it has {}: \"{}\"", e.len(), e.to_string())
      }
    }
    #[test]
    fn xnode_step_nodetest_neg() {
      let d = roxmltree::Document::parse("<Level1/>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap()))];
      let c = parse("/child::Test").expect("failed to parse expression \"/child::Test\"");
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &c)
        .expect("evaluation failed");
      if e.len() == 0 {
        assert!(true)
      } else {
        println!("parsed constructor:\n{}\n", format_constructor(&c, 0));
        panic!("sequence has more than 1 item, it has {}: \"{}\"", e.len(), e.to_string())
      }
    }
    #[test]
    fn xnode_step_2() {
      let d = roxmltree::Document::parse("<Level1><Level2>one</Level2><Level2>two</Level2><Level2>three</Level2></Level1>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap()))];
      let c = parse("/child::*/child::*").expect("failed to parse expression \"/child::a/child::b\"");
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &c)
        .expect("evaluation failed");
      if e.len() == 3 {
        assert_eq!(e[0].to_string(), "<Level2>one</Level2>");
        assert_eq!(e[1].to_string(), "<Level2>two</Level2>");
        assert_eq!(e[2].to_string(), "<Level2>three</Level2>");
      } else {
        println!("parsed constructor:\n{}\n", format_constructor(&c, 0));
        panic!("sequence does not have 3 items, it has {}: \"{}\"", e.len(), e.to_string())
      }
    }
    #[test]
    fn xnode_descendant_1() {
      let d = roxmltree::Document::parse("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap()))];
      let c = parse("descendant::*").expect("failed to parse expression \"descendant::*\"");
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 6);
      assert_eq!(e[1].to_string(), "<level3>1 1 1</level3>")
    }
    #[test]
    fn xnode_descendantorself_1() {
      let d = roxmltree::Document::parse("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap()))];
      let c = parse("descendant-or-self::*").expect("failed to parse expression \"descendant-or-self::*\"");
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 7);
      assert_eq!(e[2].to_string(), "<level3>1 1 1</level3>")
    }
    #[test]
    fn xnode_ancestor_1() {
      let d = roxmltree::Document::parse("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap().first_child().unwrap().first_child().unwrap()))];
      let c = parse("ancestor::*").expect("failed to parse expression \"ancestor::*\"");
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 3);
    }
    #[test]
    fn xnode_ancestororself_1() {
      let d = roxmltree::Document::parse("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap().first_child().unwrap()))];
      let c = parse("ancestor-or-self::*").expect("failed to parse expression \"ancestor-or-self::*\"");
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 3);
    }
    #[test]
    fn xnode_followingsibling_1() {
      let d = roxmltree::Document::parse("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap().first_child().unwrap().first_child().unwrap()))];
      let c = parse("following-sibling::*").expect("failed to parse expression \"following-sibling::*\"");
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 1);
      assert_eq!(e.to_string(), "<level3>1 1 2</level3>");
    }
    #[test]
    fn xnode_precedingsibling_1() {
      let d = roxmltree::Document::parse("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap().first_child().unwrap().last_child().unwrap()))];
      let c = parse("preceding-sibling::*").expect("failed to parse expression \"preceding-sibling::*\"");
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 1);
      assert_eq!(e.to_string(), "<level3>1 1 1</level3>");
    }
    #[test]
    fn xnode_following_1() {
      let d = roxmltree::Document::parse("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap().first_child().unwrap().first_child().unwrap()))];
      let c = parse("following::*").expect("failed to parse expression \"following::*\"");
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 4);
      assert_eq!(e.to_string(), "<level2><level3>1 2 1</level3><level3>1 2 2</level3></level2><level3>1 2 1</level3><level3>1 2 2</level3><level1>not me</level1>");
    }
    #[test]
    fn xnode_preceding_1() {
      let d = roxmltree::Document::parse("<Test><level1><level2><level3>1 1 1</level3><level3>1 1 2</level3></level2><level2><level3>1 2 1</level3><level3>1 2 2</level3></level2></level1><level1>not me</level1></Test>").expect("failed to parse XML");
      let s = vec![Rc::new(Item::XNode(d.root().first_child().unwrap().last_child().unwrap()))];
      let c = parse("preceding::*").expect("failed to parse expression \"preceding::*\"");
      let e = evaluate(&DynamicContext::new(), Some(s), Some(0), &c)
        .expect("evaluation failed");
      assert_eq!(e.len(), 7);
      assert_eq!(e[0].to_name(), "level1");
      assert_eq!(e[1].to_name(), "level2");
      assert_eq!(e[2].to_name(), "level3");
      assert_eq!(e[2].to_string(), "<level3>1 1 1</level3>");
    }
    #[test]
    fn nomxpath_parse_desc_or_self_1() {
        let _e = parse("//child::a").expect("failed to parse expression \"//child::a\"");
	assert!(true) // TODO: check the sequence constructor
    }
    #[test]
    fn nomxpath_parse_desc_or_self_2() {
        let _e = parse("//child::a/child::b").expect("failed to parse expression \"//child::a/child::b\"");
	assert!(true) // TODO: check the sequence constructor
    }
    #[test]
    fn nomxpath_parse_desc_or_self_3() {
        let _e = parse("//child::a//child::b").expect("failed to parse expression \"//child::a//child::b\"");
	assert!(true) // TODO: check the sequence constructor
    }
    #[test]
    fn nomxpath_parse_relative_path_1() {
        let e = parse("child::a/child::b").expect("failed to parse expression \"child::a/child::b\"");
	if e.len() == 1 {
	  assert!(true) // TODO: check the sequence constructor
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    #[test]
    fn nomxpath_parse_relative_path_2() {
        let _e = parse("child::a//child::b").expect("failed to parse expression \"child::a//child::b\"");
	assert!(true) // TODO: check the sequence constructor
    }

    #[test]
    fn nomxpath_parse_step_1() {
        let _e = parse("child::a").expect("failed to parse expression \"child::a\"");
	assert!(true) // TODO: check the sequence constructor
    }
    #[test]
    fn nomxpath_parse_step_2() {
        let _e = parse("child::bc").expect("failed to parse expression \"child::bc\"");
	assert!(true) // TODO: check the sequence constructor
    }
    #[test]
    fn nomxpath_parse_step_wild() {
        let _e = parse("child::*").expect("failed to parse expression \"child::*\"");
	assert!(true) // TODO: check the sequence constructor
    }

    #[test]
    fn parse_eval_predicate_pos() {
      let x = roxmltree::Document::parse("<Test><a><b/></a><a><c/></a></Test>").expect("failed to parse XML");
      let d = vec![Rc::new(Item::XNode(x.root().first_child().unwrap()))];
      let e = parse("/child::*/child::*[child::b]").expect("failed to parse expression \"//child::*/child::*[child::b]\"");
      let s = evaluate(&DynamicContext::new(), Some(d), Some(0), &e).expect("evaluation failed");
      assert_eq!(s.len(), 1);
      assert_eq!(s.to_string(), "<a><b/></a>")
    }
    #[test]
    fn parse_eval_predicate_neg() {
      let x = roxmltree::Document::parse("<Test><a><b/></a><a><c/></a></Test>").expect("failed to parse XML");
      let d = vec![Rc::new(Item::XNode(x.root().first_child().unwrap()))];
      let e = parse("/child::*[child::b]").expect("failed to parse expression \"/child::*[child::b]\"");
      let s = evaluate(&DynamicContext::new(), Some(d), Some(0), &e).expect("evaluation failed");
      assert_eq!(s.len(), 0)
    }
    #[test]
    fn parse_eval_fncall_position() {
      let x = roxmltree::Document::parse("<Test><a><b/></a><a><c/></a></Test>").expect("failed to parse XML");
      let d = vec![Rc::new(Item::XNode(x.root().first_child().unwrap()))];
      let mut e = parse("/child::*/child::*[position() eq 1]").expect("failed to parse expression \"/child::*/child::*[position() eq 1]\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), Some(d), Some(0), &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "<a><b/></a>")
    }
    #[test]
    fn parse_eval_fncall_last() {
      let x = roxmltree::Document::parse("<Test><a><b/></a><a><c/></a><a><d/></a></Test>").expect("failed to parse XML");
      let d = vec![Rc::new(Item::XNode(x.root().first_child().unwrap()))];
      let mut e = parse("/child::*/child::*[position() eq last()]").expect("failed to parse expression \"/child::*/child::*[position() eq last()]\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), Some(d), Some(0), &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "<a><d/></a>")
    }
    #[test]
    fn parse_eval_fncall_count() {
      let x = roxmltree::Document::parse("<Test><a><b/></a><a><c/></a><a><d/></a></Test>").expect("failed to parse XML");
      let d = vec![Rc::new(Item::XNode(x.root().first_child().unwrap()))];
      let mut e = parse("count(/child::*/child::*)").expect("failed to parse expression \"count(/child::*/child::*)\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), Some(d), Some(0), &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "3")
    }
    #[test]
    fn parse_eval_fncall_localname() {
      let x = roxmltree::Document::parse("<Test><a><b/></a><a><c/></a><a><d/></a></Test>").expect("failed to parse XML");
      let d = vec![Rc::new(Item::XNode(x.root().first_child().unwrap()))];
      let mut e = parse("local-name()").expect("failed to parse expression \"local-name()\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), Some(d), Some(0), &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "Test")
    }
    #[test]
    fn parse_eval_fncall_name() {
      let x = roxmltree::Document::parse("<Test><a><b/></a><a><c/></a><a><d/></a></Test>").expect("failed to parse XML");
      let d = vec![Rc::new(Item::XNode(x.root().first_child().unwrap()))];
      let mut e = parse("name()").expect("failed to parse expression \"name()\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), Some(d), Some(0), &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "Test")
    }
    #[test]
    fn parse_eval_fncall_string() {
      let mut e = parse("string(('a', 'b', 'c'))").expect("failed to parse expression \"string(('a', 'b', 'c'))\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "abc")
    }
    #[test]
    fn parse_eval_fncall_concat() {
      let mut e = parse("concat('a', 'b', 'c')").expect("failed to parse expression \"concat('a', 'b', 'c')\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "abc")
    }
    #[test]
    fn parse_eval_fncall_startswith_pos() {
      let mut e = parse("starts-with('abc', 'a')").expect("failed to parse expression \"starts-with('abc', 'a')\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.to_bool(), true)
    }
    #[test]
    fn parse_eval_fncall_startswith_neg() {
      let mut e = parse("starts-with('abc', 'b')").expect("failed to parse expression \"starts-with('abc', 'a')\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.to_bool(), false)
    }
    #[test]
    fn parse_eval_fncall_contains_pos() {
      let mut e = parse("contains('abc', 'b')").expect("failed to parse expression \"contains('abc', 'b')\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.to_bool(), true)
    }
    #[test]
    fn parse_eval_fncall_contains_neg() {
      let mut e = parse("contains('abc', 'd')").expect("failed to parse expression \"contains('abc', 'd')\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.to_bool(), false)
    }
    #[test]
    fn parse_eval_fncall_substringbefore_pos() {
      let mut e = parse("substring-before('abc', 'b')").expect("failed to parse expression \"substring-before('abc', 'b')\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "a")
    }
    #[test]
    fn parse_eval_fncall_substringbefore_neg() {
      let mut e = parse("substring-before('abc', 'd')").expect("failed to parse expression \"substring-before('abc', 'd')\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "")
    }
    #[test]
    fn parse_eval_fncall_substringafter_pos_1() {
      let mut e = parse("substring-after('abc', 'b')").expect("failed to parse expression \"substring-after('abc', 'b')\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "c")
    }
    #[test]
    fn parse_eval_fncall_substringafter_pos_2() {
      let mut e = parse("substring-after('abc', 'c')").expect("failed to parse expression \"substring-after('abc', 'b')\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "")
    }
    #[test]
    fn parse_eval_fncall_substringafter_neg() {
      let mut e = parse("substring-after('abc', 'd')").expect("failed to parse expression \"substring-after('abc', 'd')\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "")
    }
    #[test]
    fn parse_eval_fncall_normalizespace() {
      let mut e = parse("normalize-space('	a  b\nc 	')").expect("failed to parse expression \"normalize-space('	a  b\nc 	')\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "abc")
    }
    #[test]
    fn parse_eval_fncall_translate() {
      let mut e = parse("translate('abcdeabcde', 'ade', 'XY')").expect("failed to parse expression \"translate('abcdeabcde', 'ade', 'XY')\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "XbcYXbcY")
    }
    #[test]
    fn parse_eval_fncall_boolean_true() {
      let mut e = parse("boolean('abcdeabcde')").expect("failed to parse expression \"boolean('abcdeabcde')\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.len(), 1);
      match *s[0] {
        Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
	_ => panic!("not a singleton boolean true value")
      }
    }
    #[test]
    fn parse_eval_fncall_boolean_false() {
      let mut e = parse("boolean('')").expect("failed to parse expression \"boolean('')\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.len(), 1);
      match *s[0] {
        Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
	_ => panic!("not a singleton boolean false value")
      }
    }
    #[test]
    fn parse_eval_fncall_not_true() {
      let mut e = parse("not('')").expect("failed to parse expression \"not('')\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.len(), 1);
      match *s[0] {
        Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
	_ => panic!("not a singleton boolean true value")
      }
    }
    #[test]
    fn parse_eval_fncall_not_false() {
      let mut e = parse("not('abc')").expect("failed to parse expression \"not('abc')\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.len(), 1);
      match *s[0] {
        Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
	_ => panic!("not a singleton boolean false value")
      }
    }
    #[test]
    fn parse_eval_fncall_true() {
      let mut e = parse("true()").expect("failed to parse expression \"true()\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.len(), 1);
      match *s[0] {
        Item::Value(Value::Boolean(b)) => assert_eq!(b, true),
	_ => panic!("not a singleton boolean true value")
      }
    }
    #[test]
    fn parse_eval_fncall_false() {
      let mut e = parse("false()").expect("failed to parse expression \"false()\"");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      //println!("fncall: constructor:\n{}", format_constructor(&e, 0));
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.len(), 1);
      match *s[0] {
        Item::Value(Value::Boolean(b)) => assert_eq!(b, false),
	_ => panic!("not a singleton boolean false value")
      }
    }

    // Variables
    #[test]
    fn parse_eval_let_1() {
      let mut e = parse("let $x := 'a' return ($x, $x)").expect("failed to parse let expression");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.to_string(), "aa")
    }
    #[test]
    fn parse_eval_let_2() {
      let mut e = parse("let $x := 'a', $y := 'b' return ($x, $y)").expect("failed to parse let expression");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.len(), 2);
      assert_eq!(s.to_string(), "ab")
    }

    // Loops
    #[test]
    fn parse_eval_for_1() {
      let mut e = parse("for $x in ('a', 'b', 'c') return ($x, $x)").expect("failed to parse let expression");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.len(), 6);
      assert_eq!(s.to_string(), "aabbcc")
    }
    #[test]
    fn parse_eval_for_2() {
      let mut e = parse("for $x in (1, 2, 3) return $x * 2").expect("failed to parse let expression");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.len(), 3);
      assert_eq!(s.to_string(), "246")
    }

    #[test]
    fn parse_eval_if_1() {
      let mut e = parse("if (1) then 'one' else 'not one'").expect("failed to parse let expression");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.len(), 1);
      assert_eq!(s.to_string(), "one")
    }
    #[test]
    fn parse_eval_if_2() {
      let mut e = parse("if (0) then 'one' else 'not one'").expect("failed to parse let expression");
      let mut sc = StaticContext::new_with_builtins();
      static_analysis(&mut e, &mut sc);
      let s = evaluate(&DynamicContext::new(), None, None, &e).expect("evaluation failed");
      assert_eq!(s.len(), 1);
      assert_eq!(s.to_string(), "not one")
    }

    // Kind Tests
    #[test]
    fn xnode_kind_element_1() {
      let d = roxmltree::Document::parse("<Test><level1>1<level2/>2<level2/>3<level2/>4<level2/>5<level2/>6<level2/>7</level1></Test>").expect("failed to parse XML");
      let cons = parse("child::element()").expect("failed to parse element kind expression");
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap()))]), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 6);
      assert_eq!(e[0].to_name(), "level2");
      assert_eq!(e[1].to_name(), "level2");
      assert_eq!(e[2].to_name(), "level2");
      assert_eq!(e[3].to_name(), "level2");
      assert_eq!(e[4].to_name(), "level2");
      assert_eq!(e[5].to_name(), "level2");
    }
    #[test]
    fn xnode_kind_text_1() {
      let d = roxmltree::Document::parse("<Test><level1>1<level2/>2<level2/>3<level2/>4<level2/>5<level2/>6<level2/>7</level1></Test>").expect("failed to parse XML");
      let cons = parse("child::text()").expect("failed to parse text kind expression");
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap()))]), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 7);
      assert_eq!(e[0].to_string(), "1");
      assert_eq!(e[1].to_string(), "2");
      assert_eq!(e[2].to_string(), "3");
      assert_eq!(e[3].to_string(), "4");
      assert_eq!(e[4].to_string(), "5");
      assert_eq!(e[5].to_string(), "6");
      assert_eq!(e[6].to_string(), "7");
    }
    #[test]
    fn xnode_kind_any_1() {
      let d = roxmltree::Document::parse("<Test><level1>1<level2/>2<level2/>3<level2/>4<level2/>5<level2/>6<level2/>7</level1></Test>").expect("failed to parse XML");
      let cons = parse("child::node()").expect("failed to parse text kind expression");
      let e = evaluate(&DynamicContext::new(), Some(vec![Rc::new(Item::XNode(d.root().first_child().unwrap().first_child().unwrap()))]), Some(0), &cons)
        .expect("evaluation failed");
      assert_eq!(e.len(), 13);
      assert_eq!(e[0].to_string(), "1");
      assert_eq!(e[1].to_name(), "level2");
      assert_eq!(e[2].to_string(), "2");
      assert_eq!(e[3].to_name(), "level2");
      assert_eq!(e[4].to_string(), "3");
      assert_eq!(e[5].to_name(), "level2");
      assert_eq!(e[6].to_string(), "4");
      assert_eq!(e[7].to_name(), "level2");
      assert_eq!(e[8].to_string(), "5");
      assert_eq!(e[9].to_name(), "level2");
      assert_eq!(e[10].to_string(), "6");
      assert_eq!(e[11].to_name(), "level2");
      assert_eq!(e[12].to_string(), "7");
    }
}

