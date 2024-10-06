use biome_grit_syntax::GritPatternAfter;
use grit_pattern_matcher::pattern::After;

use super::{compilation_context::NodeCompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};

pub(crate) struct AfterCompiler;

impl AfterCompiler {
	pub(crate) fn from_node(
		node:&GritPatternAfter,
		context:&mut NodeCompilationContext,
	) -> Result<After<GritQueryContext>, CompileError> {
		let pattern = PatternCompiler::from_node(&node.pattern()?, context)?;
		Ok(After::new(pattern))
	}
}
