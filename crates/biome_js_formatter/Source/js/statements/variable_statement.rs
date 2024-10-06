use biome_formatter::write;
use biome_js_syntax::{JsVariableStatement, JsVariableStatementFields};

use crate::{prelude::*, utils::FormatStatementSemicolon};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsVariableStatement;

impl FormatNodeRule<JsVariableStatement> for FormatJsVariableStatement {
	fn fmt_fields(&self, node:&JsVariableStatement, f:&mut JsFormatter) -> FormatResult<()> {
		let JsVariableStatementFields { declaration, semicolon_token } = node.as_fields();

		write!(
			f,
			[declaration.format(), FormatStatementSemicolon::new(semicolon_token.as_ref())]
		)
	}
}
