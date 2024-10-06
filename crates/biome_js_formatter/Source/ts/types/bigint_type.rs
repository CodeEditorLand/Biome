use biome_formatter::write;
use biome_js_syntax::{TsBigintType, TsBigintTypeFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct FormatTsBigintType;

impl FormatNodeRule<TsBigintType> for FormatTsBigintType {
	fn fmt_fields(&self, node:&TsBigintType, f:&mut JsFormatter) -> FormatResult<()> {
		let TsBigintTypeFields { bigint_token } = node.as_fields();

		write![f, [bigint_token.format()]]
	}
}
