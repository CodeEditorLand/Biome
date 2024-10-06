use std::borrow::Cow;

use enumflags2::{bitflags, BitFlags};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub enum RuleCategory {
	/// This rule checks the syntax according to the language specification
	/// and emits error diagnostics accordingly
	Syntax,
	/// This rule performs static analysis of the source code to detect
	/// invalid or error-prone patterns, and emits diagnostics along with
	/// proposed fixes
	Lint,
	/// This rule detects refactoring opportunities and emits code action
	/// signals
	Action,
	/// This rule detects transformations that should be applied to the code
	Transformation,
}

/// Actions that suppress rules should start with this string
pub const SUPPRESSION_ACTION_CATEGORY:&str = "quickfix.suppressRule";

/// The category of a code action, this type maps directly to the
/// [CodeActionKind] type in the Language Server Protocol specification
///
/// [CodeActionKind]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#codeActionKind
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub enum ActionCategory {
	/// Base kind for quickfix actions: 'quickfix'.
	///
	/// This action provides a fix to the diagnostic emitted by the same signal
	QuickFix,
	/// Base kind for refactoring actions: 'refactor'.
	///
	/// This action provides an optional refactor opportunity
	Refactor(RefactorKind),
	/// Base kind for source actions: `source`.
	///
	/// Source code actions apply to the entire file.
	Source(SourceActionKind),
	/// This action is using a base kind not covered by any of the previous
	/// variants
	Other(Cow<'static, str>),
}

impl ActionCategory {
	/// Returns true if this category matches the provided filter
	///
	/// ## Examples
	///
	/// ```
	/// use biome_analyze::{ActionCategory, RefactorKind};
	///
	/// assert!(ActionCategory::QuickFix.matches("quickfix"));
	/// assert!(!ActionCategory::QuickFix.matches("refactor"));
	///
	/// assert!(ActionCategory::Refactor(RefactorKind::None).matches("refactor"));
	/// assert!(!ActionCategory::Refactor(RefactorKind::None).matches("refactor.extract"));
	///
	/// assert!(ActionCategory::Refactor(RefactorKind::Extract).matches("refactor"));
	/// assert!(ActionCategory::Refactor(RefactorKind::Extract).matches("refactor.extract"));
	/// ```
	pub fn matches(&self, filter:&str) -> bool { self.to_str().starts_with(filter) }

	/// Returns the representation of this [ActionCategory] as a
	/// `CodeActionKind` string
	pub fn to_str(&self) -> Cow<'static, str> {
		match self {
			ActionCategory::QuickFix => Cow::Borrowed("quickfix.biome"),

			ActionCategory::Refactor(RefactorKind::None) => Cow::Borrowed("refactor.biome"),
			ActionCategory::Refactor(RefactorKind::Extract) => {
				Cow::Borrowed("refactor.extract.biome")
			},
			ActionCategory::Refactor(RefactorKind::Inline) => {
				Cow::Borrowed("refactor.inline.biome")
			},
			ActionCategory::Refactor(RefactorKind::Rewrite) => {
				Cow::Borrowed("refactor.rewrite.biome")
			},
			ActionCategory::Refactor(RefactorKind::Other(tag)) => {
				Cow::Owned(format!("refactor.{tag}.biome"))
			},

			ActionCategory::Source(SourceActionKind::None) => Cow::Borrowed("source.biome"),
			ActionCategory::Source(SourceActionKind::FixAll) => {
				Cow::Borrowed("source.fixAll.biome")
			},
			ActionCategory::Source(SourceActionKind::OrganizeImports) => {
				Cow::Borrowed("source.organizeImports.biome")
			},
			ActionCategory::Source(SourceActionKind::Other(tag)) => {
				Cow::Owned(format!("source.{tag}.biome"))
			},

			ActionCategory::Other(tag) => Cow::Owned(format!("{tag}.biome")),
		}
	}
}

