use biome_grit_syntax::GritBubbleScope;
use biome_rowan::AstNode;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBubbleScope;
impl FormatNodeRule<GritBubbleScope> for FormatGritBubbleScope {
	fn fmt_fields(&self, node:&GritBubbleScope, f:&mut GritFormatter) -> FormatResult<()> {
		format_verbatim_node(node.syntax()).fmt(f)
	}
}
