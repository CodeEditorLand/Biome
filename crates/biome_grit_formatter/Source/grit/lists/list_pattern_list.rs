use biome_grit_syntax::GritListPatternList;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritListPatternList;
impl FormatRule<GritListPatternList> for FormatGritListPatternList {
	type Context = GritFormatContext;

	fn fmt(&self, node:&GritListPatternList, f:&mut GritFormatter) -> FormatResult<()> {
		format_verbatim_node(node.syntax()).fmt(f)
	}
}
