use biome_markdown_syntax::MarkdownLanguage;
use biome_rowan::TreeBuilder;

mod generated;
// Re-exported for tests
#[doc(hidden)]
pub use biome_markdown_syntax as syntax;

pub use crate::generated::MarkdownSyntaxFactory;

pub type DemoSyntaxTreeBuilder = TreeBuilder<'static, MarkdownLanguage, MarkdownSyntaxFactory>;

pub mod make;
