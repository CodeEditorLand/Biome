use biome_grit_syntax::GritPatternIncludes;
use grit_pattern_matcher::pattern::Includes;

use super::{PatternCompiler, compilation_context::NodeCompilationContext};
use crate::{CompileError, grit_context::GritQueryContext};

pub(crate) struct IncludesCompiler;

impl IncludesCompiler {
	pub(crate) fn from_node(
		node:&GritPatternIncludes,
		context:&mut NodeCompilationContext,
	) -> Result<Includes<GritQueryContext>, CompileError> {
		let pattern = PatternCompiler::from_maybe_curly_node(&node.includes()?, context)?;

		Ok(Includes::new(pattern))
	}
}
