use biome_rowan::{SyntaxResult, TokenText};

use crate::{JsonMemberName, inner_string_text};

impl JsonMemberName {
	pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
		Ok(inner_string_text(&self.value_token()?))
	}
}
