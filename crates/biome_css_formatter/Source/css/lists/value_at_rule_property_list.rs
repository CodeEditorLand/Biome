use biome_css_syntax::CssValueAtRulePropertyList;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRulePropertyList;
impl FormatRule<CssValueAtRulePropertyList> for FormatCssValueAtRulePropertyList {
	type Context = CssFormatContext;

	fn fmt(&self, node:&CssValueAtRulePropertyList, f:&mut CssFormatter) -> FormatResult<()> {
		f.join().entries(node.iter().formatted()).finish()
	}
}
