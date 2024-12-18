use biome_formatter::write;
use biome_js_syntax::{TsNonPrimitiveType, TsNonPrimitiveTypeFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct FormatTsNonPrimitiveType;

impl FormatNodeRule<TsNonPrimitiveType> for FormatTsNonPrimitiveType {
	fn fmt_fields(&self, node:&TsNonPrimitiveType, f:&mut JsFormatter) -> FormatResult<()> {
		let TsNonPrimitiveTypeFields { object_token } = node.as_fields();

		write![f, [object_token.format()]]
	}
}
