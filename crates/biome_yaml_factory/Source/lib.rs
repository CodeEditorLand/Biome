use biome_rowan::TreeBuilder;
use biome_yaml_syntax::YamlLanguage;

mod generated;
pub mod make;
// Re-exported for tests
#[doc(hidden)]
pub use biome_yaml_syntax as syntax;

pub use crate::generated::YamlSyntaxFactory;

pub type YamlSyntaxTreeBuilder = TreeBuilder<'static, YamlLanguage, YamlSyntaxFactory>;
