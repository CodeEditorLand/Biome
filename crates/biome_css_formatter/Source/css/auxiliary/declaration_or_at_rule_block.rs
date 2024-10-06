use biome_css_syntax::{stmt_ext::CssBlockLike, CssDeclarationOrAtRuleBlock};
use biome_formatter::write;

use crate::{prelude::*, utils::block_like::FormatCssBlockLike};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationOrAtRuleBlock;
impl FormatNodeRule<CssDeclarationOrAtRuleBlock> for FormatCssDeclarationOrAtRuleBlock {
	fn fmt_fields(
		&self,
		node:&CssDeclarationOrAtRuleBlock,
		f:&mut CssFormatter,
	) -> FormatResult<()> {
		write!(f, [FormatCssBlockLike::new(&CssBlockLike::from(node.clone()))])
	}

	fn fmt_dangling_comments(
		&self,
		_:&CssDeclarationOrAtRuleBlock,
		_:&mut CssFormatter,
	) -> FormatResult<()> {
		// Formatted inside of `fmt_fields`
		Ok(())
	}
}
