use biome_js_syntax::JsBogusExpression;

use crate::FormatBogusNodeRule;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusExpression;

impl FormatBogusNodeRule<JsBogusExpression> for FormatJsBogusExpression {}
