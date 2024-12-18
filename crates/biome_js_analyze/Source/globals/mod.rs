//! This module tracks all globals variables

pub mod javascript;
pub use javascript::{
	is_global as is_js_global,
	is_language_global as is_js_language_global,
	is_node_global as is_js_node_global,
	is_web_global as is_js_web_global,
};

pub mod typescript;
pub use typescript::{
	is_global as is_ts_global,
	is_language_global as is_ts_language_global,
	is_node_global as is_ts_node_global,
	is_web_global as is_ts_web_global,
};

pub mod module;
pub use module::is_node_builtin_module;
