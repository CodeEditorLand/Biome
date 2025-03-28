use biome_formatter::write;
use biome_js_syntax::{JsxAttribute, JsxAttributeFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxAttribute;

impl FormatNodeRule<JsxAttribute> for FormatJsxAttribute {
	fn fmt_fields(&self, node:&JsxAttribute, f:&mut JsFormatter) -> FormatResult<()> {
		let JsxAttributeFields { name, initializer } = node.as_fields();

		write![f, [name.format(), initializer.format()]]
	}
}
