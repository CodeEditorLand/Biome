use biome_js_syntax::JsBogus;

use crate::FormatBogusNodeRule;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogus;

impl FormatBogusNodeRule<JsBogus> for FormatJsBogus {}
