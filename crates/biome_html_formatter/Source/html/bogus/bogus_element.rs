use biome_html_syntax::HtmlBogusElement;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlBogusElement;
impl FormatBogusNodeRule<HtmlBogusElement> for FormatHtmlBogusElement {}
