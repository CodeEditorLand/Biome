use biome_css_syntax::{CssSyntaxKind, CssSyntaxKind::*, T};
use biome_parser::{
	parse_recovery::ParseRecoveryTokenSet,
	parsed_syntax::{ParsedSyntax, ParsedSyntax::Present},
	prelude::ParsedSyntax::Absent,
	token_set,
	Parser,
	TokenSet,
};

use crate::{
	parser::CssParser,
	syntax::{
		block::parse_declaration_block,
		parse_dashed_identifier,
		parse_error::expected_dashed_identifier,
	},
};

#[inline]
pub(crate) fn is_at_property_at_rule(p:&mut CssParser) -> bool { p.at(T![property]) }

#[inline]
pub(crate) fn parse_property_at_rule(p:&mut CssParser) -> ParsedSyntax {
	if !is_at_property_at_rule(p) {
		return Absent;
	}

	let m = p.start();

	p.bump(T![property]);

	let kind = if parse_dashed_identifier(p)
		.or_recover_with_token_set(
			p,
			&ParseRecoveryTokenSet::new(CSS_BOGUS, PROPERTY_RECOVERY_SET)
				.enable_recovery_on_line_break(),
			expected_dashed_identifier,
		)
		.is_ok()
	{
		CSS_PROPERTY_AT_RULE
	} else {
		CSS_BOGUS_AT_RULE
	};

	parse_declaration_block(p);

	Present(m.complete(p, kind))
}

const PROPERTY_RECOVERY_SET:TokenSet<CssSyntaxKind> = token_set!(T!['{']);
