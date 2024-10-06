use biome_css_syntax::{stmt_ext::CssBlockLike, CssDeclarationBlock};
use biome_formatter::write;

use crate::{prelude::*, utils::block_like::FormatCssBlockLike};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationBlock;
impl FormatNodeRule<CssDeclarationBlock> for FormatCssDeclarationBlock {
	fn fmt_fields(&self, node:&CssDeclarationBlock, f:&mut CssFormatter) -> FormatResult<()> {
		write!(f, [FormatCssBlockLike::new(&CssBlockLike::from(node.clone()))])
	}

	fn fmt_dangling_comments(
		&self,
		_:&CssDeclarationBlock,
		_:&mut CssFormatter,
	) -> FormatResult<()> {
		// Formatted inside of `fmt_fields`
		Ok(())
	}
}
