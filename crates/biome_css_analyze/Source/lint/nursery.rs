//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;

pub mod no_duplicate_custom_properties;
pub mod no_irregular_whitespace;
pub mod no_unknown_pseudo_class;
pub mod no_unknown_pseudo_element;
pub mod no_value_at_rule;

declare_lint_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: no_duplicate_custom_properties :: NoDuplicateCustomProperties ,
            self :: no_irregular_whitespace :: NoIrregularWhitespace ,
            self :: no_unknown_pseudo_class :: NoUnknownPseudoClass ,
            self :: no_unknown_pseudo_element :: NoUnknownPseudoElement ,
            self :: no_value_at_rule :: NoValueAtRule ,
        ]
     }
}
