use biome_css_syntax::CssBogusScopeRange;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusScopeRange;
impl FormatBogusNodeRule<CssBogusScopeRange> for FormatCssBogusScopeRange {}
