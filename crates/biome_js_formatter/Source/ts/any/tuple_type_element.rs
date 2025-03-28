//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_js_syntax::AnyTsTupleTypeElement;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsTupleTypeElement;
impl FormatRule<AnyTsTupleTypeElement> for FormatAnyTsTupleTypeElement {
	type Context = JsFormatContext;

	fn fmt(&self, node:&AnyTsTupleTypeElement, f:&mut JsFormatter) -> FormatResult<()> {
		match node {
			AnyTsTupleTypeElement::AnyTsType(node) => node.format().fmt(f),
			AnyTsTupleTypeElement::TsNamedTupleTypeElement(node) => node.format().fmt(f),
			AnyTsTupleTypeElement::TsOptionalTupleTypeElement(node) => node.format().fmt(f),
			AnyTsTupleTypeElement::TsRestTupleTypeElement(node) => node.format().fmt(f),
		}
	}
}
