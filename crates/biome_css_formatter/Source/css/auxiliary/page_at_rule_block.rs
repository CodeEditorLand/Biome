use biome_css_syntax::{stmt_ext::CssBlockLike, CssPageAtRuleBlock};
use biome_formatter::write;

use crate::{prelude::*, utils::block_like::FormatCssBlockLike};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPageAtRuleBlock;
impl FormatNodeRule<CssPageAtRuleBlock> for FormatCssPageAtRuleBlock {
	fn fmt_fields(&self, node:&CssPageAtRuleBlock, f:&mut CssFormatter) -> FormatResult<()> {
		write!(f, [FormatCssBlockLike::new(&CssBlockLike::from(node.clone()))])
	}

	fn fmt_dangling_comments(
		&self,
		_:&CssPageAtRuleBlock,
		_:&mut CssFormatter,
	) -> FormatResult<()> {
		// Formatted inside of `fmt_fields`
		Ok(())
	}
}
