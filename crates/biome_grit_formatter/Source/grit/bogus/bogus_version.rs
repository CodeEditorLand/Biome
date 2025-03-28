use biome_grit_syntax::GritBogusVersion;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusVersion;
impl FormatBogusNodeRule<GritBogusVersion> for FormatGritBogusVersion {}
