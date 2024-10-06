//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_css_syntax::AnyCssContainerOrCombinableQuery;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssContainerOrCombinableQuery;
impl FormatRule<AnyCssContainerOrCombinableQuery> for FormatAnyCssContainerOrCombinableQuery {
	type Context = CssFormatContext;

	fn fmt(&self, node:&AnyCssContainerOrCombinableQuery, f:&mut CssFormatter) -> FormatResult<()> {
		match node {
			AnyCssContainerOrCombinableQuery::AnyCssContainerQueryInParens(node) => {
				node.format().fmt(f)
			},
			AnyCssContainerOrCombinableQuery::CssContainerOrQuery(node) => node.format().fmt(f),
		}
	}
}
