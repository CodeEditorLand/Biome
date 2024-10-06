use biome_js_syntax::JsPropertyModifierList;

use crate::{prelude::*, utils::format_modifiers::FormatModifiers};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsPropertyModifierList;

impl FormatRule<JsPropertyModifierList> for FormatJsPropertyModifierList {
	type Context = JsFormatContext;

	fn fmt(&self, node:&JsPropertyModifierList, f:&mut JsFormatter) -> FormatResult<()> {
		FormatModifiers::from(node.clone()).fmt(f)
	}
}
