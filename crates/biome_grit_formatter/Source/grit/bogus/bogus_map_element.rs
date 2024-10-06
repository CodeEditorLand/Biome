use biome_grit_syntax::GritBogusMapElement;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusMapElement;
impl FormatBogusNodeRule<GritBogusMapElement> for FormatGritBogusMapElement {}
