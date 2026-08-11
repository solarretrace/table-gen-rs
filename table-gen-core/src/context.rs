////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator render context module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::ColumnDesc;


////////////////////////////////////////////////////////////////////////////////
// RenderContext
////////////////////////////////////////////////////////////////////////////////
/// Contextual information provided to `Renderer` method calls.
#[derive(Debug, Clone, Copy)]
pub struct RenderContext<'a> {
	/// The default `ColumnDesc` for the table.
	pub default_col_desc: &'a ColumnDesc<'a>,
	/// The `ColumnDesc`s, in column order.
	pub col_descs: &'a [ColumnDesc<'a>],
	/// The column widths, in column order.
	pub col_widths: &'a [usize],
	/// The number of rows in the table.
	pub row_count: usize,
	/// The current row, if applicable.
	pub row: Option<usize>,
	/// The current column, if applicable.
	pub col: Option<usize>,
	/// The crrent line of the row, if applicable.
	pub line: Option<usize>,
}

impl RenderContext<'_> {
	/// Returns the number of columns in the table.
	pub fn column_count(&self) -> usize {
		self.col_widths.len()
	}

	/// Returns `true` if there is no data to render.
	pub fn is_empty(&self) -> bool {
		self.column_count() == 0 || self.row_count == 0
	}

	/// Returns `true` if a cell in the first column is being processed.
	pub fn is_first_column(&self) -> bool {
		self.col == Some(0)
	}

	/// Returns `true` if a cell in the last column is being processed.
	pub fn is_last_column(&self) -> bool {
		self.col == Some(self.column_count().saturating_sub(1))
	}

	/// Returns `true` if a cell in the first row is being processed.
	pub fn is_first_row(&self) -> bool {
		self.row == Some(0)
	}

	/// Returns `true` if a cell in the last row is being processed.
	pub fn is_last_row(&self) -> bool {
		self.row == Some(self.row_count.saturating_sub(1))
	}

	/// Returns `true` if there are no headers to render.
	pub fn is_headerless(&self) -> bool {
		self.col_descs
				.iter()
				.take(self.column_count())
				.all(|col_desc| col_desc.header.is_empty())
			&& self.column_count() <= self.col_descs.len()
			|| ( self.column_count() > self.col_descs.len() 
				&& self.default_col_desc.header.is_empty())
	}

	/// Returns `true` if there are no footers to render.
	pub fn is_footerless(&self) -> bool {
		self.col_descs
				.iter()
				.take(self.column_count())
				.all(|col_desc| col_desc.footer.is_empty())
			&& self.column_count() <= self.col_descs.len()
			|| ( self.column_count() > self.col_descs.len() 
				&& self.default_col_desc.footer.is_empty())
	}
}


////////////////////////////////////////////////////////////////////////////////
// CellContext
////////////////////////////////////////////////////////////////////////////////
/// Contextual information provided to `Renderer` method calls when processing
/// individual cell lines.
#[derive(Debug, Clone, Copy)]
pub struct CellContext<'a> {
	/// The cell line's text.
	pub text: &'a str,
	/// The width of the cell.
	pub width: usize,
	/// The `ColumnDesc` associated with the cell's column.
	pub desc: &'a ColumnDesc<'a>,
}

impl CellContext<'_> {
	/// Returns the difference between the cell width and text length.
	pub fn padding(&self) -> usize {
		self.width.saturating_sub(self.text.len())
	}
}
