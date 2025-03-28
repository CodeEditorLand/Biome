use biome_formatter::write;
use biome_js_syntax::{TsExportAssignmentClause, TsExportAssignmentClauseFields};

use crate::{prelude::*, utils::FormatStatementSemicolon};

#[derive(Debug, Clone, Default)]
pub struct FormatTsExportAssignmentClause;

impl FormatNodeRule<TsExportAssignmentClause> for FormatTsExportAssignmentClause {
	fn fmt_fields(&self, node:&TsExportAssignmentClause, f:&mut JsFormatter) -> FormatResult<()> {
		let TsExportAssignmentClauseFields { eq_token, expression, semicolon_token } =
			node.as_fields();

		write!(
			f,
			[
				eq_token.format(),
				space(),
				expression.format(),
				FormatStatementSemicolon::new(semicolon_token.as_ref())
			]
		)
	}
}
