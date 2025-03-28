//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_css_syntax::AnyCssImportSupportsCondition;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssImportSupportsCondition;
impl FormatRule<AnyCssImportSupportsCondition> for FormatAnyCssImportSupportsCondition {
	type Context = CssFormatContext;

	fn fmt(&self, node:&AnyCssImportSupportsCondition, f:&mut CssFormatter) -> FormatResult<()> {
		match node {
			AnyCssImportSupportsCondition::AnyCssSupportsCondition(node) => node.format().fmt(f),
			AnyCssImportSupportsCondition::CssDeclaration(node) => node.format().fmt(f),
		}
	}
}
