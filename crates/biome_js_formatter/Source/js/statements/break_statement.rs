use biome_formatter::write;
use biome_js_syntax::{JsBreakStatement, JsBreakStatementFields};

use crate::{prelude::*, utils::FormatStatementSemicolon};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBreakStatement;

impl FormatNodeRule<JsBreakStatement> for FormatJsBreakStatement {
	fn fmt_fields(&self, node:&JsBreakStatement, f:&mut JsFormatter) -> FormatResult<()> {
		let JsBreakStatementFields { break_token, label, semicolon_token } = node.as_fields();

		write!(f, [break_token.format()])?;

		if let Some(label) = &label {
			write!(f, [space(), label.format()])?;
		}

		write!(f, [FormatStatementSemicolon::new(semicolon_token.as_ref())])
	}
}
