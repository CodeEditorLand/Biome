use biome_formatter::write;
use biome_js_syntax::{
	parentheses::NeedsParentheses,
	JsPostUpdateExpression,
	JsPostUpdateExpressionFields,
};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsPostUpdateExpression;

impl FormatNodeRule<JsPostUpdateExpression> for FormatJsPostUpdateExpression {
	fn fmt_fields(&self, node:&JsPostUpdateExpression, f:&mut JsFormatter) -> FormatResult<()> {
		let JsPostUpdateExpressionFields { operand, operator_token } = node.as_fields();

		write![f, [operand.format(), operator_token.format()]]
	}

	fn needs_parentheses(&self, item:&JsPostUpdateExpression) -> bool { item.needs_parentheses() }
}

#[cfg(test)]
mod tests {

	use biome_js_syntax::JsPostUpdateExpression;

	use crate::{assert_needs_parentheses, assert_not_needs_parentheses};

	#[test]
	fn needs_parentheses() {
		assert_needs_parentheses!("class A extends (A++) {}", JsPostUpdateExpression);

		assert_needs_parentheses!("(a++).b", JsPostUpdateExpression);
		assert_needs_parentheses!("(a++)[b]", JsPostUpdateExpression);
		assert_not_needs_parentheses!("a[b++]", JsPostUpdateExpression);

		assert_needs_parentheses!("(a++)`template`", JsPostUpdateExpression);

		assert_needs_parentheses!("(a++)()", JsPostUpdateExpression);
		assert_needs_parentheses!("new (a++)()", JsPostUpdateExpression);

		assert_needs_parentheses!("(a++)!", JsPostUpdateExpression);

		assert_needs_parentheses!("(a++) ** 3", JsPostUpdateExpression);
		assert_not_needs_parentheses!("(a++) + 3", JsPostUpdateExpression);
	}
}
