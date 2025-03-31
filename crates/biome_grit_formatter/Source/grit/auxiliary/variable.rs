use biome_formatter::write;
use biome_grit_syntax::GritVariable;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritVariable;
impl FormatNodeRule<GritVariable> for FormatGritVariable {
	fn fmt_fields(&self, node: &GritVariable, f: &mut GritFormatter) -> FormatResult<()> {
		write!(f, [node.value_token().format()])
	}
}
