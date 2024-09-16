use crate::prelude::*;
use biome_css_syntax::CssLayerReferenceList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssLayerReferenceList;
impl FormatRule<CssLayerReferenceList> for FormatCssLayerReferenceList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssLayerReferenceList, f: &mut CssFormatter) -> FormatResult<()> {
        // Using `join_with` instead of `join_nodes_with_soft_line` to avoid
        // preserving empty lines from the input source. See the comment in
        // [FormatCssSelectorList] for more information.
        let separator = soft_line_break_or_space();
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated(",") {
            joiner.entry(&formatted);
        }

        joiner.finish()
    }
}
