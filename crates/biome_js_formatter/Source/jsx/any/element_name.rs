//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_js_syntax::AnyJsxElementName;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsxElementName;
impl FormatRule<AnyJsxElementName> for FormatAnyJsxElementName {
	type Context = JsFormatContext;

	fn fmt(&self, node:&AnyJsxElementName, f:&mut JsFormatter) -> FormatResult<()> {
		match node {
			AnyJsxElementName::JsxMemberName(node) => node.format().fmt(f),
			AnyJsxElementName::JsxName(node) => node.format().fmt(f),
			AnyJsxElementName::JsxNamespaceName(node) => node.format().fmt(f),
			AnyJsxElementName::JsxReferenceIdentifier(node) => node.format().fmt(f),
		}
	}
}
