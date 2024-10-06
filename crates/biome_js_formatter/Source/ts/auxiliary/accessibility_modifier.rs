use biome_formatter::write;
use biome_js_syntax::{TsAccessibilityModifier, TsAccessibilityModifierFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct FormatTsAccessibilityModifier;

impl FormatNodeRule<TsAccessibilityModifier> for FormatTsAccessibilityModifier {
	fn fmt_fields(&self, node:&TsAccessibilityModifier, f:&mut JsFormatter) -> FormatResult<()> {
		let TsAccessibilityModifierFields { modifier_token } = node.as_fields();

		write![f, [modifier_token.format()]]
	}
}
