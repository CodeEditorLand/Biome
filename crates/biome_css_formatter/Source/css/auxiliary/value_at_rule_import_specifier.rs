use biome_css_syntax::{CssValueAtRuleImportSpecifier, CssValueAtRuleImportSpecifierFields};
use biome_formatter::write;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRuleImportSpecifier;
impl FormatNodeRule<CssValueAtRuleImportSpecifier> for FormatCssValueAtRuleImportSpecifier {
	fn fmt_fields(
		&self,
		node:&CssValueAtRuleImportSpecifier,
		f:&mut CssFormatter,
	) -> FormatResult<()> {
		let CssValueAtRuleImportSpecifierFields { name } = node.as_fields();

		write!(f, [name.format()])
	}
}
