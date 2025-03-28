use biome_formatter::separated::TrailingSeparator;
use biome_grit_syntax::GritLanguageFlavorList;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritLanguageFlavorList;
impl FormatRule<GritLanguageFlavorList> for FormatGritLanguageFlavorList {
	type Context = GritFormatContext;

	fn fmt(&self, node:&GritLanguageFlavorList, f:&mut GritFormatter) -> FormatResult<()> {
		f.join_with(&space())
			.entries(
				node.format_separated(",")
					.with_trailing_separator(TrailingSeparator::Disallowed),
			)
			.finish()
	}
}
