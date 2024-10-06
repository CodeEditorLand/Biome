use biome_css_syntax::{CssUnicodeRangeWildcard, CssUnicodeRangeWildcardFields};
use biome_formatter::write;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnicodeRangeWildcard;
impl FormatNodeRule<CssUnicodeRangeWildcard> for FormatCssUnicodeRangeWildcard {
	fn fmt_fields(&self, node:&CssUnicodeRangeWildcard, f:&mut CssFormatter) -> FormatResult<()> {
		let CssUnicodeRangeWildcardFields { value_token } = node.as_fields();

		write!(f, [value_token.format(),])
	}
}
