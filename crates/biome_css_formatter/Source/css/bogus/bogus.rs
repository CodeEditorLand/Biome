use biome_css_syntax::CssBogus;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogus;
impl FormatBogusNodeRule<CssBogus> for FormatCssBogus {}
