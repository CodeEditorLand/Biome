//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_js_syntax::AnyJsExportNamedSpecifier;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsExportNamedSpecifier;
impl FormatRule<AnyJsExportNamedSpecifier> for FormatAnyJsExportNamedSpecifier {
	type Context = JsFormatContext;

	fn fmt(&self, node:&AnyJsExportNamedSpecifier, f:&mut JsFormatter) -> FormatResult<()> {
		match node {
			AnyJsExportNamedSpecifier::JsExportNamedShorthandSpecifier(node) => {
				node.format().fmt(f)
			},
			AnyJsExportNamedSpecifier::JsExportNamedSpecifier(node) => node.format().fmt(f),
		}
	}
}
