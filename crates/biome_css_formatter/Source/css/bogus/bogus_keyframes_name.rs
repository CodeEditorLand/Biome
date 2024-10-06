use biome_css_syntax::CssBogusKeyframesName;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusKeyframesName;
impl FormatBogusNodeRule<CssBogusKeyframesName> for FormatCssBogusKeyframesName {}
