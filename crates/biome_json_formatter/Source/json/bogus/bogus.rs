use biome_json_syntax::JsonBogus;

use crate::FormatBogusNodeRule;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonBogus;

impl FormatBogusNodeRule<JsonBogus> for FormatJsonBogus {}
