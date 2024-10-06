//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_css_syntax::AnyCssPseudoValue;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssPseudoValue;
impl FormatRule<AnyCssPseudoValue> for FormatAnyCssPseudoValue {
	type Context = CssFormatContext;

	fn fmt(&self, node:&AnyCssPseudoValue, f:&mut CssFormatter) -> FormatResult<()> {
		match node {
			AnyCssPseudoValue::CssIdentifier(node) => node.format().fmt(f),
			AnyCssPseudoValue::CssString(node) => node.format().fmt(f),
		}
	}
}
