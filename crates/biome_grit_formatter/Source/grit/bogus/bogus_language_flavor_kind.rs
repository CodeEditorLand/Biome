use biome_grit_syntax::GritBogusLanguageFlavorKind;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusLanguageFlavorKind;
impl FormatBogusNodeRule<GritBogusLanguageFlavorKind> for FormatGritBogusLanguageFlavorKind {}
