use biome_formatter::write;
use biome_js_syntax::{JsDebuggerStatement, JsDebuggerStatementFields};

use crate::{prelude::*, utils::FormatStatementSemicolon};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsDebuggerStatement;

impl FormatNodeRule<JsDebuggerStatement> for FormatJsDebuggerStatement {
	fn fmt_fields(&self, node:&JsDebuggerStatement, f:&mut JsFormatter) -> FormatResult<()> {
		let JsDebuggerStatementFields { debugger_token, semicolon_token } = node.as_fields();

		write!(f, [debugger_token.format(),])?;

		if f.comments().has_dangling_comments(node.syntax()) {
			write!(f, [space(), format_dangling_comments(node.syntax())])?;
		}

		FormatStatementSemicolon::new(semicolon_token.as_ref()).fmt(f)
	}

	fn fmt_dangling_comments(
		&self,
		_:&JsDebuggerStatement,
		_:&mut JsFormatter,
	) -> FormatResult<()> {
		// Handled in `fmt_fields`
		Ok(())
	}
}
