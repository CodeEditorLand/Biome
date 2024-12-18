use biome_graphql_syntax::GraphqlBogusDefinition;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlBogusDefinition;
impl FormatBogusNodeRule<GraphqlBogusDefinition> for FormatGraphqlBogusDefinition {}
