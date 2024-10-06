use biome_css_syntax::{stmt_ext::CssBlockLike, CssDeclarationOrRuleBlock};
use biome_formatter::write;

use crate::{prelude::*, utils::block_like::FormatCssBlockLike};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationOrRuleBlock;
impl FormatNodeRule<CssDeclarationOrRuleBlock> for FormatCssDeclarationOrRuleBlock {
	fn fmt_fields(&self, node:&CssDeclarationOrRuleBlock, f:&mut CssFormatter) -> FormatResult<()> {
		write!(f, [FormatCssBlockLike::new(&CssBlockLike::from(node.clone()))])
	}

	fn fmt_dangling_comments(
		&self,
		_:&CssDeclarationOrRuleBlock,
		_:&mut CssFormatter,
	) -> FormatResult<()> {
		// Formatted inside of `fmt_fields`
		Ok(())
	}
}
