use biome_formatter::write;
use biome_js_syntax::{
	JsNullLiteralExpression,
	JsNullLiteralExpressionFields,
	parentheses::NeedsParentheses,
};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsNullLiteralExpression;

impl FormatNodeRule<JsNullLiteralExpression> for FormatJsNullLiteralExpression {
	fn fmt_fields(&self, node:&JsNullLiteralExpression, f:&mut JsFormatter) -> FormatResult<()> {
		let JsNullLiteralExpressionFields { value_token } = node.as_fields();

		write![f, [value_token.format()]]
	}

	fn needs_parentheses(&self, item:&JsNullLiteralExpression) -> bool { item.needs_parentheses() }
}
