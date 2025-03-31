use biome_formatter::write;
use biome_grit_syntax::GritUnderscore;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritUnderscore;
impl FormatNodeRule<GritUnderscore> for FormatGritUnderscore {
	fn fmt_fields(&self, node: &GritUnderscore, f: &mut GritFormatter) -> FormatResult<()> {
		write!(f, [node.token_token().format()])
	}
}
