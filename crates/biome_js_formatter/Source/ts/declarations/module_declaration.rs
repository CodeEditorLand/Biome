use biome_formatter::write;
use biome_js_syntax::{TsModuleDeclaration, TsModuleDeclarationFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct FormatTsModuleDeclaration;

impl FormatNodeRule<TsModuleDeclaration> for FormatTsModuleDeclaration {
	fn fmt_fields(&self, node:&TsModuleDeclaration, f:&mut JsFormatter) -> FormatResult<()> {
		let TsModuleDeclarationFields { module_or_namespace, name, body } = node.as_fields();

		write![
			f,
			[module_or_namespace.format(), space(), name.format(), space(), body.format(),]
		]
	}
}
