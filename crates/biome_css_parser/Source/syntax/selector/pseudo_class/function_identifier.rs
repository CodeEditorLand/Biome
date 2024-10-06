use biome_css_syntax::{
	CssSyntaxKind,
	CssSyntaxKind::{CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER, *},
	TextRange,
	T,
};
use biome_parser::{
	diagnostic::{expected_any, ParseDiagnostic},
	parsed_syntax::{
		ParsedSyntax,
		ParsedSyntax::{Absent, Present},
	},
	token_set,
	Parser,
	TokenSet,
};

use crate::{
	parser::CssParser,
	syntax::{
		parse_regular_identifier,
		selector::{
			eat_or_recover_selector_function_close_token,
			recover_selector_function_parameter,
		},
	},
};

const PSEUDO_CLASS_FUNCTION_IDENTIFIER_SET:TokenSet<CssSyntaxKind> = token_set![T![dir]];
#[inline]
pub(crate) fn is_at_pseudo_class_function_identifier(p:&mut CssParser) -> bool {
	p.at_ts(PSEUDO_CLASS_FUNCTION_IDENTIFIER_SET) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn parse_pseudo_class_function_identifier(p:&mut CssParser) -> ParsedSyntax {
	if !is_at_pseudo_class_function_identifier(p) {
		return Absent;
	}

	let m = p.start();

	p.bump_ts(PSEUDO_CLASS_FUNCTION_IDENTIFIER_SET);
	p.bump(T!['(']);

	let kind = if is_at_dir_parameter_identifier(p) {
		// SAFETY: we know that the next token is a dir parameter identifier
		let identifier = parse_regular_identifier(p).unwrap();

		if eat_or_recover_selector_function_close_token(
			p,
			identifier,
			expected_dir_parameter_identifier,
		) {
			CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER
		} else {
			CSS_BOGUS_PSEUDO_CLASS
		}
	} else {
		recover_selector_function_parameter(p, expected_dir_parameter_identifier);
		p.expect(T![')']);
		CSS_BOGUS_PSEUDO_CLASS
	};

	Present(m.complete(p, kind))
}

const DIR_PARAMETER_IDENTIFIER_SET:TokenSet<CssSyntaxKind> = token_set![T![ltr], T![rtl]];
#[inline]
fn is_at_dir_parameter_identifier(p:&mut CssParser) -> bool {
	p.at_ts(DIR_PARAMETER_IDENTIFIER_SET)
}

#[inline]
fn expected_dir_parameter_identifier(p:&CssParser, range:TextRange) -> ParseDiagnostic {
	expected_any(&["ltr", "rtl"], range, p)
}
