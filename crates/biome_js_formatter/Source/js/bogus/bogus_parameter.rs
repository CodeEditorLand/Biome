use biome_js_syntax::JsBogusParameter;

use crate::FormatBogusNodeRule;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusParameter;

impl FormatBogusNodeRule<JsBogusParameter> for FormatJsBogusParameter {}
