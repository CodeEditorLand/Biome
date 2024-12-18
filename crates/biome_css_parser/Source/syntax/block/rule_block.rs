use biome_css_syntax::{CssSyntaxKind, CssSyntaxKind::*, T};
use biome_parser::{CompletedMarker, parse_lists::ParseNodeList};

use crate::{
	parser::CssParser,
	syntax::{RuleList, block::ParseBlockBody, is_at_rule_list_element},
};

#[inline]
pub(crate) fn parse_rule_block(p:&mut CssParser) -> CompletedMarker { RuleBlock.parse_block_body(p) }
struct RuleBlock;

impl ParseBlockBody for RuleBlock {
	const BLOCK_KIND:CssSyntaxKind = CSS_RULE_BLOCK;

	fn is_at_element(&self, p:&mut CssParser) -> bool { is_at_rule_list_element(p) }

	fn parse_list(&mut self, p:&mut CssParser) { RuleList::new(T!['}']).parse_list(p); }
}
