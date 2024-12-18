use biome_formatter::write;
use biome_grit_syntax::{GritAssignmentAsPattern, GritAssignmentAsPatternFields};

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritAssignmentAsPattern;
impl FormatNodeRule<GritAssignmentAsPattern> for FormatGritAssignmentAsPattern {
	fn fmt_fields(&self, node:&GritAssignmentAsPattern, f:&mut GritFormatter) -> FormatResult<()> {
		let GritAssignmentAsPatternFields { pattern, eq_token, container } = node.as_fields();

		write!(f, [container.format(), space(), eq_token.format(), space(), pattern.format()])
	}
}
