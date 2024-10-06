use biome_grit_syntax::GritBogus;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogus;
impl FormatBogusNodeRule<GritBogus> for FormatGritBogus {}
