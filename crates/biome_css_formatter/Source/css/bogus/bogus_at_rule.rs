use biome_css_syntax::CssBogusAtRule;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusAtRule;
impl FormatBogusNodeRule<CssBogusAtRule> for FormatCssBogusAtRule {}
