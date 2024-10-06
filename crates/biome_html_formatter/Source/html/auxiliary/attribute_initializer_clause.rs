use biome_formatter::write;
use biome_html_syntax::{HtmlAttributeInitializerClause, HtmlAttributeInitializerClauseFields};

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttributeInitializerClause;
impl FormatNodeRule<HtmlAttributeInitializerClause> for FormatHtmlAttributeInitializerClause {
	fn fmt_fields(
		&self,
		node:&HtmlAttributeInitializerClause,
		f:&mut HtmlFormatter,
	) -> FormatResult<()> {
		let HtmlAttributeInitializerClauseFields { eq_token, value } = node.as_fields();

		write![f, [eq_token.format(), value.format()]]
	}
}
