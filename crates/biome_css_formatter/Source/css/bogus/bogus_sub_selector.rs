use biome_css_syntax::CssBogusSubSelector;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusSubSelector;
impl FormatBogusNodeRule<CssBogusSubSelector> for FormatCssBogusSubSelector {}
