use biome_js_syntax::JsArrayAssignmentPatternElementList;

use crate::{prelude::*, utils::array::write_array_node};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsArrayAssignmentPatternElementList;

impl FormatRule<JsArrayAssignmentPatternElementList> for FormatJsArrayAssignmentPatternElementList {
	type Context = JsFormatContext;

	fn fmt(
		&self,
		node:&JsArrayAssignmentPatternElementList,
		formatter:&mut JsFormatter,
	) -> FormatResult<()> {
		write_array_node(node, formatter)
	}
}
