//! This module provides important and useful traits to help to format tokens
//! and nodes when implementing a syntax formatter.

pub use biome_formatter::{prelude::*, separated::TrailingSeparator};
pub use biome_rowan::{AstNode as _, AstNodeList as _, AstSeparatedList as _};

pub(crate) use crate::{
	comments::JsComments,
	separated::FormatAstSeparatedListExtension,
	AsFormat as _,
	FormatNodeRule,
	FormattedIterExt,
	JsFormatContext,
	JsFormatter,
};
