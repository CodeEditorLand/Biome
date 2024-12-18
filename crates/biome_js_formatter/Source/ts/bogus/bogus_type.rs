use biome_js_syntax::TsBogusType;

use crate::FormatBogusNodeRule;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsBogusType;

impl FormatBogusNodeRule<TsBogusType> for FormatTsBogusType {}
