use biome_grit_syntax::GritDefinitionList;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritDefinitionList;
impl FormatRule<GritDefinitionList> for FormatGritDefinitionList {
	type Context = GritFormatContext;

	fn fmt(&self, node:&GritDefinitionList, f:&mut GritFormatter) -> FormatResult<()> {
		format_verbatim_node(node.syntax()).fmt(f)
	}
}
