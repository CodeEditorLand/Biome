use biome_formatter::write;
use biome_js_syntax::{JsPrivateClassMemberName, JsPrivateClassMemberNameFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsPrivateClassMemberName;

impl FormatNodeRule<JsPrivateClassMemberName> for FormatJsPrivateClassMemberName {
	fn fmt_fields(&self, node:&JsPrivateClassMemberName, f:&mut JsFormatter) -> FormatResult<()> {
		let JsPrivateClassMemberNameFields { hash_token, id_token } = node.as_fields();

		write![f, [hash_token.format(), id_token.format()]]
	}
}
