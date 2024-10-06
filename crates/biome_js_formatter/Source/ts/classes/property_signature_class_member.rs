use biome_formatter::write;
use biome_js_syntax::TsPropertySignatureClassMember;

use crate::{
	js::classes::property_class_member::{AnyJsPropertyClassMember, FormatClassPropertySemicolon},
	prelude::*,
	utils::AnyJsAssignmentLike,
};

#[derive(Debug, Clone, Default)]
pub struct FormatTsPropertySignatureClassMember;

impl FormatNodeRule<TsPropertySignatureClassMember> for FormatTsPropertySignatureClassMember {
	fn fmt_fields(
		&self,
		node:&TsPropertySignatureClassMember,
		f:&mut JsFormatter,
	) -> FormatResult<()> {
		let semicolon_token = node.semicolon_token();

		write!(
			f,
			[
				AnyJsAssignmentLike::from(node.clone()),
				FormatClassPropertySemicolon::new(
					&AnyJsPropertyClassMember::from(node.clone()),
					semicolon_token.as_ref()
				)
			]
		)
	}
}
