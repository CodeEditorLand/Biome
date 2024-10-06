use biome_formatter::write;
use biome_js_syntax::TsMethodSignatureClassMember;

use crate::{
	js::classes::method_class_member::FormatAnyJsMethodMember,
	prelude::*,
	utils::FormatOptionalSemicolon,
};

#[derive(Debug, Clone, Default)]
pub struct FormatTsMethodSignatureClassMember;

impl FormatNodeRule<TsMethodSignatureClassMember> for FormatTsMethodSignatureClassMember {
	fn fmt_fields(
		&self,
		node:&TsMethodSignatureClassMember,
		f:&mut JsFormatter,
	) -> FormatResult<()> {
		let modifiers = node.modifiers();

		if !modifiers.is_empty() {
			write!(f, [modifiers.format(), space()])?;
		}

		write!(
			f,
			[
				FormatAnyJsMethodMember::from(node.clone()),
				FormatOptionalSemicolon::new(node.semicolon_token().as_ref())
			]
		)
	}
}
