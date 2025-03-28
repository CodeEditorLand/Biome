use biome_css_syntax::CssBogusParameter;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusParameter;
impl FormatBogusNodeRule<CssBogusParameter> for FormatCssBogusParameter {}
