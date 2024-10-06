use biome_grit_syntax::GritBogusNamedArg;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusNamedArg;
impl FormatBogusNodeRule<GritBogusNamedArg> for FormatGritBogusNamedArg {}
