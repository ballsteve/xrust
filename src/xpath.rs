//! # xdm::xpath
//!
//! An XPath parser as a nom parser combinator.

extern crate nom;
use decimal;
use nom:: {
  IResult,
  character::complete::*,
  branch::alt,
  character::complete::{char, none_of},
  sequence::{delimited, pair, tuple},
  multi::{many0, separated_nonempty_list},
  combinator::{complete, map, opt, recognize},
  bytes::complete::{tag, take_while, take_while1, take_while_m_n},
};
use crate::item::*;
use crate::xdmerror::*;
use crate::parsecommon::*;
use crate::evaluate::{DynamicContext,
    SequenceConstructor, SequenceConstructorFunc,
    NameTest, WildcardOrName,
    NodeTest, NodeMatch,
    Axis,
    cons_literal, cons_context_item,
    cons_or, cons_and,
    comparison_general_equal,
    comparison_general_notequal,
    comparison_general_lessthan,
    comparison_general_lessthanequal,
    comparison_general_greaterthan,
    comparison_general_greaterthanequal,
    comparison_value_equal,
    comparison_value_notequal,
    comparison_value_lessthan,
    comparison_value_lessthanequal,
    comparison_value_greaterthan,
    comparison_value_greaterthanequal,
    comparison_node_is,
    comparison_node_before,
    comparison_node_after,
    cons_string_concat,
    cons_range,
    addsub, muldiv,
    cons_union,
    cons_intersectexcept,
    cons_instanceof,
    cons_treat,
    cons_castable,
    cons_cast,
    cons_arrow,
    cons_unary,
    cons_simplemap,
    cons_root,
    cons_child,
    cons_descendant_or_self,
    cons_relativepath,
    cons_step,
    eval,
};

// Expr ::= ExprSingle (',' ExprSingle)* ;
// we need to unpack each primary_expr
fn expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
fn expr_single(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  or_expr(input)
  // TODO: other branches
}

// OrExpr ::= AndExpr ('or' AndExpr)*
fn or_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  map (
    separated_nonempty_list(
      tuple((multispace0, tag("or"), multispace0)),
      and_expr
    ),
    |v: Vec<Vec<SequenceConstructor>>| {
      if v.len() == 1 {
        let mut s = Vec::new();
        for i in v {
          for j in i {
            s.push(j)
	  }
	}
	s
      } else {
        vec![SequenceConstructor{func: cons_or, data: None, args: Some(v), nodematch: None}]
      }
    }
  )
  (input)
}

// AndExpr ::= ComparisonExpr ('and' ComparisonExpr)*
fn and_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  map (
    separated_nonempty_list(
      tuple((multispace0, tag("and"), multispace0)),
      comparison_expr
    ),
    |v: Vec<Vec<SequenceConstructor>>| {
      if v.len() == 1 {
        let mut s = Vec::new();
        for i in v {
          for j in i {
            s.push(j)
	  }
	}
	s
      } else {
        vec![SequenceConstructor{func: cons_and, data: None, args: Some(v), nodematch: None}]
      }
    }
  )
  (input)
}

// ComparisonExpr ::= StringConcatExpr ( (ValueComp | GeneralComp | NodeComp) StringConcatExpr)?
fn comparison_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
	  vec![SequenceConstructor{func: choose_compare(b).expect("invalid comparison operator"),
	    data: None, args: Some(vec![v, t]), nodematch: None}]
	},
      }
    }
  )
  (input)
}
fn choose_compare(a: &str) -> Result<SequenceConstructorFunc, Error> {
  match a {
    "=" => Ok(comparison_general_equal),
    "!=" => Ok(comparison_general_notequal),
    "<" => Ok(comparison_general_lessthan),
    "<=" => Ok(comparison_general_lessthanequal),
    ">" => Ok(comparison_general_greaterthan),
    ">=" => Ok(comparison_general_greaterthanequal),
    "eq" => Ok(comparison_value_equal),
    "ne" => Ok(comparison_value_notequal),
    "lt" => Ok(comparison_value_lessthan),
    "le" => Ok(comparison_value_lessthanequal),
    "gt" => Ok(comparison_value_greaterthan),
    "ge" => Ok(comparison_value_greaterthanequal),
    "is" => Ok(comparison_node_is),
    "<<" => Ok(comparison_node_before),
    ">>" => Ok(comparison_node_after),
    _ => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from("not a valid comparison operator")}),
  }
}

