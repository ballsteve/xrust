//! # xdm::xpath
//!
//! An XPath parser.

use lexers::*;

const GRAMMAR: &str = r#"XPath := Expr ;
ParamList := Param (',' Param)* ;
Param := '$' EQName TypeDeclaration? ;
FunctionBody := EnclosedExpr ;
EnclosedExpr := '{' Expr? '}' ;
Expr := ExprSingle (',' ExprSingle)* ;
ExprSingle := ForExpr |
	   LetExpr |
	   QuantifiedExpr |
	   IfExpr |
	   OrExpr ;
ForExpr := SimpleForClause 'return' ExprSingle ;
SimpleForClause := 'for' SimpleForBinding (',' SimpleForBinding)* ;
SimpleForBinding := '$' VarName 'in' ExprSingle ;
LetExpr := SimpleLetClause 'return' ExprSingle ;
SimpleLetClause := 'let' SimpleLetBinding (',' SimpleLetBinding)* ;
SimpleLetBinding := '$' VarName ':=' ExprSingle ;
QuantifiedExpr := ('some' | 'every') '$' VarName 'in' ExprSingle (',' '$' VarName 'in' ExprSingle)* 'satisifes' ExprSingle ;
IfExpr := 'if' '(' Expr ')' 'then' ExprSingle 'else' ExprSingle ;
OrExpr := AndExpr ('or' AndExpr)* ;
AndExpr := ComparisonExpr ('and' ComparisonExpr)* ;
ComparisonExpr := StringConcatExpr ( (ValueComp | GeneralComp | NodeComp) StringConcatExpr)? ;
StringConcatExpr := RangeExpr ('||' RangeExpr)* ;
RangeExpr := AdditiveExpr ('to' AdditiveExpr)? ;
AdditiveExpr := MultiplicativeExpr ( ('+' | '-') MultiplicativeExpr)* ;
MultiplicativeExpr := UnionExpr ( ('*' | 'div' | 'idiv' | 'mod') UnionExpr)* ;
UnionExpr := IntersectExceptExpr ( ('union' | '|') IntersectExceptExpr)* ;
IntersectExceptExpr := InstanceOfExpr ( ('intersect' | 'except') InstanceOfExpr)* ;
InstanceOfExpr := TreatExpr ('instance' 'of' SequenceType)? ;
TreatExpr := CastableExpr ('treat' 'as' SequenceType)? ;
CastableExpr := CastExpr ( 'castable' 'as' SingleType)? ;
CastExpr := ArrowExpr ('cast' 'as' SingleType)? ;
ArrowExpr := UnaryExpr ('=>' ArrowFunctionSpecifier ArgumentList)* ;
UnaryExpr := ('-' | '+')* ValueExpr ;
ValueExpr := SimpleMapExpr ;
GeneralComp := '=' | '!=' | '<' | '<=' | '>' | '>=' ;
ValueComp := 'eq' | 'ne' | 'lt' | 'le' | 'gt' | 'ge' ;
NodeComp := 'is' | '<<' | '>>' ;
SimplMapExpr := PathExpr ('!' PathExpr)* ;
PathExpr := ('/' RelativePathExpr?) | ('//' RelativePathExpr) | RelativePathExpr ;
RelativePathExpr := StepExpr (('/' | '//') StepExpr)* ;
StepExpr := PostfixExpr | AxisStep ;
AxisStep := (ReverseStep | ForwardStep) PredicateList ;
ForwardStep := (ForwardAxis NodeTest) | AbbrevForwardStep ;
ForwardAxis := ('child' '::') |
('descendant' '::') |
('attribute' '::') |
('self' '::') |
('descendant-or-self' '::') |
('following-sibling' '::') |
('following' '::') |
('namespace' '::') ;
AbbrevForwardStep := '@'? NodeTest ;
ReverseStep := (ReverseAxis NodeTest) | AbbrevReverseStep ;
ReverseAxis := ('parent' '::') |
('ancestor' '::') |
('preceding-sibling' '::') |
('preceding' '::') |
('ancestor-or-self' '::') ;
AbbrevReverseStep := '..' ;
NodeTest := KindTest | NameTest ;
NameTest := EQName | Wildcard ;
Wildcard := '*' | (NCName ':*') | ('*:' NCName) | (BracedURILiteral '*') ;
PostfixExpr := PrimaryExpr (Predicate | ArgumentList | Lookup)* ;
ArgumentList := '(' (Argument (',' Argument)*)? ')' ;
PredicateList := Predicate* ;
Predicate := '[' Expr ']' ;
Lookup := '?' KeySpecifier ;
KeySpecifier := NCName | IntegerLiteral | ParenthesizedExpr | '*' ;
ArrowFunctionSpecifier := EQName | VarRef | ParenthesizedExpr ;
PrimaryExpr := Literal | VarRef | ParenthesizedExpr | ContextItemExpr | FunctionCall | FunctionItemExpr | MapConstructor | ArrayConstructor | UnaryLookup ;
Literal := NumericLiteral | StringLiteral ;
NumericLiteral := IntegerLiteral | DecimalLiteral | DoubleLiteral ;
VarRef := '$' VarName ;
VarName := EQName ;
ParenthesizedExpr := '(' Expr? ')' ;
ContextItemExpr := '.' ;
FunctionCall := EQName ArgumentList ;
Argument := ExprSingle | ArgumentPlaceHodler ;
ArgumentPlaceHolder := '?' ;
FunctionItemExpr := NamedFunctionRef | InlineFunctionExpr ;
NamedFunctionRef := EQName '#' IntegerLiteral ;
InlineFunctionExpr := 'function' '(' ParamList? ')' ('as' SequenceType)? FunctionBody ;
MapConstructor := 'map' '{' (MapConstructorEntry (',' MapConstructorEntry)*)? '}' ;
MapConstructorEntry := MapKeyExpr ':' MapValueExpr ;
MapKeyExpr := ExprSingle ;
MapValueExpr := ExprSingle ;
ArrayConstructor := SquareArrayConstructor | CurlyArrayConstructor ;
SquareArrayConstructor := '[' (ExprSingle (',' ExprSingle)*)? ']' ;
CurlyArrayConstructor := 'array' EnclosedExpr ;
UnaryLookup := '?' KeySpecifier ;
SingleType := SimpleTypeName '?'? ;
TypeDeclaration := 'as' SequenceType ;
SequenceType := ('empty-sequence' '(' ')') | (ItemType OccurrenceIndicator?) ;
OccurenceIndicator := '?' | '*' | '+' ;
ItemType := KindTest | ('item' '(' ')') | FunctionTest | MapTest | ArrayTest | AtomicOrUnionType | ParenthesizedItemType ;
AtomicOrUnionType := EQName ;
KindTest := DocumentTest |
ElementTest |
AttributeTest |
SchemaElementTest |
SchemaAttributeTest |
PITest |
CommentTest |
TextTest |
NamespaceNodeTest |
AnyKindTest ;
AnyKindTest := 'node' '(' ')' ;
DocumentTest := 'document-node' '(' (ElementTest | SchemaElementTest)? ')' ;
TextTest := 'text' '(' ')' ;
CommentTest := 'comment' '(' ')' ;
NamespaceNodeTest := 'namespace-node' '(' ')' ;
PITest := 'processing-instruction' '(' (NCName | StringLiteral)? ')' ;
AttributeTest := 'attribute' '(' (AttribNameOrWildcard (',' TypeName)?)? ')' ;
AttribNameOrWildcard := AttributeName | '*' ;
SchemaAttributeTest := 'schema-attribute' '(' AttributeDeclaration ')' ;
AttributeDeclaration := AttributeName ;
ElementTest := 'element' '(' (ElementNameOrWildcard (',' TypeName '?'?)?)? ')' ;
ElementNameOrWildcard := ElementName | '*' ;
SchemaElementTest := 'schema-element' '(' ElementDeclaration ')' ;
ElementDeclaration := ElementName ;
AttributeName := EQName ;
ElementName := EQName ;
SimpleTypeName := TypeName ;
TypeName := EQName ;
FunctionTest := AnyFunctionTest | TypedFunctionTest ;
AnyFunctionTest := 'function' '(' '*' ')' ;
TypedFunctionTest := 'function' '(' (SequenceType (',' SequenceType)*)? ')' 'as' SequenceType ;
MapTest := AnyMapTest | TypedMapTest ;
AnyMapTest := 'map' '(' '*' ')' ;
TypedMapTest := 'map' '(' AtomicOrUnionType ',' SequenceType ')' ;
ArrayTest := AnyArrayTest | TypeArrayTest ;
AnyArrayTest := 'array' '(' '*' ')' ;
TypedArrayTest := 'array' '(' SequenceType ')' ;
ParenthesizedItemType := '(' ItemType ')' ;
EQName := QName | URIQualifiedName ;
IntegerLiteral := Digits ;
DecimalLiteral := ('.' Digits) | (Digits '.' [0-9]*) ;
DoubleLiteral := (('.' Digits) | (Digits ('.' [0-9]*)?)) [eE] [+-]? Digits ;
StringLiteral := ('"' (EscapeQuot | [^"])* '"') | ("'" (EscapeApos | [^'])* "'") ;
URIQualifiedName := BracedURILiteral NCName ;
BracedURILiteral := 'Q' '{' [^{}]* '}' ;
EscapeQuot := '""' ;
EscapeApos := "''" ;
Comment := '(:' (CommentContents | Comment)* ':)' ;
QName := PrefixedName | UnprefixedName ;
PrefixedName := Prefix ':' LocalPart ;
UnprefixedName := LocalPart ;
Prefix := NCName ;
LocalPart := NCName ;
NCName := Name - (Char* ':' Char*) ;
Char := #x9 | #xA |#xD | [#x20-#xD7FF] | [#xE000-#xFFFD | [#x10000-#x10FFFF] ;
Digits := [0-9]+ ;
CommentContents := (Char+ - (Char * ('(:' | ':)') Char *)) ;
NameStartChar := ':' | [A-Z] | '_' | [a-z] | [#xC0-#xD6] | [#xD8-#xF6] | [#xF8-#x2FF] | [#x370-#x37D] | [#x37F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF] ;
NameChar := NameStartChar | '-' | '.' | [0-9] | #xB7 | [#x0300-#x036F] | [#x203F-#x2040] ;
Name := NameStartChar (NameChar)* ;
"#;

pub fn parse(e: &str) {
    let mut tok = EbnfTokenizer::new(GRAMMAR.chars());
    let mut lx = tok(e.chars());
    for t in lx.iter() {
        println!("Got token {}", t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xpath_parse_int_sequence() {
        parse("(1, 2, 3)");
    }
    #[test]
    fn xpath_parse_path() {
        parse("/a/b/c");
    }

}

