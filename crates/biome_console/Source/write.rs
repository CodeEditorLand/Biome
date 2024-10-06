mod html;
mod termcolor;

use std::{fmt, io};

pub use self::{html::HTML, termcolor::Termcolor};
use crate::fmt::MarkupElements;

pub trait Write {
	fn write_str(&mut self, elements:&MarkupElements, content:&str) -> io::Result<()>;
	fn write_fmt(&mut self, elements:&MarkupElements, content:fmt::Arguments) -> io::Result<()>;
}
