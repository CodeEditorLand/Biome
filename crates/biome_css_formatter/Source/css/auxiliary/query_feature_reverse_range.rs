use biome_css_syntax::{CssQueryFeatureReverseRange, CssQueryFeatureReverseRangeFields};
use biome_formatter::write;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeatureReverseRange;
impl FormatNodeRule<CssQueryFeatureReverseRange> for FormatCssQueryFeatureReverseRange {
	fn fmt_fields(
		&self,
		node:&CssQueryFeatureReverseRange,
		f:&mut CssFormatter,
	) -> FormatResult<()> {
		let CssQueryFeatureReverseRangeFields { left, comparison, right } = node.as_fields();

		write!(f, [left.format(), space(), comparison.format(), space(), right.format()])
	}
}
