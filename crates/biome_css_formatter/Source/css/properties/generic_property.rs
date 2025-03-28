use biome_css_syntax::{CssGenericProperty, CssGenericPropertyFields};
use biome_formatter::write;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssGenericProperty;
impl FormatNodeRule<CssGenericProperty> for FormatCssGenericProperty {
	fn fmt_fields(&self, node:&CssGenericProperty, f:&mut CssFormatter) -> FormatResult<()> {
		let CssGenericPropertyFields { name, colon_token, value } = node.as_fields();

		write!(f, [name.format(), colon_token.format(), space(), value.format()])
	}
}
