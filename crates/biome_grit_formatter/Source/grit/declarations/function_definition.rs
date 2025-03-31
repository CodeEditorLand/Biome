use biome_formatter::write;
use biome_grit_syntax::{GritFunctionDefinition, GritFunctionDefinitionFields};

use crate::prelude::*;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritFunctionDefinition;
impl FormatNodeRule<GritFunctionDefinition> for FormatGritFunctionDefinition {
	fn fmt_fields(&self, node: &GritFunctionDefinition, f: &mut GritFormatter) -> FormatResult<()> {
		let GritFunctionDefinitionFields { function_token, name, l_paren_token, args, r_paren_token, body } =
			node.as_fields();

		write!(
			f,
			[
				function_token.format(),
				space(),
				name.format(),
				l_paren_token.format(),
				group(&args.format()),
				r_paren_token.format(),
				space(),
				body.format()
			]
		)
	}
}
