use biome_formatter::write;
use biome_js_syntax::{TsConstructorSignatureClassMember, TsConstructorSignatureClassMemberFields};

use crate::{prelude::*, utils::FormatOptionalSemicolon};

#[derive(Debug, Clone, Default)]
pub struct FormatTsConstructorSignatureClassMember;

impl FormatNodeRule<TsConstructorSignatureClassMember> for FormatTsConstructorSignatureClassMember {
	fn fmt_fields(
		&self,
		node:&TsConstructorSignatureClassMember,
		f:&mut JsFormatter,
	) -> FormatResult<()> {
		let TsConstructorSignatureClassMemberFields {
			modifiers,
			name,
			parameters,
			semicolon_token,
		} = node.as_fields();

		write!(
			f,
			[
				modifiers.format(),
				space(),
				name.format(),
				group(&parameters.format()),
				FormatOptionalSemicolon::new(semicolon_token.as_ref())
			]
		)
	}
}
