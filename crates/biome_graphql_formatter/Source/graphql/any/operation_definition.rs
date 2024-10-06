//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_graphql_syntax::AnyGraphqlOperationDefinition;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGraphqlOperationDefinition;
impl FormatRule<AnyGraphqlOperationDefinition> for FormatAnyGraphqlOperationDefinition {
	type Context = GraphqlFormatContext;

	fn fmt(
		&self,
		node:&AnyGraphqlOperationDefinition,
		f:&mut GraphqlFormatter,
	) -> FormatResult<()> {
		match node {
			AnyGraphqlOperationDefinition::GraphqlOperationDefinition(node) => node.format().fmt(f),
			AnyGraphqlOperationDefinition::GraphqlSelectionSet(node) => node.format().fmt(f),
		}
	}
}
