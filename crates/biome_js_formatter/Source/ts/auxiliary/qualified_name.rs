use biome_formatter::write;
use biome_js_syntax::{TsQualifiedName, TsQualifiedNameFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct FormatTsQualifiedName;

impl FormatNodeRule<TsQualifiedName> for FormatTsQualifiedName {
	fn fmt_fields(&self, node:&TsQualifiedName, f:&mut JsFormatter) -> FormatResult<()> {
		let TsQualifiedNameFields { left, dot_token, right } = node.as_fields();

		write![f, [left.format(), dot_token.format(), right.format(),]]
	}
}
