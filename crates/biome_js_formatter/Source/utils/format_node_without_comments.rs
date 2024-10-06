use biome_js_syntax::{AnyJsExpression, AnyJsLiteralExpression};

use crate::{
	js::{
		auxiliary::metavariable::FormatJsMetavariable,
		bogus::bogus_expression::FormatJsBogusExpression,
		expressions::{
			array_expression::FormatJsArrayExpression,
			arrow_function_expression::FormatJsArrowFunctionExpression,
			assignment_expression::FormatJsAssignmentExpression,
			await_expression::FormatJsAwaitExpression,
			bigint_literal_expression::FormatJsBigintLiteralExpression,
			binary_expression::FormatJsBinaryExpression,
			boolean_literal_expression::FormatJsBooleanLiteralExpression,
			call_expression::FormatJsCallExpression,
			class_expression::FormatJsClassExpression,
			computed_member_expression::FormatJsComputedMemberExpression,
			conditional_expression::FormatJsConditionalExpression,
			function_expression::FormatJsFunctionExpression,
			identifier_expression::FormatJsIdentifierExpression,
			import_call_expression::FormatJsImportCallExpression,
			import_meta_expression::FormatJsImportMetaExpression,
			in_expression::FormatJsInExpression,
			instanceof_expression::FormatJsInstanceofExpression,
			logical_expression::FormatJsLogicalExpression,
			new_expression::FormatJsNewExpression,
			new_target_expression::FormatJsNewTargetExpression,
			null_literal_expression::FormatJsNullLiteralExpression,
			number_literal_expression::FormatJsNumberLiteralExpression,
			object_expression::FormatJsObjectExpression,
			parenthesized_expression::FormatJsParenthesizedExpression,
			post_update_expression::FormatJsPostUpdateExpression,
			pre_update_expression::FormatJsPreUpdateExpression,
			regex_literal_expression::FormatJsRegexLiteralExpression,
			sequence_expression::FormatJsSequenceExpression,
			static_member_expression::FormatJsStaticMemberExpression,
			string_literal_expression::FormatJsStringLiteralExpression,
			super_expression::FormatJsSuperExpression,
			template_expression::FormatJsTemplateExpression,
			this_expression::FormatJsThisExpression,
			unary_expression::FormatJsUnaryExpression,
			yield_expression::FormatJsYieldExpression,
		},
	},
	jsx::expressions::tag_expression::FormatJsxTagExpression,
	prelude::*,
	ts::expressions::{
		as_expression::FormatTsAsExpression,
		instantiation_expression::FormatTsInstantiationExpression,
		non_null_assertion_expression::FormatTsNonNullAssertionExpression,
		satisfies_expression::FormatTsSatisfiesExpression,
		type_assertion_expression::FormatTsTypeAssertionExpression,
	},
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsExpressionWithoutComments;

impl FormatRule<AnyJsExpression> for FormatAnyJsExpressionWithoutComments {
	type Context = JsFormatContext;

	fn fmt(&self, node:&AnyJsExpression, f:&mut JsFormatter) -> FormatResult<()> {
		match node {
			AnyJsExpression::AnyJsLiteralExpression(literal_expr) => {
				match literal_expr {
					AnyJsLiteralExpression::JsBigintLiteralExpression(node) => {
						FormatJsBigintLiteralExpression.fmt_node(node, f)
					},
					AnyJsLiteralExpression::JsBooleanLiteralExpression(node) => {
						FormatJsBooleanLiteralExpression.fmt_node(node, f)
					},
					AnyJsLiteralExpression::JsNullLiteralExpression(node) => {
						FormatJsNullLiteralExpression.fmt_node(node, f)
					},
					AnyJsLiteralExpression::JsNumberLiteralExpression(node) => {
						FormatJsNumberLiteralExpression.fmt_node(node, f)
					},
					AnyJsLiteralExpression::JsRegexLiteralExpression(node) => {
						FormatJsRegexLiteralExpression.fmt_node(node, f)
					},
					AnyJsLiteralExpression::JsStringLiteralExpression(node) => {
						FormatJsStringLiteralExpression.fmt_node(node, f)
					},
				}
			},
			AnyJsExpression::JsArrayExpression(node) => {
				FormatJsArrayExpression::default().fmt_node(node, f)
			},
			AnyJsExpression::JsArrowFunctionExpression(node) => {
				FormatJsArrowFunctionExpression::default().fmt_node(node, f)
			},
			AnyJsExpression::JsAssignmentExpression(node) => {
				FormatJsAssignmentExpression.fmt_node(node, f)
			},
			AnyJsExpression::JsAwaitExpression(node) => FormatJsAwaitExpression.fmt_node(node, f),
			AnyJsExpression::JsBinaryExpression(node) => FormatJsBinaryExpression.fmt_node(node, f),
			AnyJsExpression::JsBogusExpression(node) => FormatJsBogusExpression.fmt(node, f),
			AnyJsExpression::JsCallExpression(node) => FormatJsCallExpression.fmt_node(node, f),
			AnyJsExpression::JsClassExpression(node) => FormatJsClassExpression.fmt_node(node, f),
			AnyJsExpression::JsComputedMemberExpression(node) => {
				FormatJsComputedMemberExpression.fmt_node(node, f)
			},
			AnyJsExpression::JsConditionalExpression(node) => {
				FormatJsConditionalExpression::default().fmt_node(node, f)
			},
			AnyJsExpression::JsFunctionExpression(node) => {
				FormatJsFunctionExpression::default().fmt_node(node, f)
			},
			AnyJsExpression::JsMetavariable(node) => FormatJsMetavariable.fmt_node(node, f),
			AnyJsExpression::JsIdentifierExpression(node) => {
				FormatJsIdentifierExpression.fmt_node(node, f)
			},
			AnyJsExpression::JsImportCallExpression(node) => {
				FormatJsImportCallExpression.fmt_node(node, f)
			},
			AnyJsExpression::JsImportMetaExpression(node) => {
				FormatJsImportMetaExpression.fmt_node(node, f)
			},
			AnyJsExpression::JsInExpression(node) => FormatJsInExpression.fmt_node(node, f),
			AnyJsExpression::JsInstanceofExpression(node) => {
				FormatJsInstanceofExpression.fmt_node(node, f)
			},
			AnyJsExpression::JsLogicalExpression(node) => {
				FormatJsLogicalExpression.fmt_node(node, f)
			},
			AnyJsExpression::JsNewExpression(node) => FormatJsNewExpression.fmt_node(node, f),
			AnyJsExpression::JsNewTargetExpression(node) => {
				FormatJsNewTargetExpression.fmt_node(node, f)
			},
			AnyJsExpression::JsObjectExpression(node) => FormatJsObjectExpression.fmt_node(node, f),
			AnyJsExpression::JsParenthesizedExpression(node) => {
				FormatJsParenthesizedExpression.fmt_node(node, f)
			},
			AnyJsExpression::JsPostUpdateExpression(node) => {
				FormatJsPostUpdateExpression.fmt_node(node, f)
			},
			AnyJsExpression::JsPreUpdateExpression(node) => {
				FormatJsPreUpdateExpression.fmt_node(node, f)
			},
			AnyJsExpression::JsSequenceExpression(node) => {
				FormatJsSequenceExpression.fmt_node(node, f)
			},
			AnyJsExpression::JsStaticMemberExpression(node) => {
				FormatJsStaticMemberExpression.fmt_node(node, f)
			},
			AnyJsExpression::JsSuperExpression(node) => FormatJsSuperExpression.fmt_node(node, f),
			AnyJsExpression::JsTemplateExpression(node) => {
				FormatJsTemplateExpression.fmt_node(node, f)
			},
			AnyJsExpression::JsThisExpression(node) => FormatJsThisExpression.fmt_node(node, f),
			AnyJsExpression::JsUnaryExpression(node) => FormatJsUnaryExpression.fmt_node(node, f),
			AnyJsExpression::JsYieldExpression(node) => FormatJsYieldExpression.fmt_node(node, f),
			AnyJsExpression::JsxTagExpression(node) => FormatJsxTagExpression.fmt_node(node, f),
			AnyJsExpression::TsAsExpression(node) => FormatTsAsExpression.fmt_node(node, f),
			AnyJsExpression::TsInstantiationExpression(node) => {
				FormatTsInstantiationExpression.fmt_node(node, f)
			},
			AnyJsExpression::TsNonNullAssertionExpression(node) => {
				FormatTsNonNullAssertionExpression.fmt_node(node, f)
			},
			AnyJsExpression::TsSatisfiesExpression(node) => {
				FormatTsSatisfiesExpression.fmt_node(node, f)
			},
			AnyJsExpression::TsTypeAssertionExpression(node) => {
				FormatTsTypeAssertionExpression.fmt_node(node, f)
			},
		}
	}
}
