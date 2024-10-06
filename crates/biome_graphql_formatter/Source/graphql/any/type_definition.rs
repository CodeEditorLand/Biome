//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_graphql_syntax::AnyGraphqlTypeDefinition;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGraphqlTypeDefinition;
impl FormatRule<AnyGraphqlTypeDefinition> for FormatAnyGraphqlTypeDefinition {
	type Context = GraphqlFormatContext;

	fn fmt(&self, node:&AnyGraphqlTypeDefinition, f:&mut GraphqlFormatter) -> FormatResult<()> {
		match node {
			AnyGraphqlTypeDefinition::GraphqlEnumTypeDefinition(node) => node.format().fmt(f),
			AnyGraphqlTypeDefinition::GraphqlInputObjectTypeDefinition(node) => {
				node.format().fmt(f)
			},
			AnyGraphqlTypeDefinition::GraphqlInterfaceTypeDefinition(node) => node.format().fmt(f),
			AnyGraphqlTypeDefinition::GraphqlObjectTypeDefinition(node) => node.format().fmt(f),
			AnyGraphqlTypeDefinition::GraphqlScalarTypeDefinition(node) => node.format().fmt(f),
			AnyGraphqlTypeDefinition::GraphqlUnionTypeDefinition(node) => node.format().fmt(f),
		}
	}
}
