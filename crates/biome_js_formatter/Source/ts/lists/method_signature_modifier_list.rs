use biome_js_syntax::TsMethodSignatureModifierList;

use crate::{prelude::*, utils::format_modifiers::FormatModifiers};

#[derive(Debug, Clone, Default)]
pub struct FormatTsMethodSignatureModifierList;

impl FormatRule<TsMethodSignatureModifierList> for FormatTsMethodSignatureModifierList {
	type Context = JsFormatContext;

	fn fmt(&self, node:&TsMethodSignatureModifierList, f:&mut JsFormatter) -> FormatResult<()> {
		FormatModifiers::from(node.clone()).fmt(f)
	}
}
