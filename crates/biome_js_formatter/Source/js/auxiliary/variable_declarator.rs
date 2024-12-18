use biome_formatter::write;
use biome_js_syntax::JsVariableDeclarator;

use crate::{prelude::*, utils::AnyJsAssignmentLike};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsVariableDeclarator;

impl FormatNodeRule<JsVariableDeclarator> for FormatJsVariableDeclarator {
	fn fmt_fields(&self, node:&JsVariableDeclarator, f:&mut JsFormatter) -> FormatResult<()> {
		write![f, [AnyJsAssignmentLike::from(node.clone())]]
	}
}
