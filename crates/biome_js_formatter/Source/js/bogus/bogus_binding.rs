use biome_js_syntax::JsBogusBinding;

use crate::FormatBogusNodeRule;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusBinding;

impl FormatBogusNodeRule<JsBogusBinding> for FormatJsBogusBinding {}
