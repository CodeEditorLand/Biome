use biome_formatter::write;
use biome_js_syntax::{TsReadonlyModifier, TsReadonlyModifierFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct FormatTsReadonlyModifier;

impl FormatNodeRule<TsReadonlyModifier> for FormatTsReadonlyModifier {
	fn fmt_fields(&self, node:&TsReadonlyModifier, f:&mut JsFormatter) -> FormatResult<()> {
		let TsReadonlyModifierFields { modifier_token } = node.as_fields();
		write![f, [modifier_token.format()]]
	}
}
