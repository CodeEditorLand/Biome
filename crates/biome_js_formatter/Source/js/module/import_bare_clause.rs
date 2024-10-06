use biome_formatter::write;
use biome_js_syntax::{JsImportBareClause, JsImportBareClauseFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportBareClause;

impl FormatNodeRule<JsImportBareClause> for FormatJsImportBareClause {
	fn fmt_fields(&self, node:&JsImportBareClause, f:&mut JsFormatter) -> FormatResult<()> {
		let JsImportBareClauseFields { source, assertion } = node.as_fields();

		write!(f, [source.format()])?;

		if let Some(assertion) = assertion {
			write!(f, [assertion.format()])?;
		}

		Ok(())
	}
}
