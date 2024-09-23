//! Definitions for the ECMAScript AST used for codegen
//! Based on the rust analyzer parser and ast definitions

use crate::kind_src::KindsSrc;

pub const GRIT_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        ("...", "DOT3"),
        ("$_", "DOLLAR_UNDERSCORE"),
        ("<:", "MATCH"),
        (";", "SEMICOLON"),
        (",", "COMMA"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
        ("<", "L_ANGLE"),
        (">", "R_ANGLE"),
        ("+", "PLUS"),
        ("*", "STAR"),
        ("/", "SLASH"),
        ("%", "PERCENT"),
        (".", "DOT"),
        (":", "COLON"),
        ("=", "EQ"),
        ("==", "EQ2"),
        ("=>", "FAT_ARROW"),
        ("!", "BANG"),
        ("!=", "NEQ"),
        ("-", "MINUS"),
        ("<=", "LTEQ"),
        (">=", "GTEQ"),
        ("+=", "PLUSEQ"),
        ("`", "BACKTICK"),
    ],
    keywords: &[
        // top-level:
        "sequential",
        "multifile",
        "engine",
        "biome",
        "language",
        // languages:
        "js",
        "css",
        "json",
        "grit",
        "html",
        // language flavors:
        "typescript",
        "jsx",
        "js_do_not_use",
        // clauses:
        "as",
        "limit",
        "where",
        "orelse",
        "maybe",
        "after",
        "before",
        "contains",
        "until",
        "includes",
        "if",
        "else",
        "within",
        "bubble",
        "not",
        // compound patterns:
        "or",
        "and",
        "any",
        // matches:
        "some",
        "every",
        // definitions:
        "private",
        "pattern",
        "predicate",
        "function",
        // values:
        "true",
        "false",
        "undefined",
        // other:
        "like",
        "return",
    ],
    literals: &[
        "GRIT_INT",
        "GRIT_NEGATIVE_INT",
        "GRIT_DOUBLE",
        "GRIT_STRING",
        "GRIT_REGEX",
        "GRIT_SNIPPET_REGEX",
    ],
    tokens: &[
        "NEWLINE",
        "WHITESPACE",
        "COMMENT",
        "MULTILINE_COMMENT",
        "ERROR_TOKEN",
        "GRIT_ANNOTATION",
        "GRIT_BACKTICK_SNIPPET",
        "GRIT_RAW_BACKTICK_SNIPPET",
        "GRIT_NAME",
        "GRIT_VARIABLE",
    ],
    nodes: &[
        "GRIT_BRACKETED_PATTERN",
        "GRIT_BRACKETED_PREDICATE",
        "GRIT_CURLY_PATTERN",
        "GRIT_ROOT",
        "GRIT_SEQUENTIAL",
        "GRIT_FILES",
        "GRIT_DEFINITION_LIST",
        "GRIT_VERSION",
        "GRIT_LANGUAGE_DECLARATION",
        "GRIT_LANGUAGE_FLAVOR",
        "GRIT_LANGUAGE_FLAVOR_LIST",
        "GRIT_LANGUAGE_FLAVOR_KIND",
        "GRIT_PATTERN_LIST",
        "GRIT_MUL_OPERATION",
        "GRIT_DIV_OPERATION",
        "GRIT_MOD_OPERATION",
        "GRIT_ADD_OPERATION",
        "GRIT_SUB_OPERATION",
        "GRIT_PATTERN_AS",
        "GRIT_PATTERN_LIMIT",
        "GRIT_ASSIGNMENT_AS_PATTERN",
        "GRIT_PATTERN_ACCUMULATE",
        "GRIT_PATTERN_WHERE",
        "GRIT_PATTERN_NOT",
        "GRIT_PATTERN_OR",
        "GRIT_PATTERN_OR_ELSE",
        "GRIT_PATTERN_ANY",
        "GRIT_PATTERN_AND",
        "GRIT_PATTERN_MAYBE",
        "GRIT_PATTERN_AFTER",
        "GRIT_PATTERN_BEFORE",
        "GRIT_PATTERN_CONTAINS",
        "GRIT_PATTERN_CONTAINS_UNTIL_CLAUSE",
        "GRIT_PATTERN_INCLUDES",
        "GRIT_REWRITE",
        "GRIT_PATTERN_IF_ELSE",
        "GRIT_PATTERN_ELSE_CLAUSE",
        "GRIT_WITHIN",
        "GRIT_BUBBLE_SCOPE",
        "GRIT_BUBBLE",
        "GRIT_NAMED_ARG",
        "GRIT_NAMED_ARG_LIST",
        "GRIT_NODE_LIKE",
        "GRIT_LIKE",
        "GRIT_LIKE_THRESHOLD",
        "GRIT_MAP",
        "GRIT_MAP_ELEMENT_LIST",
        "GRIT_MAP_ELEMENT",
        "GRIT_MAP_ACCESSOR",
        "GRIT_LIST",
        "GRIT_LIST_PATTERN_LIST",
        "GRIT_LIST_ACCESSOR",
        "GRIT_DOT",
        "GRIT_DOTDOTDOT",
        "GRIT_SOME",
        "GRIT_EVERY",
        "GRIT_REGEX_PATTERN",
        "GRIT_REGEX_PATTERN_VARIABLES",
        "GRIT_PATTERN_DEFINITION_BODY",
        "GRIT_PATTERN_DEFINITION",
        "GRIT_PATTERN_ARG_LIST",
        "GRIT_PREDICATE_LIST",
        "GRIT_CURLY_PREDICATE_LIST",
        "GRIT_PREDICATE_DEFINITION",
        "GRIT_FUNCTION_DEFINITION",
        "GRIT_PREDICATE_NOT",
        "GRIT_PREDICATE_MAYBE",
        "GRIT_PREDICATE_AND",
        "GRIT_PREDICATE_OR",
        "GRIT_PREDICATE_ANY",
        "GRIT_PREDICATE_IF_ELSE",
        "GRIT_PREDICATE_ELSE_CLAUSE",
        "GRIT_PREDICATE_REWRITE",
        "GRIT_PREDICATE_ASSIGNMENT",
        "GRIT_PREDICATE_ACCUMULATE",
        "GRIT_PREDICATE_GREATER",
        "GRIT_PREDICATE_LESS",
        "GRIT_PREDICATE_GREATER_EQUAL",
        "GRIT_PREDICATE_LESS_EQUAL",
        "GRIT_PREDICATE_NOT_EQUAL",
        "GRIT_PREDICATE_EQUAL",
        "GRIT_PREDICATE_MATCH",
        "GRIT_PREDICATE_CALL",
        "GRIT_PREDICATE_RETURN",
        "GRIT_VARIABLE_LIST",
        "GRIT_LANGUAGE_NAME",
        "GRIT_LANGUAGE_SPECIFIC_SNIPPET",
        "GRIT_CODE_SNIPPET",
        "GRIT_NOT",
        "GRIT_UNDERSCORE",
        // literal wrappers:
        "GRIT_BACKTICK_SNIPPET_LITERAL",
        "GRIT_BOOLEAN_LITERAL",
        "GRIT_UNDEFINED_LITERAL",
        "GRIT_INT_LITERAL",
        "GRIT_NEGATIVE_INT_LITERAL",
        "GRIT_DOUBLE_LITERAL",
        "GRIT_STRING_LITERAL",
        "GRIT_RAW_BACKTICK_SNIPPET_LITERAL",
        "GRIT_REGEX_LITERAL",
        "GRIT_SNIPPET_REGEX_LITERAL",
        // bogus nodes:
        "GRIT_BOGUS",
        "GRIT_BOGUS_CONTAINER",
        "GRIT_BOGUS_DEFINITION",
        "GRIT_BOGUS_MAP_ELEMENT",
        "GRIT_BOGUS_LANGUAGE_DECLARATION",
        "GRIT_BOGUS_LANGUAGE_FLAVOR_KIND",
        "GRIT_BOGUS_LITERAL",
        "GRIT_BOGUS_NAMED_ARG",
        "GRIT_BOGUS_PATTERN",
        "GRIT_BOGUS_PREDICATE",
        "GRIT_BOGUS_VERSION",
    ],
};
