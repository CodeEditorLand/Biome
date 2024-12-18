use biome_formatter::write;
use biome_js_syntax::{JsDirective, JsDirectiveFields};

use crate::{
	prelude::*,
	utils::{FormatLiteralStringToken, FormatStatementSemicolon, StringLiteralParentKind},
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsDirective;

impl FormatNodeRule<JsDirective> for FormatJsDirective {
	fn fmt_fields(&self, node:&JsDirective, f:&mut JsFormatter) -> FormatResult<()> {
		let JsDirectiveFields { value_token, semicolon_token } = node.as_fields();

		write!(
			f,
			[
				FormatLiteralStringToken::new(&value_token?, StringLiteralParentKind::Directive),
				FormatStatementSemicolon::new(semicolon_token.as_ref())
			]
		)
	}
}
