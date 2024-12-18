use biome_json_syntax::JsonBogusValue;

use crate::FormatBogusNodeRule;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonBogusValue;

impl FormatBogusNodeRule<JsonBogusValue> for FormatJsonBogusValue {}
