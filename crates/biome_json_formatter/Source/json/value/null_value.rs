use biome_json_syntax::JsonNullValue;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonNullValue;

impl FormatNodeRule<JsonNullValue> for FormatJsonNullValue {
	fn fmt_fields(&self, node:&JsonNullValue, f:&mut JsonFormatter) -> FormatResult<()> {
		node.value_token()?.format().fmt(f)
	}
}
