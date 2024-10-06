//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_css_syntax::AnyCssDeclarationOrAtRuleBlock;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssDeclarationOrAtRuleBlock;
impl FormatRule<AnyCssDeclarationOrAtRuleBlock> for FormatAnyCssDeclarationOrAtRuleBlock {
	type Context = CssFormatContext;

	fn fmt(&self, node:&AnyCssDeclarationOrAtRuleBlock, f:&mut CssFormatter) -> FormatResult<()> {
		match node {
			AnyCssDeclarationOrAtRuleBlock::CssBogusBlock(node) => node.format().fmt(f),
			AnyCssDeclarationOrAtRuleBlock::CssDeclarationOrAtRuleBlock(node) => {
				node.format().fmt(f)
			},
		}
	}
}
