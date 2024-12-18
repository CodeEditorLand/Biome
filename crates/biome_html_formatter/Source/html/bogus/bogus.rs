use biome_html_syntax::HtmlBogus;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlBogus;
impl FormatBogusNodeRule<HtmlBogus> for FormatHtmlBogus {}
