////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Renderer features module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::util::unicode_display_width;

// Standard library imports.
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////
// Features
////////////////////////////////////////////////////////////////////////////////
/// `Renderer` feature settings. Provides customized behavior for the table
/// renderer driver to support renderer-specific requirements.
#[allow(missing_copy_implementations)]
#[allow(missing_debug_implementations)]
pub struct Features {
	/// Function to use for calculating column widths. A `None` value means
	/// column widths should not be calculated
	pub str_width_fn: Option<fn(&str) -> usize>,

	/// Function to apply processing to formatted cell text before aggregation.
	pub early_format_fn: Option<Rc<dyn Fn(String) -> String>>,

	/// Function to apply processing to formatted cell text after aggregation.
	pub late_format_fn: Option<Rc<dyn Fn(&str, usize, usize) -> String>>,

	/// Function for computing the table width contribution of the renderer. It
	/// will be provided the number of columns being rendered.
	pub width_contribution_fn: Option<Box<dyn Fn(usize) -> usize>>,

	/// Extra width to pad all columns.
	pub extra_column_width: usize,
}

impl Default for Features {
	fn default() -> Self {
		Self::new()
	}
}

impl Features {
	/// Contstructs a new `Features` with the default options.
	#[must_use]
	pub fn new() -> Self {
		Self {
			str_width_fn: Some(unicode_display_width),
			early_format_fn: None,
			late_format_fn: None,
			width_contribution_fn: Some(
				Box::new(Self::interspersed_space_width)),
			extra_column_width: 0,
		}
	}

	/// Sets the function to use for calculating column widths and returns the 
	/// `Features`.
	///
	/// The following functions are available as suitable arguments:
	/// + `Features::unicode_display_width` is the default value.
	/// + `str::len` is suitable if the inputs are always ASCII.
	#[must_use]
	pub fn with_str_width_fn(mut self, str_width_fn: Option<fn(&str) -> usize>)
		-> Self
	{
		self.str_width_fn = str_width_fn;
		self
	}

	/// Sets the function to use for pre-aggregation cell formatting and returns
	/// the `Features`.
	///
	/// The following functions are available as suitable arguments:
	/// + `std::convert::identity` is the default value.
	/// + `Features::remove_line_breaks` to prevent multiline cells.
	#[must_use]
	pub fn with_early_format_fn(
		mut self,
		early_format_fn: Rc<dyn Fn(String) -> String>)
		-> Self
	{
		self.early_format_fn = Some(early_format_fn);
		self
	}

	/// Sets the function to use for post-aggregation cell formatting and
	/// returns the `Features`.
	///
	/// The function parameters are as follows:
	///
	/// 1. The formatted cell text.
	/// 2. The column index.
	/// 3. The column width.
	#[must_use]
	pub fn with_late_format_fn(
		mut self,
		late_format_fn: Rc<dyn Fn(&str, usize, usize) -> String>)
		-> Self
	{
		self.late_format_fn = Some(late_format_fn);
		self
	}

	/// Sets the function to use for computing the irreducable portion of the
	/// table width. This should match to total all border widths across the
	/// provided number of rows.
	#[must_use]
	pub fn with_width_contribution_fn(
		mut self,
		width_contribution_fn: Box<dyn Fn(usize) -> usize>)
		-> Self
	{
		self.width_contribution_fn = Some(width_contribution_fn);
		self
	}

	/// Sets the extra column width for each table column.
	#[must_use]
	pub fn with_extra_column_width(mut self, extra_column_width: usize) -> Self
	{
		self.extra_column_width = extra_column_width;
		self
	}

	/// Replaces line break chars ("\r\n", "\r", "\n") in the given `String`
	/// with spaces.
	#[must_use]
	pub fn remove_line_breaks(text: String) -> String {
		let mut out = String::with_capacity(text.len());
		let mut chars = text.chars().peekable();
		while let Some(c) = chars.next() {
			match c {
				'\r' => {
					if chars.peek() == Some(&'\n') {
						let _ = chars.next(); // Consume the \n.
					}
					out.push(' ');
				},
				'\n' => out.push(' '),
				_    => out.push(c),
			}
		}
		out
	}

	fn interspersed_space_width(col_count: usize) -> usize {
		col_count.saturating_sub(1)
	}
}
