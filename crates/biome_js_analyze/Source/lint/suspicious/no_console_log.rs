use biome_analyze::{
	context::RuleContext,
	declare_lint_rule,
	ActionCategory,
	FixKind,
	Rule,
	RuleDiagnostic,
};
use biome_console::markup;
use biome_js_syntax::{
	global_identifier,
	AnyJsMemberExpression,
	JsCallExpression,
	JsExpressionStatement,
};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::{services::semantic::Semantic, JsRuleAction};

declare_lint_rule! {
	/// Disallow the use of `console.log`
	///
	/// ## Examples
	///
	/// ### Invalid
	///
	/// ```js,expect_diagnostic
	/// console.log()
	/// ```
	///
	/// ### Valid
	///
	/// ```js
	/// console.info("info");
	/// console.warn("warn");
	/// console.error("error");
	/// console.assert(true);
	/// console.table(["foo", "bar"]);
	/// const console = { log() {} };
	/// console.log();
	/// ```
	///
	pub NoConsoleLog {
		version: "1.0.0",
		name: "noConsoleLog",
		language: "js",
		recommended: false,
		deprecated: "Use the rule noConsole instead.",
		fix_kind: FixKind::Unsafe,
	}
}

impl Rule for NoConsoleLog {
	type Options = ();
	type Query = Semantic<JsCallExpression>;
	type Signals = Option<Self::State>;
	type State = ();

	fn run(ctx:&RuleContext<Self>) -> Self::Signals {
		let call_expression = ctx.query();
		let model = ctx.model();
		let callee = call_expression.callee().ok()?;
		let member_expression = AnyJsMemberExpression::cast(callee.into_syntax())?;
		if member_expression.member_name()?.text() != "log" {
			return None;
		}
		let object = member_expression.object().ok()?;
		let (reference, name) = global_identifier(&object)?;
		if name.text() != "console" {
			return None;
		}
		model.binding(&reference).is_none().then_some(())
	}

	fn diagnostic(ctx:&RuleContext<Self>, _:&Self::State) -> Option<RuleDiagnostic> {
		let node = ctx.query();
		let node = JsExpressionStatement::cast(node.syntax().parent()?)?;
		Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Don't use "<Emphasis>"console.log"</Emphasis>
                },
            )
            .note(markup! {
                <Emphasis>"console.log"</Emphasis>" is usually a tool for debugging and you don't want to have that in production."
            })
            .note(markup! {
                "If it is not for debugging purpose then using "<Emphasis>"console.info"</Emphasis>" might be more appropriate."
            }),
        )
	}

	fn action(ctx:&RuleContext<Self>, _:&Self::State) -> Option<JsRuleAction> {
		let call_expression = ctx.query();
		let mut mutation = ctx.root().begin();

		match JsExpressionStatement::cast(call_expression.syntax().parent()?) {
			Some(stmt) if stmt.semicolon_token().is_some() => {
				mutation.remove_node(stmt);
			},
			_ => {
				mutation.remove_node(call_expression.clone());
			},
		}

		Some(JsRuleAction::new(
			ActionCategory::QuickFix,
			ctx.metadata().applicability(),
			markup! { "Remove console.log" }.to_owned(),
			mutation,
		))
	}
}
