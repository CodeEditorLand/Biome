use biome_formatter::write;
use biome_grit_syntax::GritNot;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritNot;
impl FormatNodeRule<GritNot> for FormatGritNot {
	fn fmt_fields(&self, node:&GritNot, f:&mut GritFormatter) -> FormatResult<()> {
		write!(f, [node.token().format()])
	}
}
