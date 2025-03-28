use biome_formatter::write;
use biome_js_syntax::{JsYieldArgument, JsYieldArgumentFields};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsYieldArgument;

impl FormatNodeRule<JsYieldArgument> for FormatJsYieldArgument {
	fn fmt_fields(&self, node:&JsYieldArgument, f:&mut JsFormatter) -> FormatResult<()> {
		let JsYieldArgumentFields { star_token, expression } = node.as_fields();

		write![f, [star_token.format(), space(), expression.format()]]
	}
}
