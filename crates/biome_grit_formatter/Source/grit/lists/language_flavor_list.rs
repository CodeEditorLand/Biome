use biome_grit_syntax::GritLanguageFlavorList;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritLanguageFlavorList;
impl FormatRule<GritLanguageFlavorList> for FormatGritLanguageFlavorList {
	type Context = GritFormatContext;

	fn fmt(&self, node:&GritLanguageFlavorList, f:&mut GritFormatter) -> FormatResult<()> {
		format_verbatim_node(node.syntax()).fmt(f)
	}
}
