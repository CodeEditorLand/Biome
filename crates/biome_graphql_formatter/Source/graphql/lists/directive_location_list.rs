use biome_graphql_syntax::GraphqlDirectiveLocationList;

use crate::{prelude::*, utils::list::write_interface_like_list};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlDirectiveLocationList;
impl FormatRule<GraphqlDirectiveLocationList> for FormatGraphqlDirectiveLocationList {
	type Context = GraphqlFormatContext;

	fn fmt(&self, node:&GraphqlDirectiveLocationList, f:&mut GraphqlFormatter) -> FormatResult<()> {
		write_interface_like_list(node, f)
	}
}
