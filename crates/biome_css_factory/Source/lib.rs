use biome_css_syntax::CssLanguage;
use biome_rowan::TreeBuilder;

pub use crate::generated::CssSyntaxFactory;

mod generated;

// Re-exported for tests
#[doc(hidden)]
pub use biome_css_syntax as syntax;

pub type CssSyntaxTreeBuilder = TreeBuilder<'static, CssLanguage, CssSyntaxFactory>;

pub use generated::node_factory as make;
