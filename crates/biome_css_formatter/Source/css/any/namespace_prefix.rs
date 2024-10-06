//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_css_syntax::AnyCssNamespacePrefix;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssNamespacePrefix;
impl FormatRule<AnyCssNamespacePrefix> for FormatAnyCssNamespacePrefix {
	type Context = CssFormatContext;

	fn fmt(&self, node:&AnyCssNamespacePrefix, f:&mut CssFormatter) -> FormatResult<()> {
		match node {
			AnyCssNamespacePrefix::CssNamedNamespacePrefix(node) => node.format().fmt(f),
			AnyCssNamespacePrefix::CssUniversalNamespacePrefix(node) => node.format().fmt(f),
		}
	}
}
