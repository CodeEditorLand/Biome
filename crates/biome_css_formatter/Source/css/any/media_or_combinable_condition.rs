//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_css_syntax::AnyCssMediaOrCombinableCondition;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssMediaOrCombinableCondition;
impl FormatRule<AnyCssMediaOrCombinableCondition> for FormatAnyCssMediaOrCombinableCondition {
	type Context = CssFormatContext;

	fn fmt(&self, node:&AnyCssMediaOrCombinableCondition, f:&mut CssFormatter) -> FormatResult<()> {
		match node {
			AnyCssMediaOrCombinableCondition::AnyCssMediaInParens(node) => node.format().fmt(f),
			AnyCssMediaOrCombinableCondition::CssMediaOrCondition(node) => node.format().fmt(f),
		}
	}
}
