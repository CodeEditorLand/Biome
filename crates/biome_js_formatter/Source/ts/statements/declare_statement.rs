use biome_formatter::write;
use biome_js_syntax::{TsDeclareStatement, TsDeclareStatementFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct FormatTsDeclareStatement;

impl FormatNodeRule<TsDeclareStatement> for FormatTsDeclareStatement {
	fn fmt_fields(&self, node:&TsDeclareStatement, f:&mut JsFormatter) -> FormatResult<()> {
		let TsDeclareStatementFields { declaration, declare_token } = node.as_fields();
		write![f, [declare_token.format(), space(), declaration.format()]]
	}
}
