//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_js_syntax::AnyJsImportAssertionEntry;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsImportAssertionEntry;
impl FormatRule<AnyJsImportAssertionEntry> for FormatAnyJsImportAssertionEntry {
	type Context = JsFormatContext;

	fn fmt(&self, node:&AnyJsImportAssertionEntry, f:&mut JsFormatter) -> FormatResult<()> {
		match node {
			AnyJsImportAssertionEntry::JsBogusImportAssertionEntry(node) => node.format().fmt(f),
			AnyJsImportAssertionEntry::JsImportAssertionEntry(node) => node.format().fmt(f),
		}
	}
}
