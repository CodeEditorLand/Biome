use biome_js_syntax::JsBogusMember;

use crate::FormatBogusNodeRule;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusMember;

impl FormatBogusNodeRule<JsBogusMember> for FormatJsBogusMember {}
