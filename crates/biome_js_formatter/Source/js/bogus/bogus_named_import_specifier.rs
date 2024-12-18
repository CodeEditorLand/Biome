use biome_js_syntax::JsBogusNamedImportSpecifier;

use crate::FormatBogusNodeRule;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusNamedImportSpecifier;

impl FormatBogusNodeRule<JsBogusNamedImportSpecifier> for FormatJsBogusNamedImportSpecifier {}
