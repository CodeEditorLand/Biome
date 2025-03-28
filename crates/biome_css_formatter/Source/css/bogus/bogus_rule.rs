use biome_css_syntax::CssBogusRule;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusRule;
impl FormatBogusNodeRule<CssBogusRule> for FormatCssBogusRule {}
