use biome_formatter::separated::TrailingSeparator;
use biome_grit_syntax::GritPredicateList;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateList;
impl FormatRule<GritPredicateList> for FormatGritPredicateList {
	type Context = GritFormatContext;

	fn fmt(&self, node: &GritPredicateList, f: &mut GritFormatter) -> FormatResult<()> {
		f.join_with(&soft_line_break_or_space())
			.entries(node.format_separated(",").with_trailing_separator(TrailingSeparator::Omit))
			.finish()
	}
}
