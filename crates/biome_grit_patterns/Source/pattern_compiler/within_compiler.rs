use biome_grit_syntax::GritWithin;
use grit_pattern_matcher::pattern::Within;

use super::{PatternCompiler, compilation_context::NodeCompilationContext};
use crate::{CompileError, grit_context::GritQueryContext};

pub(crate) struct WithinCompiler;

impl WithinCompiler {
	pub(crate) fn from_node(
		node:&GritWithin,
		context:&mut NodeCompilationContext,
	) -> Result<Within<GritQueryContext>, CompileError> {
		let pattern = PatternCompiler::from_maybe_curly_node(&node.pattern()?, context)?;

		Ok(Within::new(pattern))
	}
}
