use biome_css_syntax::{CssUniversalSelector, CssUniversalSelectorFields};
use biome_formatter::write;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUniversalSelector;
impl FormatNodeRule<CssUniversalSelector> for FormatCssUniversalSelector {
	fn fmt_fields(&self, node:&CssUniversalSelector, f:&mut CssFormatter) -> FormatResult<()> {
		let CssUniversalSelectorFields { namespace, star_token } = node.as_fields();

		write!(f, [namespace.format(), star_token.format()])
	}
}
