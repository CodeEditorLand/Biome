use biome_grit_syntax::GritBogusLiteral;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusLiteral;
impl FormatBogusNodeRule<GritBogusLiteral> for FormatGritBogusLiteral {}
