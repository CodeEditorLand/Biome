//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_grit_syntax::AnyGritMapElement;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritMapElement;
impl FormatRule<AnyGritMapElement> for FormatAnyGritMapElement {
	type Context = GritFormatContext;

	fn fmt(&self, node:&AnyGritMapElement, f:&mut GritFormatter) -> FormatResult<()> {
		match node {
			AnyGritMapElement::GritBogusMapElement(node) => node.format().fmt(f),
			AnyGritMapElement::GritMapElement(node) => node.format().fmt(f),
		}
	}
}
