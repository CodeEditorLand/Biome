use biome_js_syntax::JsMethodModifierList;

use crate::{prelude::*, utils::format_modifiers::FormatModifiers};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsMethodModifierList;

impl FormatRule<JsMethodModifierList> for FormatJsMethodModifierList {
	type Context = JsFormatContext;

	fn fmt(&self, node:&JsMethodModifierList, f:&mut JsFormatter) -> FormatResult<()> {
		FormatModifiers::from(node.clone()).fmt(f)
	}
}
