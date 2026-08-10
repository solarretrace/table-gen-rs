////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator cell formatting module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::ColumnDesc;
use crate::HorzAlign;

// External library imports.
use bitflags::bitflags;


////////////////////////////////////////////////////////////////////////////////
// Features
////////////////////////////////////////////////////////////////////////////////
bitflags! {
	/// Renderer feature flags.
	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct Features: u8 {
		/// Indicates that multi-line cells are supported.
		const MULTILINE = 0b_0000_0001;
	}
}


////////////////////////////////////////////////////////////////////////////////
// Renderer
////////////////////////////////////////////////////////////////////////////////
/// Provides methods needed to implement a table renderer.
pub trait Renderer {
	/// Returns the supported features for the renderer.
	fn features(&self) -> Features;

	/// Initializes the renderer. Will be called before any rendering begins.
	fn init(
		&mut self,
		_column_descs: &[ColumnDesc<'_>],
		_row_count: usize,
		_column_widths: &[usize])
	{}

	// Data writing hooks
	////////////////////////////////////////////////////////////////////////////
	/// Hook for writing single cell's line in a data row.
	fn write_data_cell_line<W>(
		&mut self,
		out: &mut W,
		_row: usize,
		_col: usize,
		_line: usize,
		text: &str,
		width: usize,
		horz_align: HorzAlign)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		let pad = width.saturating_sub(text.len());
		let (l_pad, r_pad) = match horz_align {
			HorzAlign::Left   => (0,     pad),
			HorzAlign::Center => (pad/2, pad.div_ceil(2)),
			HorzAlign::Right  => (pad,   0),
		};
		
		for _ in 0..l_pad { write!(out, " ")?; }
		write!(out, "{}", text)?;
		for _ in 0..r_pad { write!(out, " ")?; }
		Ok(())
	}

	/// Hook for writing single cell's line in a header row.
	fn write_header_cell_line<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize,
		line: usize,
		text: &str,
		width: usize,
		horz_align: HorzAlign)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_data_cell_line(out, row, col, line, text, width, horz_align)
	}

	/// Hook for writing single cell's line in a footer row.
	fn write_footer_cell_line<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize,
		line: usize,
		text: &str,
		width: usize,
		horz_align: HorzAlign)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_data_cell_line(out, row, col, line, text, width, horz_align)
	}

	// Section hooks
	////////////////////////////////////////////////////////////////////////////
	/// Hook for writing at the start of the table render.
	fn write_table_start<W>(&mut self, _out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of the table render.
	fn write_table_end<W>(&mut self, _out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the start of the header.
	fn write_header_start<W>(&mut self, _out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of the header.
	fn write_header_end<W>(&mut self, _out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the start of the data.
	fn write_data_start<W>(&mut self, _out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of the data.
	fn write_data_end<W>(&mut self, _out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the start of the footer.
	fn write_footer_start<W>(&mut self, _out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of the footer.
	fn write_footer_end<W>(&mut self, _out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	// Row-based hooks
	////////////////////////////////////////////////////////////////////////////

	/// Hook for writing at the start of a header row.
	fn write_header_row_start<W>(&mut self, _out: &mut W, _row: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a header row.
	fn write_header_row_end<W>(&mut self, _out: &mut W, _row: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}


	/// Hook for writing at the start of a data row.
	fn write_data_row_start<W>(&mut self, _out: &mut W, _row: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a data row.
	fn write_data_row_end<W>(&mut self, _out: &mut W, _row: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the start of a footer row.
	fn write_footer_row_start<W>(&mut self, _out: &mut W, _row: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a footer row.
	fn write_footer_row_end<W>(&mut self, _out: &mut W, _row: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}


	// Cell-based hooks
	////////////////////////////////////////////////////////////////////////////

	/// Hook for writing at the start of a header cell.
	fn write_header_cell_start<W>(
		&mut self,
		_out: &mut W,
		_row: usize,
		_col: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a header cell.
	fn write_header_cell_end<W>(
		&mut self,
		_out: &mut W,
		_row: usize,
		_col: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the start of a data cell.
	fn write_data_cell_start<W>(
		&mut self,
		_out: &mut W,
		_row: usize,
		_col: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a data cell.
	fn write_data_cell_end<W>(
		&mut self,
		_out: &mut W,
		_row: usize,
		_col: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the start of a footer cell.
	fn write_footer_cell_start<W>(
		&mut self,
		_out: &mut W,
		_row: usize,
		_col: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a footer cell.
	fn write_footer_cell_end<W>(
		&mut self,
		_out: &mut W,
		_row: usize,
		_col: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}


	// Line-based hooks
	////////////////////////////////////////////////////////////////////////////
	/// Hook for writing at the start of a header line.
	fn write_header_line_start<W>(
		&mut self,
		_out: &mut W,
		_row: usize,
		_line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a header line.
	fn write_header_line_end<W>(
		&mut self,
		out: &mut W,
		row: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_data_line_end(out, row, line)
	}

	/// Hook for writing at the start of a data line.
	fn write_data_line_start<W>(
		&mut self,
		_out: &mut W,
		_row: usize,
		_line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a data line.
	fn write_data_line_end<W>(
		&mut self,
		out: &mut W,
		_row: usize,
		_line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out)
	}

	/// Hook for writing at the start of a footer line.
	fn write_footer_line_start<W>(
		&mut self,
		_out: &mut W,
		_row: usize,
		_line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a footer line.
	fn write_footer_line_end<W>(
		&mut self,
		out: &mut W,
		row: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_data_line_end(out, row, line)
	}

	// Cell line-based hooks
	////////////////////////////////////////////////////////////////////////////

	/// Hook for writing at the start of a cell line.
	fn write_header_cell_line_start<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_data_cell_line_start(out, row, col, line)
	}

	/// Hook for writing at the end of a cell line.
	fn write_header_cell_line_end<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_data_cell_line_end(out, row, col, line)
	}

	/// Hook for writing at the start of a cell line.
	fn write_data_cell_line_start<W>(
		&mut self,
		_out: &mut W,
		_row: usize,
		_col: usize,
		_line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a cell line.
	fn write_data_cell_line_end<W>(
		&mut self,
		out: &mut W,
		_row: usize,
		_col: usize,
		_line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		write!(out, " ")
	}

	/// Hook for writing at the start of a cell line.
	fn write_footer_cell_line_start<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_data_cell_line_start(out, row, col, line)
	}

	/// Hook for writing at the end of a cell line.
	fn write_footer_cell_line_end<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_data_cell_line_end(out, row, col, line)
	}
}



