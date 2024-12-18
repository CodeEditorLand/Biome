use biome_formatter::write;
use biome_js_syntax::{TsImportType, TsImportTypeFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct FormatTsImportType;

impl FormatNodeRule<TsImportType> for FormatTsImportType {
	fn fmt_fields(&self, node:&TsImportType, f:&mut JsFormatter) -> FormatResult<()> {
		let TsImportTypeFields {
			typeof_token,
			import_token,
			arguments,
			qualifier_clause,
			type_arguments,
		} = node.as_fields();

		if let Some(typeof_token) = typeof_token {
			write!(f, [typeof_token.format(), space()])?;
		}

		write![
			f,
			[
				import_token.format(),
				arguments.format(),
				qualifier_clause.format(),
				type_arguments.format(),
			]
		]
	}
}
