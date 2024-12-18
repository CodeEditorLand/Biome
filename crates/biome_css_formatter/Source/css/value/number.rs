use biome_css_syntax::CssNumber;
use biome_formatter::token::number::format_number_token;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNumber;
impl FormatNodeRule<CssNumber> for FormatCssNumber {
	fn fmt_fields(&self, node:&CssNumber, f:&mut CssFormatter) -> FormatResult<()> {
		format_number_token(&node.value_token()?).fmt(f)
	}
}
