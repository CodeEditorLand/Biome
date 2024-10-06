use biome_css_syntax::CssBogusMediaQuery;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusMediaQuery;
impl FormatBogusNodeRule<CssBogusMediaQuery> for FormatCssBogusMediaQuery {}
