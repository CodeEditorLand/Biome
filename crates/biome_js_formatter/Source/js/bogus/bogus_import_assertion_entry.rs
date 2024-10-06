use biome_js_syntax::JsBogusImportAssertionEntry;

use crate::FormatBogusNodeRule;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusImportAssertionEntry;

impl FormatBogusNodeRule<JsBogusImportAssertionEntry> for FormatJsBogusImportAssertionEntry {}
