use biome_css_syntax::CssBogusSelector;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusSelector;
impl FormatBogusNodeRule<CssBogusSelector> for FormatCssBogusSelector {}
