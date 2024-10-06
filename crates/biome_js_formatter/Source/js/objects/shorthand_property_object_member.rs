use biome_formatter::write;
use biome_js_syntax::{JsShorthandPropertyObjectMember, JsShorthandPropertyObjectMemberFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsShorthandPropertyObjectMember;

impl FormatNodeRule<JsShorthandPropertyObjectMember> for FormatJsShorthandPropertyObjectMember {
	fn fmt_fields(
		&self,
		node:&JsShorthandPropertyObjectMember,
		f:&mut JsFormatter,
	) -> FormatResult<()> {
		let JsShorthandPropertyObjectMemberFields { name } = node.as_fields();

		write![f, [name.format()]]
	}
}
