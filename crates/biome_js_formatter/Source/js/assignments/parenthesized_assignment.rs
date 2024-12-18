use biome_formatter::write;
use biome_js_syntax::{JsParenthesizedAssignment, JsParenthesizedAssignmentFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsParenthesizedAssignment;

impl FormatNodeRule<JsParenthesizedAssignment> for FormatJsParenthesizedAssignment {
	fn fmt_fields(&self, node:&JsParenthesizedAssignment, f:&mut JsFormatter) -> FormatResult<()> {
		let JsParenthesizedAssignmentFields { l_paren_token, assignment, r_paren_token } =
			node.as_fields();

		write![f, [l_paren_token.format(), assignment.format(), r_paren_token.format(),]]
	}
}
