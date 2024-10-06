use biome_graphql_syntax::GraphqlBogusValue;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlBogusValue;
impl FormatBogusNodeRule<GraphqlBogusValue> for FormatGraphqlBogusValue {}
