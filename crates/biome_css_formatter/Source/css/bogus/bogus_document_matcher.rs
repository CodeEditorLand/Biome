use biome_css_syntax::CssBogusDocumentMatcher;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusDocumentMatcher;
impl FormatBogusNodeRule<CssBogusDocumentMatcher> for FormatCssBogusDocumentMatcher {}
