use biome_formatter::write;
use biome_js_syntax::{TsStringLiteralType, TsStringLiteralTypeFields};

use crate::{
	prelude::*,
	utils::{FormatLiteralStringToken, StringLiteralParentKind},
};

#[derive(Debug, Clone, Default)]
pub struct FormatTsStringLiteralType;

impl FormatNodeRule<TsStringLiteralType> for FormatTsStringLiteralType {
	fn fmt_fields(&self, node:&TsStringLiteralType, f:&mut JsFormatter) -> FormatResult<()> {
		let TsStringLiteralTypeFields { literal_token } = node.as_fields();

		write!(
			f,
			[FormatLiteralStringToken::new(
				&literal_token?,
				StringLiteralParentKind::Expression
			)]
		)
	}
}
