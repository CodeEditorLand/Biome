use biome_formatter::write;
use biome_js_syntax::{parentheses::NeedsParentheses, JsThisExpression, JsThisExpressionFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsThisExpression;

impl FormatNodeRule<JsThisExpression> for FormatJsThisExpression {
	fn fmt_fields(&self, node:&JsThisExpression, f:&mut JsFormatter) -> FormatResult<()> {
		let JsThisExpressionFields { this_token } = node.as_fields();

		write![f, [this_token.format()]]
	}

	fn needs_parentheses(&self, item:&JsThisExpression) -> bool { item.needs_parentheses() }
}
