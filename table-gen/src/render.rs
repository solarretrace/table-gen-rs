////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator cell formatting module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::CellContext;
use crate::Features;
use crate::HorzAlign;
use crate::RenderContext;
use crate::util::TruncateState;
use crate::util::unicode_grapheme_aware_truncation;


////////////////////////////////////////////////////////////////////////////////
// Renderer
////////////////////////////////////////////////////////////////////////////////
/// Provides methods needed to implement a table renderer.
pub trait Renderer {
	/// Returns the supported features for the renderer.
	#[must_use]
	fn features(&self) -> Features;

	/// Initializes the renderer. Will be called before any rendering begins.
	fn init(&mut self, _ctx: &RenderContext<'_>) {}

	// Data writing hooks
	////////////////////////////////////////////////////////////////////////////
	/// Hook for writing single cell's line in a data row.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_data_cell_line<W>(
		&mut self,
		out: &mut W,
		_ctx: &RenderContext<'_>,
		cell: &CellContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		let text_width = match cell.text_width {
			None        => return write!(out, "{}", cell.text),
			Some(width) => width,
		};
		let align = cell.desc.horz_align;
		let cell_width = cell.cell_width;
		let ellipses_space = if align == HorzAlign::Center { 2 } else { 1 };

		let (text, state) = if text_width > cell_width {
			unicode_grapheme_aware_truncation(
				cell.text,
				text_width,
				cell_width.saturating_sub(ellipses_space),
				cell.desc.horz_align)
		} else {
			(cell.text, TruncateState::Neither(text_width))
		};

		// Compute cell padding.
		let pad = cell.cell_width.saturating_sub(state.width());
		let (mut l_pad, mut r_pad) = match align {
			HorzAlign::Left   => (0,     pad),
			HorzAlign::Center => (pad/2, pad.div_ceil(2)),
			HorzAlign::Right  => (pad,   0),
		};
		if state.left_truncated() { l_pad = l_pad.saturating_sub(1); }
		if state.right_truncated() { r_pad = r_pad.saturating_sub(1); }

		
		for _ in 0..l_pad { write!(out, " ")?; }
		if state.left_truncated() { write!(out, "…")?; }
		write!(out, "{}", text)?;
		if state.right_truncated() { write!(out, "…")?; }
		for _ in 0..r_pad { write!(out, " ")?; }
		Ok(())
	}

	/// Hook for writing single cell's line in a header row.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_header_cell_line<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>,
		cell: &CellContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_data_cell_line(out, ctx, cell)
	}

	/// Hook for writing single cell's line in a footer row.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_footer_cell_line<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>,
		cell: &CellContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_data_cell_line(out, ctx, cell)
	}

	// Section hooks
	////////////////////////////////////////////////////////////////////////////
	/// Hook for writing at the start of the table render.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_table_start<W>(&mut self, _out: &mut W, _ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of the table render.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_table_end<W>(&mut self, _out: &mut W, _ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the start of the header.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_header_start<W>(&mut self, _out: &mut W, _ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of the header.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_header_end<W>(&mut self, _out: &mut W, _ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the start of the data.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_data_start<W>(&mut self, _out: &mut W, _ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of the data.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_data_end<W>(&mut self, _out: &mut W, _ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the start of the footer.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_footer_start<W>(&mut self, _out: &mut W, _ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of the footer.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_footer_end<W>(&mut self, _out: &mut W, _ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	// Row-based hooks
	////////////////////////////////////////////////////////////////////////////

	/// Hook for writing at the start of a header row.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_header_row_start<W>(
		&mut self,
		_out: &mut W, 
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a header row.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_header_row_end<W>(
		&mut self,
		_out: &mut W, 
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}


	/// Hook for writing at the start of a data row.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_data_row_start<W>(
		&mut self,
		_out: &mut W, 
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a data row.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_data_row_end<W>(
		&mut self,
		_out: &mut W, 
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the start of a footer row.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_footer_row_start<W>(
		&mut self,
		_out: &mut W, 
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a footer row.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_footer_row_end<W>(
		&mut self,
		_out: &mut W, 
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	// Cell-based hooks
	////////////////////////////////////////////////////////////////////////////

	/// Hook for writing at the start of a header cell.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_header_cell_start<W>(
		&mut self,
		_out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a header cell.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_header_cell_end<W>(
		&mut self,
		_out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the start of a data cell.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_data_cell_start<W>(
		&mut self,
		_out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a data cell.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_data_cell_end<W>(
		&mut self,
		_out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the start of a footer cell.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_footer_cell_start<W>(
		&mut self,
		_out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a footer cell.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_footer_cell_end<W>(
		&mut self,
		_out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	// Line-based hooks
	////////////////////////////////////////////////////////////////////////////
	/// Hook for writing at the start of a header line.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_header_line_start<W>(
		&mut self,
		_out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a header line.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_header_line_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_data_line_end(out, ctx)
	}

	/// Hook for writing at the start of a data line.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_data_line_start<W>(
		&mut self,
		_out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a data line.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_data_line_end<W>(
		&mut self,
		out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out)
	}

	/// Hook for writing at the start of a footer line.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_footer_line_start<W>(
		&mut self,
		_out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a footer line.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_footer_line_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_data_line_end(out, ctx)
	}

	// Cell line-based hooks
	////////////////////////////////////////////////////////////////////////////

	/// Hook for writing at the start of a cell line.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_header_cell_line_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_data_cell_line_start(out, ctx)
	}

	/// Hook for writing at the end of a cell line.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_header_cell_line_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_data_cell_line_end(out, ctx)
	}

	/// Hook for writing at the start of a cell line.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_data_cell_line_start<W>(
		&mut self,
		_out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		Ok(())
	}

	/// Hook for writing at the end of a cell line.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_data_cell_line_end<W>(
		&mut self,
		out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		write!(out, " ")
	}

	/// Hook for writing at the start of a cell line.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_footer_cell_line_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_data_cell_line_start(out, ctx)
	}

	/// Hook for writing at the end of a cell line.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	fn write_footer_cell_line_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_data_cell_line_end(out, ctx)
	}
}
