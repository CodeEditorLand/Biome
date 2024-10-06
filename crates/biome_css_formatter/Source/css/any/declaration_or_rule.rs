//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_css_syntax::AnyCssDeclarationOrRule;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssDeclarationOrRule;
impl FormatRule<AnyCssDeclarationOrRule> for FormatAnyCssDeclarationOrRule {
	type Context = CssFormatContext;

	fn fmt(&self, node:&AnyCssDeclarationOrRule, f:&mut CssFormatter) -> FormatResult<()> {
		match node {
			AnyCssDeclarationOrRule::AnyCssRule(node) => node.format().fmt(f),
			AnyCssDeclarationOrRule::CssBogus(node) => node.format().fmt(f),
			AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(node) => node.format().fmt(f),
			AnyCssDeclarationOrRule::CssMetavariable(node) => node.format().fmt(f),
		}
	}
}
