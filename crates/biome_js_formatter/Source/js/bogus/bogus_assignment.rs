use biome_js_syntax::JsBogusAssignment;

use crate::FormatBogusNodeRule;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusAssignment;

impl FormatBogusNodeRule<JsBogusAssignment> for FormatJsBogusAssignment {}
