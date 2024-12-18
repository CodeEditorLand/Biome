use biome_formatter::write;
use biome_grit_syntax::{GritPatternMaybe, GritPatternMaybeFields};

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternMaybe;
impl FormatNodeRule<GritPatternMaybe> for FormatGritPatternMaybe {
	fn fmt_fields(&self, node:&GritPatternMaybe, f:&mut GritFormatter) -> FormatResult<()> {
		let GritPatternMaybeFields { pattern, maybe_token } = node.as_fields();

		write!(f, [maybe_token.format(), space(), pattern.format()])
	}
}
