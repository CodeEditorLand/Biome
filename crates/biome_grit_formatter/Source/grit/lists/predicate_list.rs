use biome_grit_syntax::GritPredicateList;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateList;
impl FormatRule<GritPredicateList> for FormatGritPredicateList {
	type Context = GritFormatContext;

	fn fmt(&self, node:&GritPredicateList, f:&mut GritFormatter) -> FormatResult<()> {
		format_verbatim_node(node.syntax()).fmt(f)
	}
}
