use biome_formatter::write;
use biome_js_syntax::{JsExportDefaultExpressionClause, JsExportDefaultExpressionClauseFields};

use crate::{prelude::*, utils::FormatStatementSemicolon};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsExportDefaultExpressionClause;

impl FormatNodeRule<JsExportDefaultExpressionClause> for FormatJsExportDefaultExpressionClause {
	fn fmt_fields(
		&self,
		node:&JsExportDefaultExpressionClause,
		f:&mut JsFormatter,
	) -> FormatResult<()> {
		let JsExportDefaultExpressionClauseFields { default_token, expression, semicolon_token } =
			node.as_fields();

		write!(
			f,
			[
				default_token.format(),
				space(),
				expression.format(),
				FormatStatementSemicolon::new(semicolon_token.as_ref())
			]
		)
	}
}
