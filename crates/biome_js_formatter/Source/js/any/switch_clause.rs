//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_js_syntax::AnyJsSwitchClause;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsSwitchClause;
impl FormatRule<AnyJsSwitchClause> for FormatAnyJsSwitchClause {
	type Context = JsFormatContext;

	fn fmt(&self, node:&AnyJsSwitchClause, f:&mut JsFormatter) -> FormatResult<()> {
		match node {
			AnyJsSwitchClause::JsCaseClause(node) => node.format().fmt(f),
			AnyJsSwitchClause::JsDefaultClause(node) => node.format().fmt(f),
		}
	}
}
