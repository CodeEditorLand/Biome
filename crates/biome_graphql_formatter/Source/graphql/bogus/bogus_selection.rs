use biome_graphql_syntax::GraphqlBogusSelection;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlBogusSelection;
impl FormatBogusNodeRule<GraphqlBogusSelection> for FormatGraphqlBogusSelection {}
