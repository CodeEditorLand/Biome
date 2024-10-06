use biome_css_syntax::CssGenericComponentValueList;

use crate::{prelude::*, utils::component_value_list::write_component_value_list};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssGenericComponentValueList;
impl FormatRule<CssGenericComponentValueList> for FormatCssGenericComponentValueList {
	type Context = CssFormatContext;

	fn fmt(&self, node:&CssGenericComponentValueList, f:&mut CssFormatter) -> FormatResult<()> {
		write_component_value_list(node, f)
	}
}
