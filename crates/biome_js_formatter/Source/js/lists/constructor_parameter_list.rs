use biome_js_syntax::{JsConstructorParameterList, parameter_ext::AnyJsParameterList};

use crate::{
	js::{bindings::parameters::ParameterLayout, lists::parameter_list::FormatJsAnyParameterList},
	prelude::*,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsConstructorParameterList;

impl FormatRule<JsConstructorParameterList> for FormatJsConstructorParameterList {
	type Context = JsFormatContext;

	fn fmt(&self, node:&JsConstructorParameterList, f:&mut JsFormatter) -> FormatResult<()> {
		FormatJsAnyParameterList::with_layout(
			&AnyJsParameterList::from(node.clone()),
			ParameterLayout::Default,
		)
		.fmt(f)
	}
}
