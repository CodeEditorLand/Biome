//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_js_syntax::AnyTsEnumMemberName;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsEnumMemberName;
impl FormatRule<AnyTsEnumMemberName> for FormatAnyTsEnumMemberName {
	type Context = JsFormatContext;

	fn fmt(&self, node:&AnyTsEnumMemberName, f:&mut JsFormatter) -> FormatResult<()> {
		match node {
			AnyTsEnumMemberName::JsComputedMemberName(node) => node.format().fmt(f),
			AnyTsEnumMemberName::TsLiteralEnumMemberName(node) => node.format().fmt(f),
		}
	}
}