// StringConcatExpr ::= RangeExpr ( '||' RangeExpr)*
fn stringconcat_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
        vec![SequenceConstructor{func: cons_string_concat, data: None, args: Some(v), nodematch: None}]
      }
    }
  )
  (input)
}

// RangeExpr ::= AdditiveExpr ( 'to' AdditiveExpr)?
fn range_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
          vec![SequenceConstructor{func: cons_range, data: None, args: Some(vec![v, u]), nodematch: None}]
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
fn additive_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
        // The arguments to the addsub function are the items to be summed
	// These are pair-wise items: first is the operator as a string literal,
	// second is the value
	// we fake an entry for first part of the first pair
	let mut r: Vec<Vec<SequenceConstructor>> = Vec::new();

	r.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String("".to_string()))))]);
	r.push(a);

	for ((_x, c, _y), d) in b {
	  r.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String(c.to_string()))))]);
	  r.push(d);
	}
        vec![SequenceConstructor::new(addsub).set_args(Some(r))]
      }
    }
  )
  (input)
}
// MultiplicativeExpr ::= UnionExpr ( ('*' | 'div' | 'idiv' | 'mod') UnionExpr)*
fn multiplicative_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
        // The arguments to the addsub function are the items to be summed
	// These are pair-wise items: first is the operator as a string literal,
	// second is the value
	// we fake an entry for first part of the first pair
	let mut r = Vec::new();

        r.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String("".to_string()))))]);
	r.push(a);

	for ((_x, c, _y), d) in b {
	  r.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String(c.to_string()))))]);
	  r.push(d);
	}
        vec![SequenceConstructor::new(muldiv).set_args(Some(r))]
      }
    }
  )
  (input)
}

// UnionExpr ::= IntersectExceptExpr ( ('union' | '|') IntersectExceptExpr)*
fn union_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
        vec![SequenceConstructor::new(cons_union).set_args(Some(v))]
      }
    }
  )
  (input)
}

// IntersectExceptExpr ::= InstanceOfExpr ( ('intersect' | 'except') InstanceOfExpr)*
fn intersectexcept_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
	// we fake an entry for first part of the first pair
	let mut r = Vec::new();

        r.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String("".to_string()))))]);
	r.push(a);

	for ((_x, c, _y), d) in b {
	  r.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String(c.to_string()))))]);
	  r.push(d);
	}
        vec![SequenceConstructor::new(cons_intersectexcept).set_args(Some(r))]
      }
    }
  )
  (input)
}

// InstanceOfExpr ::= TreatExpr ( 'instance' 'of' SequenceType)?
fn instanceof_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
	Some(t) => {
	  let mut r = Vec::new();
	  r.push(u);
	  let (_a, _b, _c, _d, _e, st) = t;
	  r.push(st);
	  vec![SequenceConstructor::new(cons_instanceof).set_args(Some(r))]
	}
      }
    }
  )
  (input)
}

// SequenceType ::= ( 'empty-sequence' '(' ')' | (ItemType OccurrenceIndicator?)
// TODO: implement this parser fully
fn sequencetype_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  map (
    tag("empty-sequence()"),
    |_v| {
      Vec::new()
    }
  )
  (input)
}

// TreatExpr ::= CastableExpr ( 'treat' 'as' SequenceType)?
fn treat_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
	Some(t) => {
	  let mut r = Vec::new();
	  r.push(u);
	  let (_a, _b, _c, _d, _e, st) = t;
	  r.push(st);
	  vec![SequenceConstructor::new(cons_treat).set_args(Some(r))]
	}
      }
    }
  )
  (input)
}

// CastableExpr ::= CastExpr ( 'castable' 'as' SingleType)?
fn castable_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
	Some(t) => {
	  let mut r = Vec::new();
	  r.push(u);
	  let (_a, _b, _c, _d, _e, st) = t;
	  r.push(st);
	  vec![SequenceConstructor::new(cons_castable).set_args(Some(r))]
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
fn singletype_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
fn cast_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
	Some(t) => {
	  let mut r = Vec::new();
	  r.push(u);
	  let (_a, _b, _c, _d, _e, st) = t;
	  r.push(st);
	  vec![SequenceConstructor::new(cons_cast).set_args(Some(r))]
	}
      }
    }
  )
  (input)
}

