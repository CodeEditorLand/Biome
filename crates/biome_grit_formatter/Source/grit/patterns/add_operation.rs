use biome_formatter::write;
use biome_grit_syntax::{GritAddOperation, GritAddOperationFields};

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritAddOperation;
impl FormatNodeRule<GritAddOperation> for FormatGritAddOperation {
	fn fmt_fields(&self, node: &GritAddOperation, f: &mut GritFormatter) -> FormatResult<()> {
		let GritAddOperationFields { left, right, plus_token } = node.as_fields();

		write!(f, [left.format(), space(), plus_token.format(), space(), right.format()])
	}
}
