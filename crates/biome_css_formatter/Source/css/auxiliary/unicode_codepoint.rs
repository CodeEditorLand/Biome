use biome_css_syntax::{CssUnicodeCodepoint, CssUnicodeCodepointFields};
use biome_formatter::write;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnicodeCodepoint;
impl FormatNodeRule<CssUnicodeCodepoint> for FormatCssUnicodeCodepoint {
	fn fmt_fields(&self, node:&CssUnicodeCodepoint, f:&mut CssFormatter) -> FormatResult<()> {
		let CssUnicodeCodepointFields { value_token } = node.as_fields();

		write!(f, [value_token.format(),])
	}
}
