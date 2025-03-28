//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_grit_syntax::AnyGritMapKey;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritMapKey;
impl FormatRule<AnyGritMapKey> for FormatAnyGritMapKey {
	type Context = GritFormatContext;

	fn fmt(&self, node:&AnyGritMapKey, f:&mut GritFormatter) -> FormatResult<()> {
		match node {
			AnyGritMapKey::GritName(node) => node.format().fmt(f),
			AnyGritMapKey::GritVariable(node) => node.format().fmt(f),
		}
	}
}
