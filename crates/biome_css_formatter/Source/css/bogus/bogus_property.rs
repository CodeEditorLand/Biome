use biome_css_syntax::CssBogusProperty;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusProperty;
impl FormatBogusNodeRule<CssBogusProperty> for FormatCssBogusProperty {}
