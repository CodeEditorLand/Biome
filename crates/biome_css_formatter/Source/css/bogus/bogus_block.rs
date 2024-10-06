use biome_css_syntax::CssBogusBlock;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusBlock;
impl FormatBogusNodeRule<CssBogusBlock> for FormatCssBogusBlock {}
