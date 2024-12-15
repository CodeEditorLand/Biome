use biome_rowan::{SyntaxResult, TokenText};

use crate::{JsonStringValue, inner_string_text};

impl JsonStringValue {
	pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
		Ok(inner_string_text(&self.value_token()?))
	}
}