/// The sub-category of a refactor code action.
///
/// [Check the LSP spec](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#codeActionKind) for more information:
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub enum RefactorKind {
	/// This action describes a refactor with no particular sub-category
	None,
	/// Base kind for refactoring extraction actions: 'refactor.extract'.
	///
	/// Example extract actions:
	/// - Extract method
	/// - Extract function
	/// - Extract variable
	/// - Extract interface from class
	Extract,
	/// Base kind for refactoring inline actions: 'refactor.inline'.
	///
	/// Example inline actions:
	/// - Inline function
	/// - Inline variable
	/// - Inline constant
	/// - ...
	Inline,
	/// Base kind for refactoring rewrite actions: 'refactor.rewrite'.
	///
	/// Example rewrite actions:
	/// - Convert JavaScript function to class
	/// - Add or remove parameter
	/// - Encapsulate field
	/// - Make method static
	/// - Move method to base class
	/// - ...
	Rewrite,
	/// This action is using a refactor kind not covered by any of the previous
	/// variants
	Other(Cow<'static, str>),
}

/// The sub-category of a source code action
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub enum SourceActionKind {
	/// This action describes a source action with no particular sub-category
	None,
	// Base kind for a 'fix all' source action: `source.fixAll`.
	//
	// 'Fix all' actions automatically fix errors that have a clear fix that
	// do not require user input. They should not suppress errors or perform
	// unsafe fixes such as generating new types or classes.
	FixAll,
	/// Base kind for an organize imports source action:
	/// `source.organizeImports`.
	OrganizeImports,
	/// This action is using a source action kind not covered by any of the
	/// previous variants
	Other(Cow<'static, str>),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[bitflags]
#[repr(u8)]
pub(crate) enum Categories {
	Syntax = 1 << RuleCategory::Syntax as u8,
	Lint = 1 << RuleCategory::Lint as u8,
	Action = 1 << RuleCategory::Action as u8,
	Transformation = 1 << RuleCategory::Transformation as u8,
}

#[derive(Debug, Copy, Clone)]
/// The categories supported by the analyzer.
///
/// The default implementation of this type returns an instance with all the
/// categories.
///
/// Use [RuleCategoriesBuilder] to generate the categories you want to query.
pub struct RuleCategories(BitFlags<Categories>);

impl RuleCategories {
	pub fn empty() -> Self {
		let empty:BitFlags<Categories> = BitFlags::empty();
		Self(empty)
	}

	pub fn all() -> Self {
		let empty:BitFlags<Categories> = BitFlags::all();
		Self(empty)
	}

	/// Checks whether the current categories contain a specific
	/// [RuleCategories]
	pub fn contains(&self, other:impl Into<RuleCategories>) -> bool {
		self.0.contains(other.into().0)
	}
}

impl Default for RuleCategories {
	fn default() -> Self { Self::all() }
}

impl RuleCategories {
	pub fn is_syntax(&self) -> bool { self.0.contains(Categories::Syntax) }
}

impl From<RuleCategory> for RuleCategories {
	fn from(input:RuleCategory) -> Self {
		match input {
			RuleCategory::Syntax => RuleCategories(BitFlags::from_flag(Categories::Syntax)),
			RuleCategory::Lint => RuleCategories(BitFlags::from_flag(Categories::Lint)),
			RuleCategory::Action => RuleCategories(BitFlags::from_flag(Categories::Action)),
			RuleCategory::Transformation => {
				RuleCategories(BitFlags::from_flag(Categories::Transformation))
			},
		}
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for RuleCategories {
	fn serialize<S>(&self, serializer:S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer, {
		let mut flags = Vec::new();

		if self.0.contains(Categories::Syntax) {
			flags.push(RuleCategory::Syntax);
		}

		if self.0.contains(Categories::Lint) {
			flags.push(RuleCategory::Lint);
		}

		if self.0.contains(Categories::Action) {
			flags.push(RuleCategory::Action);
		}

		if self.0.contains(Categories::Transformation) {
			flags.push(RuleCategory::Transformation);
		}

		serializer.collect_seq(flags)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RuleCategories {
	fn deserialize<D>(deserializer:D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>, {
		use std::fmt::{self, Formatter};

		use serde::de::{self, SeqAccess};

		struct Visitor;

		impl<'de> de::Visitor<'de> for Visitor {
			type Value = RuleCategories;

			fn expecting(&self, formatter:&mut Formatter) -> fmt::Result {
				write!(formatter, "RuleCategories")
			}

			fn visit_seq<A>(self, mut seq:A) -> Result<Self::Value, A::Error>
			where
				A: SeqAccess<'de>, {
				let mut result = RuleCategories::empty();

				while let Some(item) = seq.next_element::<RuleCategory>()? {
					result.0 |= RuleCategories::from(item).0;
				}

				Ok(result)
			}
		}

		deserializer.deserialize_seq(Visitor)
	}
}

#[cfg(feature = "serde")]
impl schemars::JsonSchema for RuleCategories {
	fn schema_name() -> String { String::from("RuleCategories") }

	fn json_schema(gen:&mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
		<Vec<RuleCategory>>::json_schema(gen)
	}
}

#[derive(Debug, Default)]
/// A convenient type create a [RuleCategories] type
///
/// ```
/// use biome_analyze::{RuleCategoriesBuilder, RuleCategory};
/// let mut categories = RuleCategoriesBuilder::default().with_syntax().with_lint().build();
///
/// assert!(categories.contains(RuleCategory::Lint));
/// assert!(categories.contains(RuleCategory::Syntax));
/// assert!(!categories.contains(RuleCategory::Action));
/// assert!(!categories.contains(RuleCategory::Transformation));
/// ```
pub struct RuleCategoriesBuilder {
	flags:BitFlags<Categories>,
}

impl RuleCategoriesBuilder {
	pub fn with_syntax(mut self) -> Self {
		self.flags.insert(Categories::Syntax);
		self
	}

	pub fn with_lint(mut self) -> Self {
		self.flags.insert(Categories::Lint);
		self
	}

	pub fn with_action(mut self) -> Self {
		self.flags.insert(Categories::Action);
		self
	}

	pub fn with_transformation(mut self) -> Self {
		self.flags.insert(Categories::Transformation);
		self
	}

	pub fn build(self) -> RuleCategories { RuleCategories(self.flags) }
}
