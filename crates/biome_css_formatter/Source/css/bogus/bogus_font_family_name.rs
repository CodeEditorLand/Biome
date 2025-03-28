use biome_css_syntax::CssBogusFontFamilyName;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusFontFamilyName;
impl FormatBogusNodeRule<CssBogusFontFamilyName> for FormatCssBogusFontFamilyName {}
