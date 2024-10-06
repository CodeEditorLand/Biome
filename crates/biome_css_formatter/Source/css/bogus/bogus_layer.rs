use biome_css_syntax::CssBogusLayer;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusLayer;
impl FormatBogusNodeRule<CssBogusLayer> for FormatCssBogusLayer {}
