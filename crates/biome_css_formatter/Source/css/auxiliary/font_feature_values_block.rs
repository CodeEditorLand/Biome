use biome_css_syntax::{stmt_ext::CssBlockLike, CssFontFeatureValuesBlock};
use biome_formatter::write;

use crate::{prelude::*, utils::block_like::FormatCssBlockLike};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontFeatureValuesBlock;
impl FormatNodeRule<CssFontFeatureValuesBlock> for FormatCssFontFeatureValuesBlock {
	fn fmt_fields(&self, node:&CssFontFeatureValuesBlock, f:&mut CssFormatter) -> FormatResult<()> {
		write!(f, [FormatCssBlockLike::new(&CssBlockLike::from(node.clone()))])
	}

	fn fmt_dangling_comments(
		&self,
		_:&CssFontFeatureValuesBlock,
		_:&mut CssFormatter,
	) -> FormatResult<()> {
		// Formatted inside of `fmt_fields`
		Ok(())
	}
}
