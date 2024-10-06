use biome_formatter::write;
use biome_js_syntax::{JsStaticModifier, JsStaticModifierFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsStaticModifier;

impl FormatNodeRule<JsStaticModifier> for FormatJsStaticModifier {
	fn fmt_fields(&self, node:&JsStaticModifier, f:&mut JsFormatter) -> FormatResult<()> {
		let JsStaticModifierFields { modifier_token } = node.as_fields();

		write![f, [modifier_token.format()]]
	}
}
