use biome_css_syntax::CssSubSelectorList;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSubSelectorList;
impl FormatRule<CssSubSelectorList> for FormatCssSubSelectorList {
	type Context = CssFormatContext;

	fn fmt(&self, node:&CssSubSelectorList, f:&mut CssFormatter) -> FormatResult<()> {
		f.join().entries(node.iter().formatted()).finish()
	}
}
