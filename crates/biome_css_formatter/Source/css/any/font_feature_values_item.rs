//! This is a generated file. Don't modify it by hand! Run 'cargo codegen
//! formatter' to re-generate the file.

use biome_css_syntax::AnyCssFontFeatureValuesItem;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssFontFeatureValuesItem;
impl FormatRule<AnyCssFontFeatureValuesItem> for FormatAnyCssFontFeatureValuesItem {
	type Context = CssFormatContext;

	fn fmt(&self, node:&AnyCssFontFeatureValuesItem, f:&mut CssFormatter) -> FormatResult<()> {
		match node {
			AnyCssFontFeatureValuesItem::CssBogusFontFeatureValuesItem(node) => {
				node.format().fmt(f)
			},
			AnyCssFontFeatureValuesItem::CssFontFeatureValuesItem(node) => node.format().fmt(f),
		}
	}
}
