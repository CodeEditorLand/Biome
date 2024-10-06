use biome_formatter::{
	prelude::{soft_line_break_or_space, space},
	write,
	FormatResult,
};
use biome_graphql_syntax::GraphqlLanguage;
use biome_rowan::{AstNode, AstSeparatedList};

use crate::{context::GraphqlFormatContext, prelude::*, GraphqlFormatter};

pub(crate) fn write_interface_like_list<N, I>(
	node:&N,
	f:&mut GraphqlFormatter,
) -> FormatResult<()>
where
	N: AstSeparatedList<Language = GraphqlLanguage, Node = I>,
	I: AstNode<Language = GraphqlLanguage> + AsFormat<GraphqlFormatContext>, {
	for (index, element) in node.elements().enumerate() {
		let node = element.node();

		if index != 0 {
			if node.map_or(false, |node| node.syntax().has_leading_comments()) {
				write!(f, [soft_line_break_or_space()])?;
			} else {
				write!(f, [space()])?;
			}
		}

		write!(f, [node.format()])?;

		let trailing_separator = element.trailing_separator()?;

		if let Some(token) = trailing_separator {
			write![f, [space(), token.format()]]?;
		}
	}

	Ok(())
}
