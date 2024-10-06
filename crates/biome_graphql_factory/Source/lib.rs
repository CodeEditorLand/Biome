use biome_graphql_syntax::GraphqlLanguage;
use biome_rowan::TreeBuilder;

mod generated;
// Re-exported for tests
#[doc(hidden)]
pub use biome_graphql_syntax as syntax;

pub use crate::generated::GraphqlSyntaxFactory;

pub type GritSyntaxTreeBuilder = TreeBuilder<'static, GraphqlLanguage, GraphqlSyntaxFactory>;
