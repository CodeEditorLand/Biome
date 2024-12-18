use biome_css_syntax::{CssSyntaxKind, CssSyntaxKind::*};
use biome_parser::{CompletedMarker, parse_lists::ParseNodeList};

use crate::{
	parser::CssParser,
	syntax::{DeclarationList, block::ParseBlockBody, is_at_declaration},
};

#[inline]
pub(crate) fn parse_declaration_block(p:&mut CssParser) -> CompletedMarker { DeclarationBlock.parse_block_body(p) }
struct DeclarationBlock;

impl ParseBlockBody for DeclarationBlock {
	const BLOCK_KIND:CssSyntaxKind = CSS_DECLARATION_BLOCK;

	fn is_at_element(&self, p:&mut CssParser) -> bool { is_at_declaration(p) }

	fn parse_list(&mut self, p:&mut CssParser) { DeclarationList.parse_list(p); }
}
