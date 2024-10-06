use biome_css_syntax::{CssSyntaxKind, CssSyntaxKind::*, T};
use biome_parser::{
	parse_recovery::ParseRecoveryTokenSet,
	parsed_syntax::ParsedSyntax::Present,
	prelude::{ParsedSyntax::Absent, *},
};

use crate::{
	lexer::CssLexContext,
	parser::CssParser,
	syntax::{
		block::parse_declaration_block,
		parse_custom_identifier,
		parse_error::expected_non_css_wide_keyword_identifier,
	},
};

#[inline]
pub(crate) fn is_at_color_profile_at_rule(p:&mut CssParser) -> bool { p.at(T![color_profile]) }

#[inline]
pub(crate) fn parse_color_profile_at_rule(p:&mut CssParser) -> ParsedSyntax {
	if !is_at_color_profile_at_rule(p) {
		return Absent;
	}

	let m = p.start();

	p.bump(T![color_profile]);

	// TODO: This should actually be `<dashed-ident> | device-cmyk`.
	let kind = if parse_custom_identifier(p, CssLexContext::Regular)
		.or_recover_with_token_set(
			p,
			&ParseRecoveryTokenSet::new(CSS_BOGUS, COLOR_PROFILE_RECOVERY_SET)
				.enable_recovery_on_line_break(),
			expected_non_css_wide_keyword_identifier,
		)
		.is_ok()
	{
		CSS_COLOR_PROFILE_AT_RULE
	} else {
		CSS_BOGUS_AT_RULE
	};

	parse_declaration_block(p);

	Present(m.complete(p, kind))
}

const COLOR_PROFILE_RECOVERY_SET:TokenSet<CssSyntaxKind> = token_set![T!['{']];
