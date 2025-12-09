//! Functions that manipulate type information

use crate::item::Node;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::pair::pair;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::tuple6;
use crate::parser::combinators::whitespace::xpwhitespace;
use crate::parser::xpath::functions::arrow_expr;
use crate::parser::xpath::nodetests::qualname_test;
use crate::parser::{ParseError, ParseInput, StaticState};
use crate::transform::Transform;
use qualname::{NamespacePrefix, NamespaceUri};

// InstanceOfExpr ::= TreatExpr ( 'instance' 'of' SequenceType)?
pub(crate) fn instanceof_expr<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(
        pair(
            treat_expr::<N, L>(),
            opt(tuple6(
                xpwhitespace(),
                tag("instance"),
                xpwhitespace(),
                tag("of"),
                xpwhitespace(),
                sequencetype_expr::<N, L>(),
            )),
        ),
        |(v, o)| {
            if o.is_none() {
                v
            } else {
                Transform::NotImplemented("instanceof_expr".to_string())
            }
        },
    ))
}

// SequenceType ::= ( 'empty-sequence' '(' ')' | (ItemType OccurrenceIndicator?)
// TODO: implement this parser fully
fn sequencetype_expr<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(tag("empty-sequence()"), |_| {
        Transform::NotImplemented("sequencetype_expr".to_string())
    }))
}

// TreatExpr ::= CastableExpr ( 'treat' 'as' SequenceType)?
fn treat_expr<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(
        pair(
            castable_expr::<N, L>(),
            opt(tuple6(
                xpwhitespace(),
                tag("treat"),
                xpwhitespace(),
                tag("as"),
                xpwhitespace(),
                sequencetype_expr::<N, L>(),
            )),
        ),
        |(v, o)| {
            if o.is_none() {
                v
            } else {
                Transform::NotImplemented("treat_expr".to_string())
            }
        },
    ))
}

// CastableExpr ::= CastExpr ( 'castable' 'as' SingleType)?
fn castable_expr<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(
        pair(
            cast_expr::<N, L>(),
            opt(tuple6(
                xpwhitespace(),
                tag("castable"),
                xpwhitespace(),
                tag("as"),
                xpwhitespace(),
                singletype_expr::<N, L>(),
            )),
        ),
        |(v, o)| {
            if o.is_none() {
                v
            } else {
                Transform::NotImplemented("castable_expr".to_string())
            }
        },
    ))
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
fn singletype_expr<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(pair(qualname_test(), tag("?")), |_| {
        Transform::NotImplemented("singletype_expr".to_string())
    }))
}

// CastExpr ::= ArrowExpr ( 'cast' 'as' SingleType)?
fn cast_expr<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(
        pair(
            arrow_expr::<N, L>(),
            opt(tuple6(
                xpwhitespace(),
                tag("cast"),
                xpwhitespace(),
                tag("as"),
                xpwhitespace(),
                singletype_expr::<N, L>(),
            )),
        ),
        |(v, o)| {
            if o.is_none() {
                v
            } else {
                Transform::NotImplemented("cast_expr".to_string())
            }
        },
    ))
}
