////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator cell formatting module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::Cell;
use crate::Collate;
use crate::CollateRow;
use crate::ColumnDesc;
use crate::ColumnOrd;
use crate::Features;
use crate::Row;

// Standard library imports.
use std::cell::OnceCell;
use std::fmt::Display;
use std::ops::Bound;


////////////////////////////////////////////////////////////////////////////////
// Format
////////////////////////////////////////////////////////////////////////////////
/// Table cell formatter. Responsible for generating text for cells.
#[derive(Debug, Clone)]
pub (in crate) struct Format<'a, S> {
	/// The table data source.
	inner: Collate<'a, S>,
}

impl<R, S, I> From<I> for Format<'_, S>
	where
		R: Row,
		S: Iterator<Item=R>,
		I: IntoIterator<Item=R, IntoIter=S>,
{
	fn from(inner: I) -> Self {
		Format::new(inner.into())
	}
}

impl<'a, R, S> From<Collate<'a, S>> for Format<'a, S>
	where
		R: Row,
		S: Iterator<Item=R>,
{
	fn from(inner: Collate<'a, S>) -> Self {
		Format::new(inner)
	}
}

impl<'a, R, S> Format<'a, S>
	where
		S: Iterator<Item=R>,
		R: Row,
{
	/// Constructs a new `Format` for the given data source.
	#[must_use]
	pub (in crate) fn new(inner: Collate<'a, S>) -> Self {
		Self {
			inner,
		}
	}

	/// Returns the supported features for the renderer.
	#[must_use]
	pub (in crate) fn features(&self) -> &Features {
		self.inner.features()
	}

	/// The row selection bounds.
	#[must_use]
	pub (in crate) fn row_selection(&self) -> &(Bound<usize>, Bound<usize>) {
		self.inner.row_selection()
	}

	/// The column output specifications.
	#[must_use]
	pub (in crate) fn column_descs(&self) -> &'a [ColumnDesc<'a>] {
		self.inner.column_descs()
	}

	/// The sort parameters for columns, in order of sort priority.
	#[must_use]
	pub (in crate) fn column_order(&self) -> &'a [ColumnOrd] {
		self.inner.column_order()
	}
}

impl<'a, R, S> Iterator for Format<'a, S>
	where
		S: Iterator<Item=R>,
		R: Row,
{
	type Item = FormatRow<'a, R>;
	fn next(&mut self) -> Option<Self::Item> {
		self.inner
			.next()
			.map(|collate_row| FormatRow::new(
				collate_row,
				self.inner.column_descs(),
				self.features().post_format_fn))
	}
}


////////////////////////////////////////////////////////////////////////////////
// FormatRow
////////////////////////////////////////////////////////////////////////////////
/// A single table row with collated column selection and ordering.
#[derive(Debug, Clone)]
pub (in crate) struct FormatRow<'a, R> {
	/// The row to format.
	inner: CollateRow<'a, R>,
	/// The column output specifications,
	col_descs: &'a [ColumnDesc<'a>],
	/// The cached cell texts.
	cache: Vec<OnceCell<Box<str>>>,
	/// Function to apply post-processing to formatted cell text.
	post_format_fn: fn(String) -> String,
}

impl<R> Row for FormatRow<'_, R>
	where R: Row
{
	fn len(&self) -> usize {
		self.inner.len()
	}

	fn cell(&self, col_idx: usize) -> Option<&dyn Cell> {
		self.inner.cell(col_idx)
	}
}

impl<'a, R> FormatRow<'a, R>
	where R: Row,
{
	/// Constructs a new `FormatRow` over the given `CollateRow` and
	/// `ColumnDesc`s.
	#[must_use]
	pub (in crate) fn new(
		inner: CollateRow<'a, R>,
		col_descs: &'a [ColumnDesc<'a>],
		post_format_fn: fn(String) -> String)
		-> Self
	{
		let cache = vec![OnceCell::new(); inner.len()];
		Self {
			inner,
			col_descs,
			cache,
			post_format_fn
		}
	}

	/// Returns the text of the cell at the given column index.
	#[must_use]
	pub (in crate) fn text(&self, col_idx: usize) -> &str {
		self.cache[col_idx].get_or_init(|| match self.inner.cell(col_idx) {
			Some(cell) => {
				(self.post_format_fn)(self.col_descs
						.get(col_idx)
						.map_or_else(
							DisplayFmt::default,
							|spec| spec.display_fmt)
						.apply(cell))
					.into_boxed_str()
			},
			None => String::new().into_boxed_str(),
		})
	}
}

////////////////////////////////////////////////////////////////////////////////
// DisplayFmt
////////////////////////////////////////////////////////////////////////////////
/// Parameters for cell `Display` output formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DisplayFmt {
	/// Format precision specifier.
	precision: Option<usize>,
	/// Format sign specifier.
	sign: Option<Sign>,
}

impl Default for DisplayFmt {
	fn default() -> Self {
		Self::new()
	}
}

impl DisplayFmt {
	/// Constructs a new `DisplayFmt` with the default settings.
	#[must_use]
	pub fn new() -> Self {
		Self {
			precision: None,
			sign: None,
		}
	}

	/// Sets the precision and returns the `DisplayFmt`.
	///
	/// Indicates that the cell value should be formatted with the given
	/// precision. See the 
	/// [std library formatting specification](https://doc.rust-lang.org/std/fmt/index.html#precision)
	/// for details.
	#[must_use]
	pub fn with_precision<T>(mut self, precision: T) -> Self 
		where T: Into<Option<usize>>
	{
		self.precision = precision.into();
		self
	}

	/// Sets the sign and returns the `DisplayFmt`.
	///
	/// Indicates that the cell value should be formatted with the given sign.
	/// See the 
	/// [std library formatting specification](https://doc.rust-lang.org/std/fmt/index.html#sign0)
	/// for details.
	#[must_use]
	pub fn with_sign<T>(mut self, sign: T) -> Self 
		where T: Into<Option<Sign>>
	{
		self.sign = sign.into();
		self
	}
	
	/// Applies the `DisplayFmt` to the given cell, rendering it as a
	/// `Box<str>`.
	#[must_use]
	pub fn apply<C>(&self, cell: C) -> String
		where C: Display
	{
		use Sign::*;
		match (self.precision, self.sign) {
			(Some(p), Some(Plus))  => format!("{:+.p$}", cell, p=p),
			(Some(p), Some(Minus)) => format!("{:-.p$}", cell, p=p),
			(Some(p), Some(Zero))  => format!("{:0.p$}", cell, p=p),
			(Some(p), None)        => format!("{:.p$}", cell, p=p),
			(None,    Some(Plus))  => format!("{:+}", cell),
			(None,    Some(Minus)) => format!("{:-}", cell),
			(None,    Some(Zero))  => format!("{:0}", cell),
			(None,    None)        => format!("{}", cell),
		}
	}
}

/// Format sign specifier.
///
/// Indicates that the cell value should be formatted with the given sign. See
/// the 
/// [std library formatting specification](https://doc.rust-lang.org/std/fmt/index.html#sign0)
/// for details.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sign {
	/// Format numerical values with the '+' format option.
	Plus,
	/// Format numerical values with the '-' format option.
	Minus,
	/// Format numerical values with the '0' format option.
	Zero,
}
