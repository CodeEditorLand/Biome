use biome_grit_syntax::GritBogusPattern;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusPattern;
impl FormatBogusNodeRule<GritBogusPattern> for FormatGritBogusPattern {}
