use biome_css_syntax::{CssSyntaxKind, CssSyntaxKind::*, T};
use biome_parser::{
	parse_lists::ParseNodeList,
	parse_recovery::{ParseRecovery, RecoveryResult},
	parsed_syntax::{ParsedSyntax, ParsedSyntax::Absent},
	CompletedMarker,
	Parser,
};

use crate::{
	parser::CssParser,
	syntax::{
		at_rule::{is_at_at_rule, parse_at_rule},
		block::ParseBlockBody,
		is_at_declaration,
		parse_declaration_with_semicolon,
		parse_error::expected_any_declaration_or_at_rule,
	},
};

#[inline]
pub(crate) fn parse_declaration_or_at_rule_list_block(p:&mut CssParser) -> CompletedMarker {
	DeclarationOrAtRuleListBlock.parse_block_body(p)
}

struct DeclarationOrAtRuleListBlock;

impl ParseBlockBody for DeclarationOrAtRuleListBlock {
	const BLOCK_KIND:CssSyntaxKind = CSS_DECLARATION_OR_AT_RULE_BLOCK;

	fn is_at_element(&self, p:&mut CssParser) -> bool { is_at_declaration_or_at_rule_item(p) }

	fn parse_list(&mut self, p:&mut CssParser) { DeclarationOrAtRuleList.parse_list(p); }
}

#[inline]
fn is_at_declaration_or_at_rule_item(p:&mut CssParser) -> bool {
	is_at_at_rule(p) || is_at_declaration(p)
}

struct DeclarationOrAtRuleListParseRecovery;
impl ParseRecovery for DeclarationOrAtRuleListParseRecovery {
	type Kind = CssSyntaxKind;
	type Parser<'source> = CssParser<'source>;

	const RECOVERED_KIND:Self::Kind = CSS_BOGUS;

	fn is_at_recovered(&self, p:&mut Self::Parser<'_>) -> bool {
		p.at(T!['}']) || is_at_declaration_or_at_rule_item(p)
	}
}

struct DeclarationOrAtRuleList;
impl ParseNodeList for DeclarationOrAtRuleList {
	type Kind = CssSyntaxKind;
	type Parser<'source> = CssParser<'source>;

	const LIST_KIND:Self::Kind = CSS_DECLARATION_OR_AT_RULE_LIST;

	fn parse_element(&mut self, p:&mut Self::Parser<'_>) -> ParsedSyntax {
		if is_at_at_rule(p) {
			parse_at_rule(p)
		} else if is_at_declaration(p) {
			parse_declaration_with_semicolon(p)
		} else {
			Absent
		}
	}

	fn is_at_list_end(&self, p:&mut Self::Parser<'_>) -> bool { p.at(T!['}']) }

	fn recover(&mut self, p:&mut Self::Parser<'_>, parsed_element:ParsedSyntax) -> RecoveryResult {
		parsed_element.or_recover(
			p,
			&DeclarationOrAtRuleListParseRecovery,
			expected_any_declaration_or_at_rule,
		)
	}
}
