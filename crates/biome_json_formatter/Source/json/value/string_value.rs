use biome_json_syntax::JsonStringValue;

use crate::{format_string::format_string_token, prelude::*};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonStringValue;

impl FormatNodeRule<JsonStringValue> for FormatJsonStringValue {
	fn fmt_fields(&self, node:&JsonStringValue, f:&mut JsonFormatter) -> FormatResult<()> {
		format_string_token(&node.value_token()?).fmt(f)
	}
}
