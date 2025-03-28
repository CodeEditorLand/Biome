use biome_js_syntax::JsClassDeclaration;

use crate::{prelude::*, utils::format_class::FormatClass};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsClassDeclaration;

impl FormatNodeRule<JsClassDeclaration> for FormatJsClassDeclaration {
	fn fmt_fields(&self, node:&JsClassDeclaration, f:&mut JsFormatter) -> FormatResult<()> {
		FormatClass::from(&node.clone().into()).fmt(f)
	}

	fn fmt_dangling_comments(&self, _:&JsClassDeclaration, _:&mut JsFormatter) -> FormatResult<()> {
		// Formatted as part of `FormatClass`
		Ok(())
	}
}