// ArrowExpr ::= UnaryExpr ( '=>' ArrowFunctionSpecifier ArgumentList)*
fn arrow_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
        vec![SequenceConstructor::new(cons_arrow)]
      }
    }
  )
  (input)
}

// ArrowFunctionSpecifier ::= EQName | VarRef | ParenthesizedExpr
// TODO: finish this parser with EQName and VarRef
fn arrowfunctionspecifier(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
fn qname_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  map (
    qname,
    |q| {
      match q {
        NodeTest::Name(NameTest{name: Some(WildcardOrName::Name(localpart)), ns: None, prefix: None}) => {
	  vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String(localpart.to_string()))))]
	}
        _ => {
      	  vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String(String::from("invalid qname")))))]
	}
      }
    }
  )
  (input)
}

// ArgumentList ::= '(' (Argument (',' Argument)*)? ')'
// TODO: finish this parser with actual arguments
fn argumentlist(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
fn unary_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
        let mut a = Vec::new();
	for i in u {
	  a.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String(String::from(i)))))]);
	}
	a.push(v);
        vec![SequenceConstructor::new(cons_unary).set_args(Some(a))]
      }
    }
  )
  (input)
}

// ValueExpr (SimpleMapExpr) ::= PathExpr ('!' PathExpr)*
fn value_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
        let mut s = Vec::new();
	s.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String(String::from("")))))]);
	s.push(u);
	for (a, b) in v {
	  s.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String(String::from(a)))))]);
	  s.push(b);
	}
        vec![SequenceConstructor::new(cons_simplemap).set_args(Some(s))]
      }
    }
  )
  (input)
}

// PathExpr ::= ('/' RelativePathExpr?) | ('//' RelativePathExpr) | RelativePathExpr
fn path_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  alt((
    absolute_descendant_expr,
    absolute_path_expr,
    relativepath_expr,
  ))
  (input)
}
// ('/' RelativePathExpr?)
fn absolute_path_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  map(
    pair(
      tag("/"),
      opt(relativepath_expr),
    ),
    |(_u, v)| {
      match v {
        Some(a) => {
	  vec![SequenceConstructor::new(cons_root),
	  SequenceConstructor::new(cons_child).set_args(Some(vec![a]))]
	}
	None => {
	  vec![SequenceConstructor::new(cons_root)]
	}
      }
    }
  )
  (input)
}
// ('//' RelativePathExpr)
fn absolute_descendant_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  map(
    pair(
      tag("//"),
      relativepath_expr,
    ),
    |(_u, v)| {
      vec![SequenceConstructor::new(cons_root),
	SequenceConstructor::new(cons_descendant_or_self).set_args(Some(vec![v]))]
    }
  )
  (input)
}

// RelativePathExpr ::= StepExpr (('/' | '//') StepExpr)*
fn relativepath_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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

        r.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String("".to_string()))))]);
	r.push(a);

	for ((_x, c, _y), d) in b {
	  r.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String(c.to_string()))))]);
	  r.push(d);
	}
        vec![SequenceConstructor::new(cons_relativepath).set_args(Some(r))]
      }
    }
  )
  (input)
}
// For debugging: a version of the above function that steps through the parsing
fn relativepath_expr_dbg(newinput: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  let myin = newinput;
  //println!("relpath: starting with \"{}\"", myin);
  let (myin, a) = step_expr(myin)?;
  let mut r = Vec::new();

  r.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String("".to_string()))))]);
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
    r.push(vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String(c.to_string()))))]);

    let (_myin, d) = step_expr(myin)?;
    //println!("got next step");
    r.push(d);
    break;
  }

  //println!("relpath: finished");
  Ok((myin, vec![SequenceConstructor::new(cons_relativepath).set_args(Some(r))]))
}

// StepExpr ::= PostfixExpr | AxisStep
fn step_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  //println!("step_expr: input \"{}\"", input);
  alt((
    postfix_expr,
    axisstep
  ))
  (input)
}

// AxisStep ::= (ReverseStep | ForwardStep) PredicateList
// TODO: predicates
fn axisstep(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  alt((
    reversestep,
    forwardstep
  ))
  (input)
}

