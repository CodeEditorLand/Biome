use biome_css_syntax::CssSyntaxKind::*;
use biome_parser::{
	parsed_syntax::{
		ParsedSyntax,
		ParsedSyntax::{Absent, Present},
	},
	Parser,
};

use crate::{
	parser::CssParser,
	syntax::{
		is_at_identifier,
		parse_error::expected_identifier,
		selector::parse_selector_identifier,
	},
};

#[inline]
pub(crate) fn parse_pseudo_class_identifier(p:&mut CssParser) -> ParsedSyntax {
	if !is_at_identifier(p) {
		return Absent;
	}

	let m = p.start();
	parse_selector_identifier(p).or_add_diagnostic(p, expected_identifier);
	Present(m.complete(p, CSS_PSEUDO_CLASS_IDENTIFIER))
}
