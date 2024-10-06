use biome_grit_syntax::GritBogusContainer;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusContainer;
impl FormatBogusNodeRule<GritBogusContainer> for FormatGritBogusContainer {}
