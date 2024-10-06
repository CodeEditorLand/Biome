use biome_formatter::write;
use biome_js_syntax::TsMethodSignatureTypeMember;

use crate::{
	js::classes::method_class_member::FormatAnyJsMethodMember,
	prelude::*,
	utils::FormatTypeMemberSeparator,
};

#[derive(Debug, Clone, Default)]
pub struct FormatTsMethodSignatureTypeMember;

impl FormatNodeRule<TsMethodSignatureTypeMember> for FormatTsMethodSignatureTypeMember {
	fn fmt_fields(
		&self,
		node:&TsMethodSignatureTypeMember,
		f:&mut JsFormatter,
	) -> FormatResult<()> {
		write![
			f,
			[
				FormatAnyJsMethodMember::from(node.clone()),
				FormatTypeMemberSeparator::new(node.separator_token().as_ref())
			]
		]
	}
}
