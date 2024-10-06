use biome_css_syntax::CssBogusUnicodeRangeValue;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusUnicodeRangeValue;
impl FormatBogusNodeRule<CssBogusUnicodeRangeValue> for FormatCssBogusUnicodeRangeValue {}
