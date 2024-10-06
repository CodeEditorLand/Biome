use biome_css_syntax::CssBogusPseudoClass;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusPseudoClass;
impl FormatBogusNodeRule<CssBogusPseudoClass> for FormatCssBogusPseudoClass {}
