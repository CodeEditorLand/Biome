use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_semantic::{Reference, ReferencesExtensions};
use biome_js_syntax::AnyJsClass;

use crate::services::semantic::Semantic;

declare_lint_rule! {
	/// Disallow reassigning class members.
	///
	/// A class declaration creates a variable that we can modify, however, the modification is a mistake in most cases.
	///
	/// ## Examples
	///
	/// ### Invalid
	///
	/// ```js,expect_diagnostic
	/// class A {}
	/// A = 0;
	/// ```
	///
	/// ```js,expect_diagnostic
	/// A = 0;
	/// class A {}
	/// ```
	///
	/// ```js,expect_diagnostic
	/// class A {
	/// 	b() {
	/// 		A = 0;
	/// 	}
	/// }
	/// ```
	///
	/// ```js,expect_diagnostic
	/// let A = class A {
	/// 	b() {
	/// 		A = 0;
	/// 		// `let A` is shadowed by the class name.
	/// 	}
	/// }
	/// ```
	///
	/// ### Valid
	///
	/// ```js
	/// let A = class A {}
	/// A = 0; // A is a variable.
	/// ```
	///
	/// ```js
	/// let A = class {
	///     b() {
	///         A = 0; // A is a variable.
	///     }
	/// }
	/// ```
	///
	/// ```js
	/// class A {
	/// 	b(A) {
	/// 		A = 0; // A is a parameter.
	/// 	}
	/// }
	/// ```
	///
	pub NoClassAssign {
		version: "1.0.0",
		name: "noClassAssign",
		language: "js",
		sources: &[RuleSource::Eslint("no-class-assign")],
		recommended: true,
	}
}

impl Rule for NoClassAssign {
	type Options = ();
	type Query = Semantic<AnyJsClass>;
	type Signals = Vec<Self::State>;
	type State = Reference;

	fn run(ctx:&RuleContext<Self>) -> Self::Signals {
		let node = ctx.query();
		let model = ctx.model();

		if let Some(id) = node.id() {
			if let Some(id_binding) = id.as_js_identifier_binding() {
				return id_binding.all_writes(model).collect();
			}
		}

		Vec::new()
	}

	fn diagnostic(ctx:&RuleContext<Self>, reference:&Self::State) -> Option<RuleDiagnostic> {
		let binding = ctx.query().id()?.as_js_identifier_binding()?.name_token().ok()?;
		let class_name = binding.text_trimmed();

		Some(
			RuleDiagnostic::new(
				rule_category!(),
				reference.syntax().text_trimmed_range(),
				markup! {"'"{class_name}"' is a class."},
			)
			.detail(binding.text_trimmed_range(), markup! {"'"{class_name}"' is defined here."}),
		)
	}
}
