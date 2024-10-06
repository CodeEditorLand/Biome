use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};
use biome_js_syntax::{map_syntax_node, JsSyntaxNode};

use crate::{prelude::*, AsFormat, IntoFormat, JsFormatContext};

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatJsSyntaxNode;

impl biome_formatter::FormatRule<JsSyntaxNode> for FormatJsSyntaxNode {
	type Context = JsFormatContext;

	fn fmt(&self, node:&JsSyntaxNode, f:&mut JsFormatter) -> FormatResult<()> {
		map_syntax_node!(node.clone(), node => node.format().fmt(f))
	}
}

impl AsFormat<JsFormatContext> for JsSyntaxNode {
	type Format<'a> = FormatRefWithRule<'a, JsSyntaxNode, FormatJsSyntaxNode>;

	fn format(&self) -> Self::Format<'_> { FormatRefWithRule::new(self, FormatJsSyntaxNode) }
}

impl IntoFormat<JsFormatContext> for JsSyntaxNode {
	type Format = FormatOwnedWithRule<JsSyntaxNode, FormatJsSyntaxNode>;

	fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self, FormatJsSyntaxNode) }
}
