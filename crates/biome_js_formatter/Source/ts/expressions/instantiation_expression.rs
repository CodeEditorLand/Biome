use biome_formatter::write;
use biome_js_syntax::{
	parentheses::NeedsParentheses,
	TsInstantiationExpression,
	TsInstantiationExpressionFields,
};

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub struct FormatTsInstantiationExpression;
impl FormatNodeRule<TsInstantiationExpression> for FormatTsInstantiationExpression {
	fn fmt_fields(&self, node:&TsInstantiationExpression, f:&mut JsFormatter) -> FormatResult<()> {
		let TsInstantiationExpressionFields { expression, arguments } = node.as_fields();

		write![f, [expression.format(), arguments.format()]]
	}

	fn needs_parentheses(&self, item:&TsInstantiationExpression) -> bool {
		item.needs_parentheses()
	}
}
