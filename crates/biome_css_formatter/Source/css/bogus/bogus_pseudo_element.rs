use biome_css_syntax::CssBogusPseudoElement;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusPseudoElement;
impl FormatBogusNodeRule<CssBogusPseudoElement> for FormatCssBogusPseudoElement {}
