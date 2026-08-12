////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! A table renderer that prints called methods args.
////////////////////////////////////////////////////////////////////////////////

// Workspace library imports.
use table_gen::CellContext;
use table_gen::Features;
use table_gen::RenderContext;
use table_gen::Renderer;


////////////////////////////////////////////////////////////////////////////////
// DebugRenderer
////////////////////////////////////////////////////////////////////////////////
/// A table renderer that prints called methods args.
#[derive(Debug, Clone, Copy, Default)]
pub struct DebugRenderer;

impl Renderer for DebugRenderer {
	fn features(&self) -> Features {
		Features::default()
	}

	// Data writing hooks
	///////////////////////////////////////////////
	fn write_data_cell_line<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>,
		cell: &CellContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_cell_line(\
			row: {:?}, col: {:?}, line: {:?}, {:?})",
			ctx.row, ctx.col, ctx.line, cell)
	}

	fn write_header_cell_line<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>,
		cell: &CellContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_cell_line(\
			row: {:?}, col: {:?}, line: {:?}, {:?})",
			ctx.row, ctx.col, ctx.line, cell)
	}

	fn write_footer_cell_line<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>,
		cell: &CellContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_cell_line(\
			row: {:?}, col: {:?}, line: {:?}, {:?})",
			ctx.row, ctx.col, ctx.line, cell)
	}

	// Section hooks
	///////////////////////////////////////////////
	fn write_table_start<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_table_start(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_table_end<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_table_end(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_header_start<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_start(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_header_end<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_end(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_data_start<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_start(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_data_end<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_end(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_footer_start<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_start(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_footer_end<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_end(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	// Row-based hooks
	////////////////////////////////////////////////////////////////////////////

	fn write_header_row_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_row_start(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_header_row_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_row_end(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}


	fn write_data_row_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_row_start(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_data_row_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_row_end(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_footer_row_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_row_start(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_footer_row_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_row_end(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}


	// Cell-based hooks
	////////////////////////////////////////////////////////////////////////////

	fn write_header_cell_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_cell_start(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_header_cell_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_cell_end(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_data_cell_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_cell_start(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_data_cell_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_cell_end(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_footer_cell_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_cell_start(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_footer_cell_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_cell_end(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}


	// Line-based hooks
	///////////////////////////////////////////////
	fn write_header_line_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_line_start(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_header_line_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_line_end(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_data_line_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_line_start(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_data_line_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_line_end(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_footer_line_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_line_start(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_footer_line_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_line_end(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	// Cell line-based hooks
	////////////////////////////////////////////////////////////////////////////

	fn write_header_cell_line_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_cell_line_start(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_header_cell_line_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_cell_line_end(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_data_cell_line_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_cell_line_start(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_data_cell_line_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_cell_line_end(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_footer_cell_line_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_cell_line_start(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}

	fn write_footer_cell_line_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_cell_line_end(\
			row: {:?}, col: {:?}, line: {:?})",
			ctx.row, ctx.col, ctx.line)
	}
}
