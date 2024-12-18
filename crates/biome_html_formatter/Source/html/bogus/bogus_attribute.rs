use biome_html_syntax::HtmlBogusAttribute;

use crate::FormatBogusNodeRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlBogusAttribute;
impl FormatBogusNodeRule<HtmlBogusAttribute> for FormatHtmlBogusAttribute {}
