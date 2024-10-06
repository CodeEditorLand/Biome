use biome_css_syntax::{CssValueAtRuleImportClause, CssValueAtRuleImportClauseFields};
use biome_formatter::write;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRuleImportClause;
impl FormatNodeRule<CssValueAtRuleImportClause> for FormatCssValueAtRuleImportClause {
	fn fmt_fields(
		&self,
		node:&CssValueAtRuleImportClause,
		f:&mut CssFormatter,
	) -> FormatResult<()> {
		let CssValueAtRuleImportClauseFields { specifiers, from_token, source } = node.as_fields();

		write!(f, [specifiers.format(), space(), from_token.format(), space(), source.format()])
	}
}
