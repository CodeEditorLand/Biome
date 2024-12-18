use biome_formatter::write;
use biome_grit_syntax::{GritSome, GritSomeFields};

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritSome;
impl FormatNodeRule<GritSome> for FormatGritSome {
	fn fmt_fields(&self, node:&GritSome, f:&mut GritFormatter) -> FormatResult<()> {
		let GritSomeFields { pattern, some_token } = node.as_fields();

		write!(f, [some_token.format(), space(), pattern.format()])
	}
}
