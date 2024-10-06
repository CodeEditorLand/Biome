use biome_grit_syntax::GritBogusPredicate;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusPredicate;
impl FormatBogusNodeRule<GritBogusPredicate> for FormatGritBogusPredicate {}
