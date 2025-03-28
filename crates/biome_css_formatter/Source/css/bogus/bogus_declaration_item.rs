use biome_css_syntax::CssBogusDeclarationItem;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusDeclarationItem;
impl FormatBogusNodeRule<CssBogusDeclarationItem> for FormatCssBogusDeclarationItem {}
