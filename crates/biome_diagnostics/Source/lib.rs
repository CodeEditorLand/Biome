#![deny(rust_2018_idioms)]

use ::serde::{Deserialize, Serialize};

pub mod adapters;
pub mod advice;
pub mod context;
pub mod diagnostic;
pub mod display;
pub mod display_github;
pub mod error;
pub mod location;
pub mod panic;
pub mod serde;

mod suggestion;

use std::fmt::Write;

#[doc(hidden)]
// Convenience re-export for procedural macro
pub use biome_console as console;
use biome_console::{
	fmt::{Formatter, Termcolor},
	markup,
};
// Re-export macros from utility crates
pub use biome_diagnostics_categories::{Category, category, category_concat};
pub use biome_diagnostics_macros::Diagnostic;
pub use termcolor;

pub use self::suggestion::{Applicability, CodeSuggestion};
pub use crate::{
	advice::{Advices, CodeFrameAdvice, CommandAdvice, DiffAdvice, LogAdvice, LogCategory, Visit},
	context::{Context, DiagnosticExt},
	diagnostic::{Diagnostic, DiagnosticTags, Severity},
	display::{
		Backtrace,
		MessageAndDescription,
		PrintDescription,
		PrintDiagnostic,
		set_bottom_frame,
	},
	display_github::PrintGitHubDiagnostic,
	error::{Error, Result},
	location::{LineIndex, LineIndexBuf, Location, Resource, SourceCode},
};

pub mod prelude {
	//! Anonymously re-exports all the traits declared by this module, this is
	//! intended to be imported as `use biome_diagnostics::prelude::*;` to
	//! automatically bring all these traits into the ambient context

	pub use crate::{
		advice::{Advices as _, Visit as _},
		context::{Context as _, DiagnosticExt as _},
		diagnostic::Diagnostic as _,
	};
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum DiagnosticTag {
	Unnecessary,
	Deprecated,
	Both,
}

impl DiagnosticTag {
	pub fn is_unnecessary(&self) -> bool {
		matches!(self, DiagnosticTag::Unnecessary | DiagnosticTag::Both)
	}

	pub fn is_deprecated(&self) -> bool {
		matches!(self, DiagnosticTag::Deprecated | DiagnosticTag::Both)
	}
}

/// Utility function for testing purpose. The function will print an [Error]
/// to a string, which is then returned by the function.
pub fn print_diagnostic_to_string(diagnostic:&Error) -> String {
	let mut buffer = termcolor::Buffer::no_color();

	Formatter::new(&mut Termcolor(&mut buffer))
		.write_markup(markup! {
			{PrintDiagnostic::verbose(diagnostic)}
		})
		.expect("failed to emit diagnostic");

	let mut content = String::new();

	writeln!(
		content,
		"{}",
		std::str::from_utf8(buffer.as_slice()).expect("non utf8 in error buffer")
	)
	.unwrap();

	content
}
