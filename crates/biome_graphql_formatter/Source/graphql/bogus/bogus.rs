use biome_graphql_syntax::GraphqlBogus;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlBogus;
impl FormatBogusNodeRule<GraphqlBogus> for FormatGraphqlBogus {}
