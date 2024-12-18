use biome_css_syntax::CssValueAtRuleGenericValue;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRuleGenericValue;
impl FormatBogusNodeRule<CssValueAtRuleGenericValue> for FormatCssValueAtRuleGenericValue {}
