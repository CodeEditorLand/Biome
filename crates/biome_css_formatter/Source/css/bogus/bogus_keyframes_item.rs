use biome_css_syntax::CssBogusKeyframesItem;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusKeyframesItem;
impl FormatBogusNodeRule<CssBogusKeyframesItem> for FormatCssBogusKeyframesItem {}
