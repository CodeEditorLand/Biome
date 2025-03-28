use biome_formatter::write;
use biome_grit_syntax::GritAnnotation;

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritAnnotation;
impl FormatNodeRule<GritAnnotation> for FormatGritAnnotation {
	fn fmt_fields(&self, node:&GritAnnotation, f:&mut GritFormatter) -> FormatResult<()> {
		write!(f, [node.value_token().format()])
	}
}
