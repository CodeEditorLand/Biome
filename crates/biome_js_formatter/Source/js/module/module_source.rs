use biome_formatter::write;
use biome_js_syntax::{JsModuleSource, JsModuleSourceFields};

use crate::{
	prelude::*,
	utils::{FormatLiteralStringToken, StringLiteralParentKind},
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsModuleSource;

impl FormatNodeRule<JsModuleSource> for FormatJsModuleSource {
	fn fmt_fields(&self, node:&JsModuleSource, f:&mut JsFormatter) -> FormatResult<()> {
		let JsModuleSourceFields { value_token } = node.as_fields();

		write!(
			f,
			[FormatLiteralStringToken::new(
				&value_token?,
				StringLiteralParentKind::Expression
			)]
		)
	}
}
