use biome_css_syntax::CssBogusPropertyValue;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusPropertyValue;
impl FormatBogusNodeRule<CssBogusPropertyValue> for FormatCssBogusPropertyValue {}
