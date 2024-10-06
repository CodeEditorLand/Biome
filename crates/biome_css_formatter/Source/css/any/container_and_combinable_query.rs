//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_css_syntax::AnyCssContainerAndCombinableQuery;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssContainerAndCombinableQuery;
impl FormatRule<AnyCssContainerAndCombinableQuery> for FormatAnyCssContainerAndCombinableQuery {
	type Context = CssFormatContext;

	fn fmt(
		&self,
		node:&AnyCssContainerAndCombinableQuery,
		f:&mut CssFormatter,
	) -> FormatResult<()> {
		match node {
			AnyCssContainerAndCombinableQuery::AnyCssContainerQueryInParens(node) => {
				node.format().fmt(f)
			},
			AnyCssContainerAndCombinableQuery::CssContainerAndQuery(node) => node.format().fmt(f),
		}
	}
}
