use biome_analyze::{GroupCategory, RegistryVisitor, RuleCategory, RuleGroup};
use biome_json_syntax::JsonLanguage;

use crate::analyzers::{
	indent_size::IndentSize,
	nursery_rules::NurseryRules,
	schema::Schema,
	trailing_comma::TrailingComma,
};

mod indent_size;
mod nursery_rules;
mod schema;
mod trailing_comma;

pub(crate) struct MigrationGroup;
pub(crate) struct MigrationCategory;

impl RuleGroup for MigrationGroup {
	type Category = MigrationCategory;
	type Language = JsonLanguage;

	const NAME:&'static str = "migrations";

	fn record_rules<V:RegistryVisitor<Self::Language> + ?Sized>(registry:&mut V) {
		// Order here is important, rules should be added from the most old, to
		// the most recent v1.3.0
		registry.record_rule::<IndentSize>();
		// v1.5.0
		registry.record_rule::<Schema>();
		// v1.8.0
		registry.record_rule::<TrailingComma>();
		// v1.8.0
		registry.record_rule::<NurseryRules>();
	}
}

impl GroupCategory for MigrationCategory {
	type Language = JsonLanguage;

	const CATEGORY:RuleCategory = RuleCategory::Action;

	fn record_groups<V:RegistryVisitor<Self::Language> + ?Sized>(registry:&mut V) {
		registry.record_group::<MigrationGroup>();
	}
}
