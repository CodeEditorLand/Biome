use biome_formatter::{format_args, write};
use biome_js_syntax::{parentheses::NeedsParentheses, JsClassExpression};

use crate::{prelude::*, utils::format_class::FormatClass};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsClassExpression;

impl FormatNodeRule<JsClassExpression> for FormatJsClassExpression {
	fn fmt_fields(&self, node:&JsClassExpression, f:&mut JsFormatter) -> FormatResult<()> {
		if node.decorators().is_empty() || !self.needs_parentheses(node) {
			FormatClass::from(&node.clone().into()).fmt(f)
		} else {
			write!(
				f,
				[
					indent(&format_args![
						soft_line_break_or_space(),
						&FormatClass::from(&node.clone().into()),
					]),
					soft_line_break_or_space()
				]
			)
		}
	}

	fn needs_parentheses(&self, item:&JsClassExpression) -> bool {
		//! item.decorators().is_empty() ||
		item.needs_parentheses()
	}

	fn fmt_dangling_comments(&self, _:&JsClassExpression, _:&mut JsFormatter) -> FormatResult<()> {
		// Formatted as part of `FormatClass`
		Ok(())
	}
}

#[cfg(test)]
mod tests {

	use biome_js_syntax::JsClassExpression;

	use crate::{assert_needs_parentheses, assert_not_needs_parentheses};

	#[test]
	fn needs_parentheses() {
		assert_needs_parentheses!("console.log((class {})())", JsClassExpression);
		assert_needs_parentheses!("console.log(new (class {})())", JsClassExpression);

		assert_needs_parentheses!("(class {}).test", JsClassExpression);
		assert_not_needs_parentheses!("a => class {} ", JsClassExpression);

		assert_needs_parentheses!("export default (class  {})", JsClassExpression);
	}
}
