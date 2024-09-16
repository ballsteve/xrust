//! Functions for handling variables.

use crate::item::Node;
use crate::parser::combinators::map::{map, map_with_state};
use crate::parser::combinators::pair::pair;
use crate::parser::combinators::tag::tag;
//use crate::parser::combinators::debug::inspect;
use crate::parser::xpath::nodetests::qualname_test;
use crate::parser::xpath::support::get_nt_localname;
use crate::parser::{ParseError, ParseInput};
use crate::transform::{Transform, in_scope_namespaces};

// VarRef ::= '$' VarName
pub(crate) fn variable_reference<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(map_with_state(pair(tag("$"), qualname_test()), |(_, qn), state| {
        Transform::VariableReference(get_nt_localname(&qn), in_scope_namespaces(state.cur.clone()))
    }))
}
