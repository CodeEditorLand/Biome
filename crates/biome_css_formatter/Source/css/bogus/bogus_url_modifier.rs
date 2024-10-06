use biome_css_syntax::CssBogusUrlModifier;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusUrlModifier;
impl FormatBogusNodeRule<CssBogusUrlModifier> for FormatCssBogusUrlModifier {}