// ForwardStep ::= (ForwardAxis NodeTest) | AbbrevForwardStep
// TODO: abbreviated step
fn forwardstep(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  map (
    pair(
      forwardaxis,
      nodetest
    ),
    |(a, n)| {
      vec![SequenceConstructor::new(cons_step).set_nodematch(Some(NodeMatch{axis: Axis::from(a), nodetest: n}))]
    }
  )
  (input)
}
// ReverseStep ::= (ReverseAxis NodeTest) | AbbrevReverseStep
// TODO: abbreviated step
fn reversestep(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  map (
    pair(
      reverseaxis,
      nodetest
    ),
    |(a, n)| {
      vec![SequenceConstructor::new(cons_step).set_nodematch(Some(NodeMatch{axis: Axis::from(a), nodetest: n}))]
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
        tag("descendant"),
        tag("attribute"),
        tag("self"),
        tag("descendant-or-self"),
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
        tag("ancestor"),
        tag("ancestor-or-self"),
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
// TODO: KindTest
fn nodetest(input: &str) -> IResult<&str, NodeTest> {
  nametest
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
fn postfix_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  primary_expr(input)
}

// PrimaryExpr ::= Literal | VarRef | ParenthesizedExpr | ContextItemExpr | FunctionCall | FunctionItemExpr | MapConstructor | ArrayConstructor | UnaryLookup
// TODO: finish this parser
fn primary_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  alt((
    literal,
    context_item,
    parenthesized_expr
  ))
  (input)
}

// Literal ::= NumericLiteral | StringLiteral
fn literal(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  alt((
    numeric_literal ,
    string_literal
  ))
  (input)
}

// NumericLiteral ::= IntegerLiteral | DecimalLiteral | DoubleLiteral
fn numeric_literal(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  alt((
    double_literal,
    decimal_literal,
    integer_literal,
  ))
  (input)
}
// IntegerLiteral ::= Digits
fn integer_literal(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  map(digit1, |s: &str| {
    let n = s.parse::<i64>().unwrap();
    vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::Integer(n))))]
  })
  (input)
}
// DecimalLiteral ::= ('.' Digits) | (Digits '.' [0-9]*)
// Construct a double, but if that fails fall back to decimal
fn decimal_literal(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  map(
    alt((
      recognize(complete(pair(tag("."), digit1))),
      recognize(complete(tuple((digit1, tag("."), digit0)))),
    )),
    |s: &str| {
      let n = s.parse::<f64>();
      let i = match n {
        Ok(m) => Box::new(Value::Double(m)),
	Err(_) => Box::new(Value::Decimal(decimal::d128!(s))),
      };
      vec![SequenceConstructor::new(cons_literal).set_data(Some(i))]
    }
  )
  (input)
}
// DoubleLiteral ::= (('.' Digits) | (Digits ('.' [0-9]*)?)) [eE] [+-]? Digits
// Construct a double
fn double_literal(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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
        Ok(m) => Box::new(Value::Double(m)),
	Err(_) => panic!("unable to convert to double"),
      };
      vec![SequenceConstructor::new(cons_literal).set_data(Some(i))]
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
fn string_literal(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  map(
    alt((
      string_literal_double ,
      string_literal_single
    )),
    |s| vec![SequenceConstructor::new(cons_literal).set_data(Some(Box::new(Value::String(s))))]
  )
  (input)
}
// ContextItemExpr ::= '.'
fn context_item(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  map(
    tag("."),
    |_| vec![SequenceConstructor::new(cons_context_item)]
  )
  (input)
}
// ParenthesizedExpr ::= '(' Expr? ')'
fn parenthesized_expr(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
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

pub fn parse(e: &str) -> Result<Vec<SequenceConstructor>, Error> {
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
	  let s = eval(e, &DynamicContext::new()).expect("unable to evaluate sequence constructor");
	  assert_eq!(s[0].to_int().unwrap(), 1)
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    // Parses to a singleton double/decimal sequence constructor
    #[test]
    fn nomxpath_parse_decimal() {
        let e = parse("1.2").expect("failed to parse expression \"1.2\"");
	if e.len() == 1 {
	  let s = eval(e, &DynamicContext::new()).expect("unable to evaluate sequence constructor");
	  assert_eq!(s[0].to_double().unwrap(), 1.2)
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    // Parses to a singleton double sequence constructor
    #[test]
    fn nomxpath_parse_double() {
        let e = parse("1.2e2").expect("failed to parse expression \"1.2e2\"");
	if e.len() == 1 {
	  let s = eval(e, &DynamicContext::new()).expect("unable to evaluate sequence constructor");
	  assert_eq!(s[0].to_double().unwrap(), 120.0)
	} else {
	  panic!("sequence is not a singleton")
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
	  let s = eval(e, &DynamicContext::new()).expect("unable to evaluate sequence constructor");
	  assert_eq!(s[0].to_string(), "abc")
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    // Parses to a singleton string
    #[test]
    fn nomxpath_parse_string_apos_esc() {
        let e = parse("'abc''def'").expect("failed to parse expression \"'abc''def'\"");
	if e.len() == 1 {
	  let s = eval(e, &DynamicContext::new()).expect("unable to evaluate sequence constructor");
	  assert_eq!(s[0].to_string(), "abc'def")
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    // Parses to a singleton string
    #[test]
    fn nomxpath_parse_string_quot() {
        let e = parse(r#""abc""#).expect("failed to parse expression \"\"abc\"\"");
	if e.len() == 1 {
	  let s = eval(e, &DynamicContext::new()).expect("unable to evaluate sequence constructor");
	  assert_eq!(s[0].to_string(), "abc")
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    // Parses to a singleton string
    #[test]
    fn nomxpath_parse_string_quot_esc() {
        let e = parse(r#""abc""def""#).expect("failed to parse expression \"\"abc\"\"def\"\"");
	if e.len() == 1 {
	  let s = eval(e, &DynamicContext::new()).expect("unable to evaluate sequence constructor");
	  assert_eq!(s[0].to_string(), r#"abc"def"#)
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    #[test]
    fn nomxpath_parse_literal_sequence() {
        let e = parse("1,'abc',2").expect("failed to parse \"1,'abc',2\"");
	if e.len() == 3 {
	  let s = eval(e, &DynamicContext::new()).expect("unable to evaluate sequence constructor");
	  assert_eq!(s[0].to_int().unwrap(), 1);
	  assert_eq!(s[1].to_string(), r#"abc"#);
	  assert_eq!(s[2].to_int().unwrap(), 2);
	} else {
	  panic!("sequence does not have 3 items")
	}
    }
    #[test]
    fn nomxpath_parse_literal_seq_ws() {
        let e = parse("1 , 'abc', 2").expect("failed to parse \"1 , 'abc', 2\"");
	if e.len() == 3 {
	  let s = eval(e, &DynamicContext::new()).expect("unable to evaluate sequence constructor");
	  assert_eq!(s[0].to_int().unwrap(), 1);
	  assert_eq!(s[1].to_string(), r#"abc"#);
	  assert_eq!(s[2].to_int().unwrap(), 2);
	} else {
	  panic!("sequence does not have 3 items")
	}
    }

    // Parses to a singleton context item sequence constructor
    #[test]
    fn nomxpath_parse_context_item() {
        let e = parse(".").expect("failed to parse expression \".\"");
	if e.len() == 1 {
	  let s = &e[0].data;
	  match s {
	    None => assert!(true),
	    _ => panic!("item is not a context item constructor")
	  }
	} else {
	  panic!("sequence is not a singleton")
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
	  let s = eval(e, &DynamicContext::new()).expect("unable to evaluate sequence constructor");
	  assert_eq!(s[0].to_int().unwrap(), 1)
	} else {
	  panic!("sequence is not a singleton")
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
    fn nomxpath_parse_root() {
        let e = parse("/").expect("failed to parse expression \"/\"");
	if e.len() == 1 {
	  assert!(true) // TODO: check the sequence constructor
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    #[test]
    fn nomxpath_parse_root_step_1() {
        let _e = parse("/child::a").expect("failed to parse expression \"/child::a\"");
	assert!(true) // TODO: check the sequence constructor
    }
    #[test]
    fn nomxpath_parse_root_step_2() {
        let _e = parse("/child::a/child::b").expect("failed to parse expression \"/child::a/child::b\"");
	assert!(true) // TODO: check the sequence constructor
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
}

