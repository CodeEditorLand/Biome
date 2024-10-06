use biome_js_syntax::TsTemplateChunkElement;

use crate::{js::auxiliary::template_chunk_element::AnyTemplateChunkElement, prelude::*};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTemplateChunkElement;

impl FormatNodeRule<TsTemplateChunkElement> for FormatTsTemplateChunkElement {
	fn fmt_fields(
		&self,
		node:&TsTemplateChunkElement,
		formatter:&mut JsFormatter,
	) -> FormatResult<()> {
		AnyTemplateChunkElement::from(node.clone()).fmt(formatter)
	}
}
