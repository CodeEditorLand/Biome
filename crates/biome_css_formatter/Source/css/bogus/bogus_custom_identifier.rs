use biome_css_syntax::CssBogusCustomIdentifier;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusCustomIdentifier;
impl FormatBogusNodeRule<CssBogusCustomIdentifier> for FormatCssBogusCustomIdentifier {}
