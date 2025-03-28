use biome_css_syntax::{CssNamedNamespacePrefix, CssNamedNamespacePrefixFields};
use biome_formatter::write;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNamedNamespacePrefix;
impl FormatNodeRule<CssNamedNamespacePrefix> for FormatCssNamedNamespacePrefix {
	fn fmt_fields(&self, node:&CssNamedNamespacePrefix, f:&mut CssFormatter) -> FormatResult<()> {
		let CssNamedNamespacePrefixFields { name } = node.as_fields();

		write!(f, [name.format()])
	}
}
