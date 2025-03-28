use biome_js_syntax::{
	JsLiteralExportName,
	JsLiteralExportNameFields,
	JsSyntaxKind::JS_STRING_LITERAL,
};

use crate::{
	prelude::*,
	utils::{FormatLiteralStringToken, StringLiteralParentKind},
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsLiteralExportName;

impl FormatNodeRule<JsLiteralExportName> for FormatJsLiteralExportName {
	fn fmt_fields(&self, node:&JsLiteralExportName, f:&mut JsFormatter) -> FormatResult<()> {
		let JsLiteralExportNameFields { value } = node.as_fields();

		let value = value?;

		if value.kind() == JS_STRING_LITERAL {
			FormatLiteralStringToken::new(&value, StringLiteralParentKind::Expression).fmt(f)
		} else {
			value.format().fmt(f)
		}
	}
}
