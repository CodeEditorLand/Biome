use rustc_hash::FxHashMap;

use crate::{FixKind, Rule, RuleKey};
use std::any::{Any, TypeId};
use std::fmt::Debug;
use std::path::PathBuf;

/// A convenient new type data structure to store the options that belong to a rule
#[derive(Debug)]
pub struct RuleOptions(TypeId, Box<dyn Any>, Option<FixKind>);

impl RuleOptions {
    /// Creates a new [RuleOptions]
    pub fn new<O: 'static>(options: O, fix_kind: Option<FixKind>) -> Self {
        Self(TypeId::of::<O>(), Box::new(options), fix_kind)
    }

    /// It returns the deserialized rule option
    pub fn value<O: 'static>(&self) -> &O {
        let RuleOptions(type_id, value, _) = &self;
        let current_id = TypeId::of::<O>();
        debug_assert_eq!(type_id, &current_id);
        // SAFETY: the code should fail when asserting the types.
        // If the code throws an error here, it means that the developer didn't test
        // the rule with the options
        value.downcast_ref::<O>().unwrap()
    }

    pub fn fix_kind(&self) -> Option<FixKind> {
        self.2
    }
}

/// A convenient new type data structure to insert and get rules
#[derive(Debug, Default)]
pub struct AnalyzerRules(FxHashMap<RuleKey, RuleOptions>);

impl AnalyzerRules {
    /// It tracks the options of a specific rule
    pub fn push_rule(&mut self, rule_key: RuleKey, options: RuleOptions) {
        self.0.insert(rule_key, options);
    }

    /// It retrieves the options of a stored rule, given its name
    pub fn get_rule_options<O: 'static>(&self, rule_key: &RuleKey) -> Option<&O> {
        self.0.get(rule_key).map(|o| o.value::<O>())
    }

    pub fn get_rule_fix_kind(&self, rule_key: &RuleKey) -> Option<FixKind> {
        self.0.get(rule_key).and_then(|options| options.fix_kind())
    }
}

/// A data structured derived from the `biome.json` file
#[derive(Debug, Default)]
pub struct AnalyzerConfiguration {
    /// A list of rules and their options
    pub rules: AnalyzerRules,

    /// A collections of bindings that the analyzers should consider as "external".
    ///
    /// For example, lint rules should ignore them.
    pub globals: Vec<String>,

    /// Allows to choose a different quote when applying fixes inside the lint rules
    pub preferred_quote: PreferredQuote,

    /// Indicates the type of runtime or transformation used for interpreting JSX.
    pub jsx_runtime: Option<JsxRuntime>,
}

/// A set of information useful to the analyzer infrastructure
#[derive(Debug, Default)]
pub struct AnalyzerOptions {
    /// A data structured derived from the [`biome.json`] file
    pub configuration: AnalyzerConfiguration,

    /// The file that is being analyzed
    pub file_path: PathBuf,
}

impl AnalyzerOptions {
    pub fn globals(&self) -> Vec<&str> {
        self.configuration
            .globals
            .iter()
            .map(|global| global.as_str())
            .collect()
    }

    pub fn jsx_runtime(&self) -> Option<JsxRuntime> {
        self.configuration.jsx_runtime
    }

    pub fn rule_options<R>(&self) -> Option<R::Options>
    where
        R: Rule<Options: Clone> + 'static,
    {
        self.configuration
            .rules
            .get_rule_options::<R::Options>(&RuleKey::rule::<R>())
            .cloned()
    }

    pub fn rule_fix_kind<R>(&self) -> Option<FixKind>
    where
        R: Rule<Options: Clone> + 'static,
    {
        self.configuration
            .rules
            .get_rule_fix_kind(&RuleKey::rule::<R>())
    }

    pub fn preferred_quote(&self) -> &PreferredQuote {
        &self.configuration.preferred_quote
    }
}

#[derive(Debug, Default)]
pub enum PreferredQuote {
    /// Double quotes
    #[default]
    Double,
    /// Single quotes
    Single,
}

impl PreferredQuote {
    pub const fn is_double(&self) -> bool {
        matches!(self, Self::Double)
    }

    pub const fn is_single(&self) -> bool {
        matches!(self, Self::Single)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum JsxRuntime {
    #[default]
    Transparent,
    ReactClassic,
}
