use biome_css_syntax::CssBogusFontFeatureValuesItem;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusFontFeatureValuesItem;
impl FormatBogusNodeRule<CssBogusFontFeatureValuesItem> for FormatCssBogusFontFeatureValuesItem {}
