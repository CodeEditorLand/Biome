use biome_js_syntax::{
	parentheses::NeedsParentheses,
	AnyJsComputedMember,
	JsComputedMemberAssignment,
};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsComputedMemberAssignment;

impl FormatNodeRule<JsComputedMemberAssignment> for FormatJsComputedMemberAssignment {
	fn fmt_fields(&self, node:&JsComputedMemberAssignment, f:&mut JsFormatter) -> FormatResult<()> {
		AnyJsComputedMember::from(node.clone()).fmt(f)
	}

	fn needs_parentheses(&self, item:&JsComputedMemberAssignment) -> bool {
		item.needs_parentheses()
	}
}
