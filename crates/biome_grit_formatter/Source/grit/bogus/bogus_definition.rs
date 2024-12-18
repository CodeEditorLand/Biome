use biome_grit_syntax::GritBogusDefinition;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusDefinition;
impl FormatBogusNodeRule<GritBogusDefinition> for FormatGritBogusDefinition {}
