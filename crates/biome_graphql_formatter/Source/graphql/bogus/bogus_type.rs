use biome_graphql_syntax::GraphqlBogusType;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlBogusType;
impl FormatBogusNodeRule<GraphqlBogusType> for FormatGraphqlBogusType {}
