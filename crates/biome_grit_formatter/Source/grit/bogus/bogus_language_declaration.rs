use biome_grit_syntax::GritBogusLanguageDeclaration;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusLanguageDeclaration;
impl FormatBogusNodeRule<GritBogusLanguageDeclaration> for FormatGritBogusLanguageDeclaration {}
