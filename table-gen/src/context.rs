////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator render context module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::ColumnDef;
use crate::ColumnDefs;


////////////////////////////////////////////////////////////////////////////////
// RenderContext
////////////////////////////////////////////////////////////////////////////////
/// Contextual information provided to `Renderer` method calls.
#[derive(Debug, Clone, Copy)]
pub struct RenderContext<'a> {
	/// The table `ColumnDefs`.
	pub column_defs: &'a ColumnDefs<'a>,
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
	#[must_use]
	pub fn column_count(&self) -> usize {
		self.col_widths.len()
	}

	/// Returns `true` if there is no data to render.
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.column_count() == 0 || self.row_count == 0
	}

	/// Returns `true` if a cell in the first column is being processed.
	#[must_use]
	pub fn is_first_column(&self) -> bool {
		self.col == Some(0)
	}

	/// Returns `true` if a cell in the last column is being processed.
	#[must_use]
	pub fn is_last_column(&self) -> bool {
		self.col == Some(self.column_count().saturating_sub(1))
	}

	/// Returns `true` if a cell in the first row is being processed.
	#[must_use]
	pub fn is_first_row(&self) -> bool {
		self.row == Some(0)
	}

	/// Returns `true` if a cell in the last row is being processed.
	#[must_use]
	pub fn is_last_row(&self) -> bool {
		self.row == Some(self.row_count.saturating_sub(1))
	}

	/// Returns `true` if there are no headers to render.
	#[must_use]
	pub fn is_headerless(&self) -> bool {
		self.column_defs.is_headerless(self.column_count())
	}

	/// Returns `true` if there are no footers to render.
	#[must_use]
	pub fn is_footerless(&self) -> bool {
		self.column_defs.is_footerless(self.column_count())
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
	/// The width of the text.
	pub text_width: Option<usize>,
	/// The width of the cell.
	pub cell_width: usize,
	/// The `ColumnDef` associated with the cell's column.
	pub desc: &'a ColumnDef<'a>,
}

impl CellContext<'_> {
	/// Returns the difference between the cell width and text length.
	#[must_use]
	pub fn padding(&self) -> usize {
		match self.text_width {
			Some(width) => self.cell_width.saturating_sub(width),
			None        => 0,
		}
	}
}
