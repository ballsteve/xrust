//! Functions for handling variables.

use crate::item::Node;
use crate::parser::combinators::map::map_with_state;
use crate::parser::combinators::pair::pair;
use crate::parser::combinators::tag::tag;
//use crate::parser::combinators::debug::inspect;
use crate::parser::xpath::nodetests::qualname_test;
use crate::parser::xpath::support::get_nt_localname;
use crate::parser::{ParseError, ParseInput, StaticState};
use crate::transform::{Transform, in_scope_namespaces};
use qualname::{NamespacePrefix, NamespaceUri};

// VarRef ::= '$' VarName
pub(crate) fn variable_reference<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map_with_state(
        pair(tag("$"), qualname_test()),
        |(_, qn), state, _ss| {
            Transform::VariableReference(
                get_nt_localname(&qn),
                in_scope_namespaces(state.cur.clone()),
            )
        },
    ))
}
