//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_css_syntax::AnyCssProperty;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssProperty;
impl FormatRule<AnyCssProperty> for FormatAnyCssProperty {
	type Context = CssFormatContext;

	fn fmt(&self, node:&AnyCssProperty, f:&mut CssFormatter) -> FormatResult<()> {
		match node {
			AnyCssProperty::CssBogusProperty(node) => node.format().fmt(f),
			AnyCssProperty::CssComposesProperty(node) => node.format().fmt(f),
			AnyCssProperty::CssGenericProperty(node) => node.format().fmt(f),
		}
	}
}
