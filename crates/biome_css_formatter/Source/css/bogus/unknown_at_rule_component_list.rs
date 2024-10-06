use biome_css_syntax::CssUnknownAtRuleComponentList;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnknownAtRuleComponentList;
impl FormatBogusNodeRule<CssUnknownAtRuleComponentList> for FormatCssUnknownAtRuleComponentList {}
