use biome_grit_syntax::GritNot;
use biome_rowan::AstNode;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritNot;
impl FormatNodeRule<GritNot> for FormatGritNot {
	fn fmt_fields(&self, node:&GritNot, f:&mut GritFormatter) -> FormatResult<()> {
		format_verbatim_node(node.syntax()).fmt(f)
	}
}
