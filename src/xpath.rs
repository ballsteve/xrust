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
  multi::{many0, many1, separated_nonempty_list},
  combinator::{complete, map, opt, recognize},
  bytes::complete::{tag, take_while, take_while1, take_while_m_n},
};
use crate::item::*;
use crate::xdmerror::*;
use crate::evaluate::{SequenceConstructor, SequenceConstructorFunc,
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
        vec![SequenceConstructor{func: cons_or, data: None, args: Some(v)}]
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
        vec![SequenceConstructor{func: cons_and, data: None, args: Some(v)}]
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
	    data: None, args: Some(vec![v, t])}]
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
        vec![SequenceConstructor{func: cons_string_concat, data: None, args: Some(v)}]
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
          vec![SequenceConstructor{func: cons_range, data: None, args: Some(vec![v, u])}]
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
	let mut r = Vec::new();

        r.push(vec![SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::String("".to_string()))), args: None}]);
	r.push(a);

	for ((_x, c, _y), d) in b {
	  r.push(vec![SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::String(c.to_string()))), args: None}]);
	  r.push(d);
	}
        vec![SequenceConstructor{func: addsub, data: None, args: Some(r)}]
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

        r.push(vec![SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::String("".to_string()))), args: None}]);
	r.push(a);

	for ((_x, c, _y), d) in b {
	  r.push(vec![SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::String(c.to_string()))), args: None}]);
	  r.push(d);
	}
        vec![SequenceConstructor{func: muldiv, data: None, args: Some(r)}]
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
        vec![SequenceConstructor{func: cons_union, data: None, args: Some(v)}]
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

        r.push(vec![SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::String("".to_string()))), args: None}]);
	r.push(a);

	for ((_x, c, _y), d) in b {
	  r.push(vec![SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::String(c.to_string()))), args: None}]);
	  r.push(d);
	}
        vec![SequenceConstructor{func: cons_intersectexcept, data: None, args: Some(r)}]
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
	  vec![SequenceConstructor{func: cons_instanceof, data: None, args: Some(r)}]
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
	  vec![SequenceConstructor{func: cons_treat, data: None, args: Some(r)}]
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
	  vec![SequenceConstructor{func: cons_castable, data: None, args: Some(r)}]
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
fn qname(input: &str) -> IResult<&str, (&str, &str)> {
  alt((
    prefixed_name,
    unprefixed_name,
  ))
  (input)
}
fn unprefixed_name(input: &str) -> IResult<&str, (&str, &str)> {
  map (
    ncname,
    |localpart| {
      ("", localpart)
    }
  )
  (input)
}
fn prefixed_name(input: &str) -> IResult<&str, (&str, &str)> {
  map (
    tuple((
      ncname,
      tag(":"),
      ncname
    )),
    |(prefix, _, localpart)| {
      (prefix, localpart)
    }
  )
  (input)
}
// NCName ::= Name - (Char* ':' Char*)
// Name ::= NameStartChar NameChar*
// NameStartChar ::= ':' | [A-Z] | '_' | [a-z] | [#xC0-#xD6] | [#xD8-#xF6] | [#xF8-#x2FF] | [#x370-#x37D] | [#x37F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]
// NameChar ::= NameStartChar | '-' | '.' | [0-9] | #xB7 | [#x0300-#x036F] | [#x203F-#x2040]
fn ncname(input: &str) -> IResult<&str, &str> {
  //println!("ncname: input=\"{}\"", input);
//  recognize (
  map (
    pair (
      ncnamestartchar,
      take_while(is_ncnamechar),
    ),
    |(a, b)| {
      //println!("ncname: got \"{}\" and \"{}\"", a, b);
      a
    }
  )
  (input)
}
//fn ncname_old(input: &str) -> IResult<&str, &str> {
//  recognize (
//    pair (
//      ncnamestartchar,
//      many0(ncnamechar),
//    )
//  )
//  (input)
//}
//fn ncname_broken(input: &str) -> IResult<&str, String> {
//  map (
//    many1(none_of(":")),
//    |v| {
//      v.iter().collect::<String>()
//    }
//  )
//  (input)
//}
fn name(input: &str) -> IResult<&str, &str> {
  //println!("name: input=\"{}\"", input);
  recognize (
    pair (
      namestartchar,
      take_while1(is_namechar),
    )
  )
  (input)
}
//fn namechar(input: &str) -> IResult<&str, char> {
//  alt((
//    namestartchar,
//    one_of(".-0123456789\u{B7}"),
//    take_while1(is_namechar_range),
//  ))
//  (input)
//}
fn is_namechar(ch: char) -> bool {
  if is_namestartchar(ch) {
    true
  } else {
    match ch {
      '.' => true,
      '-' => true,
      '0'..='9' => true,
      '\u{B7}' => true,
      '\u{0300}'..='\u{036F}' => true,
      '\u{203F}'..='\u{2040}' => true,
      _ => false
    }
  }
}
//fn ncnamechar(input: &str) -> IResult<&str, char> {
//  alt((
//    ncnamestartchar,
//    one_of(".-0123456789"),
//    one_of("\u{B7}"),
//    one_of(('\u{0300}'..='\u{036F}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{203F}'..='\u{2040}').map(char::from).collect::<Vec<_>>()),
//  ))
//  (input)
//}
fn ncnamechar(input: &str) -> IResult<&str, &str> {
  take_while_m_n(1, 1, is_ncnamechar)
  (input)
}
fn is_ncnamechar(ch: char) -> bool {
  //println!("is_ncnamechar: input \"{}\"", ch);
  if is_ncnamestartchar(ch) {
    //println!("is_ncnamechar: input is a ncnamestartchar");
    true
  } else {
    match ch {
      '.' |
      '-' |
      '0'..='9' |
      '\u{B7}' |
      '\u{0300}'..='\u{036F}' |
      '\u{203F}'..='\u{2040}' => {
        println!("is_ncnamechar: true");
        true
      },
      _ => false
    }
  }
}
//fn namestartchar(input: &str) -> IResult<&str, char> {
//  alt((
//    one_of(":_"),
//    one_of(('A'..='Z').map(char::from).collect::<Vec<_>>()),
//    one_of(('a'..='z').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{C0}'..='\u{D6}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{D8}'..='\u{F6}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{F8}'..='\u{2FF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{370}'..='\u{37D}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{37F}'..='\u{1FFF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{200C}'..='\u{200D}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{2070}'..='\u{218F}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{2C00}'..='\u{2FEF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{3001}'..='\u{D7FF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{F900}'..='\u{FDCF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{FDF0}'..='\u{FFFD}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{10000}'..='\u{EFFFF}').map(char::from).collect::<Vec<_>>()),
//  ))
//  (input)
//}
fn namestartchar(input: &str) -> IResult<&str, &str> {
  take_while_m_n(1, 1, is_namestartchar)
  (input)
}
fn is_namestartchar(ch: char) -> bool {
  match ch {
    ':' => true,
    _ => is_ncnamestartchar(ch)
  }
}
// Same as above, but without the colon
//fn ncnamestartchar(input: &str) -> IResult<&str, char> {
//  alt((
//    one_of("_"),
//    one_of(('A'..='Z').map(char::from).collect::<Vec<_>>()),
//    one_of(('a'..='z').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{C0}'..='\u{D6}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{D8}'..='\u{F6}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{F8}'..='\u{2FF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{370}'..='\u{37D}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{37F}'..='\u{1FFF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{200C}'..='\u{200D}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{2070}'..='\u{218F}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{2C00}'..='\u{2FEF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{3001}'..='\u{D7FF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{F900}'..='\u{FDCF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{FDF0}'..='\u{FFFD}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{10000}'..='\u{EFFFF}').map(char::from).collect::<Vec<_>>()),
//  ))
//  (input)
//}
fn ncnamestartchar(input: &str) -> IResult<&str, &str> {
  //println!("ncnamestartchar: input \"{}\"", input);
  take_while_m_n(1, 1, is_ncnamestartchar)
  (input)
}
fn is_ncnamestartchar(ch: char) -> bool {
  //println!("is_ncnamestartchar: input \"{}\"", ch);
  match ch {
    '_' |
    'A'..='Z' |
    'a'..='z' |
    '\u{C0}'..='\u{D6}' => {
      //println!("is_ncnamestartchar: true");
      true
    },
    // etc
    _ => false
  }
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
	  vec![SequenceConstructor{func: cons_cast, data: None, args: Some(r)}]
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
        vec![SequenceConstructor{func: cons_arrow, data: None, args: None}]
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
    |(_prefix, localpart)| {
      vec![SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::String(localpart.to_string()))), args: None}]
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
	  a.push(vec![SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::String(String::from(i)))), args: None}]);
	}
	a.push(v);
        vec![SequenceConstructor{func: cons_unary, data: None, args: Some(a)}]
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
	s.push(vec![SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::String(String::from("")))), args: None}]);
	s.push(u);
	for (a, b) in v {
	  s.push(vec![SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::String(String::from(a)))), args: None}]);
	  s.push(b);
	}
        vec![SequenceConstructor{func: cons_simplemap, data: None, args: Some(s)}]
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
	  vec![SequenceConstructor{func: cons_root, data: None, args: None},
	  SequenceConstructor{func: cons_child, data: None, args: Some(vec![a])}]
	}
	None => {
	  vec![SequenceConstructor{func: cons_root, data: None, args: None}]
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
      vec![SequenceConstructor{func: cons_root, data: None, args: None},
	SequenceConstructor{func: cons_descendant_or_self, data: None, args: Some(vec![v])}]
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

        r.push(vec![SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::String("".to_string()))), args: None}]);
	r.push(a);

	for ((_x, c, _y), d) in b {
	  r.push(vec![SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::String(c.to_string()))), args: None}]);
	  r.push(d);
	}
        vec![SequenceConstructor{func: cons_relativepath, data: None, args: Some(r)}]
      }
    }
  )
  (input)
}
// For debugging: a version of the above function that steps through the parsing
fn relativepath_expr_dbg(newinput: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  let mut myin = newinput;
  //println!("relpath: starting with \"{}\"", myin);
  let (myin, a) = step_expr(myin)?;
  let mut r = Vec::new();

  r.push(vec![SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::String("".to_string()))), args: None}]);
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
    r.push(vec![SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::String(c.to_string()))), args: None}]);

    let (myin, d) = step_expr(myin)?;
    //println!("got next step");
    r.push(d);
    break;
  }

  //println!("relpath: finished");
  Ok((myin, vec![SequenceConstructor{func: cons_relativepath, data: None, args: Some(r)}]))
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
    |(_a, _n)| {
      vec![SequenceConstructor{func: cons_step, data: None, args: None}]
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
    |(_a, (_prefix, _localpart))| {
      vec![SequenceConstructor{func: cons_step, data: None, args: None}]
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

// NodeTest ::= KindTest | NameTest
// TODO: KindTest
fn nodetest(input: &str) -> IResult<&str, (&str, &str)> {
  nametest
  (input)
}

// NameTest ::= EQName | Wildcard
// TODO: allow EQName rather than QName
fn nametest(input: &str) -> IResult<&str, (&str, &str)> {
  alt((
    qname,
    wildcard,
  ))
  (input)
}

// Wildcard ::= '*' | (NCName ':*') | ('*:' NCName) | (BracedURILiteral '*')
// TODO: more specific wildcards
fn wildcard(input: &str) -> IResult<&str, (&str, &str)> {
  map(
    tag("*"),
    |_w| {
      ("*", "*")
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
    vec![SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::Integer(n))), args: None}]
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
        Ok(m) => Item::Value(Value::Double(m)),
	Err(_) => Item::Value(Value::Decimal(decimal::d128!(s))),
      };
      vec![SequenceConstructor{func: cons_literal, data: Some(i), args: None}]
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
        Ok(m) => Item::Value(Value::Double(m)),
	Err(_) => panic!("unable to convert to double"),
      };
      vec![SequenceConstructor{func: cons_literal, data: Some(i), args: None}]
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
    |s| vec![SequenceConstructor{func: cons_literal, data: Some(Item::Value(Value::String(s))), args: None}]
  )
  (input)
}
// ContextItemExpr ::= '.'
fn context_item(input: &str) -> IResult<&str, Vec<SequenceConstructor>> {
  map(
    tag("."),
    |_| vec![SequenceConstructor{func: cons_context_item, data: None, args: None}]
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
	  match e[0] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::Integer(v))), args: None} => assert_eq!(v, 1),
	    _ => panic!("item is not a literal integer value constructor")
	  }
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    // Parses to a singleton double/decimal sequence constructor
    #[test]
    fn nomxpath_parse_decimal() {
        let e = parse("1.2").expect("failed to parse expression \"1.2\"");
	if e.len() == 1 {
	  match e[0] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::Double(v))), args: None} => assert_eq!(v, 1.2),
	    _ => panic!("item is not a literal double constructor")
	  }
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    // Parses to a singleton double sequence constructor
    #[test]
    fn nomxpath_parse_double() {
        let e = parse("1.2e2").expect("failed to parse expression \"1.2e2\"");
	if e.len() == 1 {
	  match e[0] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::Double(v))), args: None} => assert_eq!(v, 120.0),
	    _ => panic!("item is not a literal double constructor")
	  }
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    //#[test]
    //fn nomxpath_parse_double() {
        //assert_eq!(parse("2.0").unwrap(), Item::Value(Value::Double(2.0)));
    //}
    // Parses to a singleton string
    #[test]
    fn nomxpath_parse_string_apos() {
        let e = parse("'abc'").expect("failed to parse expression \"'abc'\"");
	if e.len() == 1 {
	  match &e[0] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::String(v))), args: None} => assert_eq!(v, "abc"),
	    _ => panic!("item is not a literal string value")
	  }
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    // Parses to a singleton string
    #[test]
    fn nomxpath_parse_string_apos_esc() {
        let e = parse("'abc''def'").expect("failed to parse expression \"'abc''def'\"");
	if e.len() == 1 {
	  match &e[0] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::String(v))), args: None} => assert_eq!(v, "abc'def"),
	    _ => panic!("item is not a literal string value")
	  }
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    // Parses to a singleton string
    #[test]
    fn nomxpath_parse_string_quot() {
        let e = parse(r#""abc""#).expect("failed to parse expression \"\"abc\"\"");
	if e.len() == 1 {
	  match &e[0] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::String(v))), args: None} => assert_eq!(v, "abc"),
	    _ => panic!("item is not a literal string value")
	  }
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    // Parses to a singleton string
    #[test]
    fn nomxpath_parse_string_quot_esc() {
        let e = parse(r#""abc""def""#).expect("failed to parse expression \"\"abc\"\"def\"\"");
	if e.len() == 1 {
	  match &e[0] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::String(v))), args: None} => assert_eq!(v, r#"abc"def"#),
	    _ => panic!("item is not a literal string value")
	  }
	} else {
	  panic!("sequence is not a singleton")
	}
    }
    #[test]
    fn nomxpath_parse_literal_sequence() {
        let e = parse("1,'abc',2").expect("failed to parse \"1,'abc',2\"");
	if e.len() == 3 {
	  match &e[0] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::Integer(v))), args: None} => assert_eq!(*v, 1),
	    _ => panic!("item 0 is not a literal integer value")
	  }
	  match &e[1] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::String(v))), args: None} => assert_eq!(v, r#"abc"#),
	    _ => panic!("item 1 is not a literal string value")
	  }
	  match &e[2] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::Integer(v))), args: None} => assert_eq!(*v, 2),
	    _ => panic!("item 2 is not a literal integer value")
	  }
	} else {
	  panic!("sequence does not have 3 items")
	}
    }
    #[test]
    fn nomxpath_parse_literal_seq_ws() {
        let e = parse("1 , 'abc', 2").expect("failed to parse \"1 , 'abc', 2\"");
	if e.len() == 3 {
	  match &e[0] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::Integer(v))), args: None} => assert_eq!(*v, 1),
	    _ => panic!("item 0 is not a literal integer value")
	  }
	  match &e[1] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::String(v))), args: None} => assert_eq!(v, r#"abc"#),
	    _ => panic!("item 1 is not a literal string value")
	  }
	  match &e[2] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::Integer(v))), args: None} => assert_eq!(*v, 2),
	    _ => panic!("item 2 is not a literal integer value")
	  }
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
	  match e[0] {
	    SequenceConstructor{func: _cons_literal, data: Some(Item::Value(Value::Integer(v))), args: None} => assert_eq!(v, 1),
	    _ => panic!("item is not a literal integer value constructor")
	  }
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

