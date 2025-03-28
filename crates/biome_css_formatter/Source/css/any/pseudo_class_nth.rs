//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_css_syntax::AnyCssPseudoClassNth;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssPseudoClassNth;
impl FormatRule<AnyCssPseudoClassNth> for FormatAnyCssPseudoClassNth {
	type Context = CssFormatContext;

	fn fmt(&self, node:&AnyCssPseudoClassNth, f:&mut CssFormatter) -> FormatResult<()> {
		match node {
			AnyCssPseudoClassNth::CssPseudoClassNth(node) => node.format().fmt(f),
			AnyCssPseudoClassNth::CssPseudoClassNthIdentifier(node) => node.format().fmt(f),
			AnyCssPseudoClassNth::CssPseudoClassNthNumber(node) => node.format().fmt(f),
		}
	}
}
