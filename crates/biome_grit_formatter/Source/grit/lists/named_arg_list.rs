use biome_grit_syntax::GritNamedArgList;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritNamedArgList;
impl FormatRule<GritNamedArgList> for FormatGritNamedArgList {
	type Context = GritFormatContext;

	fn fmt(&self, node:&GritNamedArgList, f:&mut GritFormatter) -> FormatResult<()> {
		format_verbatim_node(node.syntax()).fmt(f)
	}
}
