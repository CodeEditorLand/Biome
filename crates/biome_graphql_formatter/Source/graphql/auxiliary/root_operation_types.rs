use biome_formatter::write;
use biome_graphql_syntax::{GraphqlRootOperationTypes, GraphqlRootOperationTypesFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlRootOperationTypes;
impl FormatNodeRule<GraphqlRootOperationTypes> for FormatGraphqlRootOperationTypes {
	fn fmt_fields(
		&self,
		node:&GraphqlRootOperationTypes,
		f:&mut GraphqlFormatter,
	) -> FormatResult<()> {
		let GraphqlRootOperationTypesFields { l_curly_token, root_operation_type, r_curly_token } =
			node.as_fields();

		write!(
			f,
			[
				l_curly_token.format(),
				block_indent(&root_operation_type.format()),
				r_curly_token.format(),
			]
		)
	}
}
