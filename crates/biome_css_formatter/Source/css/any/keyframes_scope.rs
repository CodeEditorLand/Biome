//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_css_syntax::AnyCssKeyframesScope;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssKeyframesScope;
impl FormatRule<AnyCssKeyframesScope> for FormatAnyCssKeyframesScope {
	type Context = CssFormatContext;

	fn fmt(&self, node:&AnyCssKeyframesScope, f:&mut CssFormatter) -> FormatResult<()> {
		match node {
			AnyCssKeyframesScope::CssKeyframesScopeFunction(node) => node.format().fmt(f),
			AnyCssKeyframesScope::CssKeyframesScopePrefix(node) => node.format().fmt(f),
		}
	}
}
