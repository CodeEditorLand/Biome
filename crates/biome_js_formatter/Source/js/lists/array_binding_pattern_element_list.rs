use biome_js_syntax::JsArrayBindingPatternElementList;

use crate::{prelude::*, utils::array::write_array_node};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsArrayBindingPatternElementList;

impl FormatRule<JsArrayBindingPatternElementList> for FormatJsArrayBindingPatternElementList {
	type Context = JsFormatContext;

	fn fmt(
		&self,
		node:&JsArrayBindingPatternElementList,
		formatter:&mut JsFormatter,
	) -> FormatResult<()> {
		write_array_node(node, formatter)
	}
}
