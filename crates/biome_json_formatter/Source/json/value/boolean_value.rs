use biome_json_syntax::JsonBooleanValue;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonBooleanValue;

impl FormatNodeRule<JsonBooleanValue> for FormatJsonBooleanValue {
	fn fmt_fields(&self, node:&JsonBooleanValue, f:&mut JsonFormatter) -> FormatResult<()> {
		node.value_token()?.format().fmt(f)
	}
}
