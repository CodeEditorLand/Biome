//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_css_syntax::AnyCssKeyframesBlock;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssKeyframesBlock;
impl FormatRule<AnyCssKeyframesBlock> for FormatAnyCssKeyframesBlock {
	type Context = CssFormatContext;

	fn fmt(&self, node:&AnyCssKeyframesBlock, f:&mut CssFormatter) -> FormatResult<()> {
		match node {
			AnyCssKeyframesBlock::CssBogusBlock(node) => node.format().fmt(f),
			AnyCssKeyframesBlock::CssKeyframesBlock(node) => node.format().fmt(f),
		}
	}
}
