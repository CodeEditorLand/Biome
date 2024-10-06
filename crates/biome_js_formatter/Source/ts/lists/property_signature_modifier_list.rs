use biome_js_syntax::TsPropertySignatureModifierList;

use crate::{prelude::*, utils::format_modifiers::FormatModifiers};

#[derive(Debug, Clone, Default)]
pub struct FormatTsPropertySignatureModifierList;

impl FormatRule<TsPropertySignatureModifierList> for FormatTsPropertySignatureModifierList {
	type Context = JsFormatContext;

	fn fmt(&self, node:&TsPropertySignatureModifierList, f:&mut JsFormatter) -> FormatResult<()> {
		FormatModifiers::from(node.clone()).fmt(f)
	}
}
