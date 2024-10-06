use biome_js_syntax::TsTemplateElementList;

use crate::{js::lists::template_element_list::AnyTemplateElementList, prelude::*};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTemplateElementList;

impl FormatRule<TsTemplateElementList> for FormatTsTemplateElementList {
	type Context = JsFormatContext;

	fn fmt(&self, node:&TsTemplateElementList, f:&mut JsFormatter) -> FormatResult<()> {
		AnyTemplateElementList::TsTemplateElementList(node.clone()).fmt(f)
	}
}
