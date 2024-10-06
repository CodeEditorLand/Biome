//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_css_syntax::AnyCssSupportsAndCombinableCondition;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSupportsAndCombinableCondition;
impl FormatRule<AnyCssSupportsAndCombinableCondition>
	for FormatAnyCssSupportsAndCombinableCondition
{
	type Context = CssFormatContext;

	fn fmt(
		&self,
		node:&AnyCssSupportsAndCombinableCondition,
		f:&mut CssFormatter,
	) -> FormatResult<()> {
		match node {
			AnyCssSupportsAndCombinableCondition::AnyCssSupportsInParens(node) => {
				node.format().fmt(f)
			},
			AnyCssSupportsAndCombinableCondition::CssSupportsAndCondition(node) => {
				node.format().fmt(f)
			},
		}
	}
}
