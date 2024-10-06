use grit_pattern_matcher::pattern::Pattern;

use crate::{GritPattern, ParseError};

pub fn parse_pattern(source:String) -> Result<GritPattern, ParseError> {
	Ok(GritPattern { _pattern:Pattern::Undefined, source })
}
