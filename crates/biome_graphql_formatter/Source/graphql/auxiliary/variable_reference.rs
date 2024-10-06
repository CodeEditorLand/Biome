use biome_formatter::write;
use biome_graphql_syntax::{GraphqlVariableReference, GraphqlVariableReferenceFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlVariableReference;
impl FormatNodeRule<GraphqlVariableReference> for FormatGraphqlVariableReference {
	fn fmt_fields(
		&self,
		node:&GraphqlVariableReference,
		f:&mut GraphqlFormatter,
	) -> FormatResult<()> {
		let GraphqlVariableReferenceFields { dollar_token, name } = node.as_fields();
		write!(f, [dollar_token.format(), name.format()])
	}
}
