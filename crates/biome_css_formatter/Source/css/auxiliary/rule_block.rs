use biome_css_syntax::{stmt_ext::CssBlockLike, CssRuleBlock};
use biome_formatter::write;

use crate::{prelude::*, utils::block_like::FormatCssBlockLike};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRuleBlock;
impl FormatNodeRule<CssRuleBlock> for FormatCssRuleBlock {
	fn fmt_fields(&self, node:&CssRuleBlock, f:&mut CssFormatter) -> FormatResult<()> {
		write!(f, [FormatCssBlockLike::new(&CssBlockLike::from(node.clone()))])
	}

	fn fmt_dangling_comments(&self, _:&CssRuleBlock, _:&mut CssFormatter) -> FormatResult<()> {
		// Formatted inside of `fmt_fields`
		Ok(())
	}
}
