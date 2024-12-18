use biome_js_syntax::JsBogusStatement;

use crate::FormatBogusNodeRule;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusStatement;

impl FormatBogusNodeRule<JsBogusStatement> for FormatJsBogusStatement {}
