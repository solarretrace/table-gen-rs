////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! A table renderer that prints called methods args.
////////////////////////////////////////////////////////////////////////////////

// Workspace library imports.
use table_gen_core::CellContext;
use table_gen_core::Features;
use table_gen_core::RenderContext;
use table_gen_core::Renderer;


////////////////////////////////////////////////////////////////////////////////
// DebugRenderer
////////////////////////////////////////////////////////////////////////////////
/// A table renderer that prints called methods args.
#[derive(Debug, Clone, Copy, Default)]
pub struct DebugRenderer;

impl Renderer for DebugRenderer {
	fn features(&self) -> Features {
		Features::all()
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


////////////////////////////////////////////////////////////////////////////////
// Test module
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test {
	use super::*;
	use table_gen_core::Table;
	use table_gen_core::ColumnDesc;
	use table_gen_core::VertAlign;
	use table_gen_core::HorzAlign;
	use table_gen_core::DisplayFmt;
	use table_gen_core::Sign;

	#[test]
	fn empty_table() {
		let data: Vec<[usize; 0]> = vec![];

		let mut table = Table::new_builder(data, DebugRenderer)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
write_table_start(row: None, col: None, line: None)
write_data_start(row: None, col: None, line: None)
write_data_end(row: None, col: None, line: None)
write_table_end(row: None, col: None, line: None)
");
	}

	#[test]
	fn empty_table_header_footer() {
		let data: Vec<[usize; 2]> = vec![];

		let specs = vec![
			ColumnDesc::new()
				.with_header("H0")
				.with_footer("F0"),
			ColumnDesc::new()
				.with_header("H1")
				.with_footer("F1"),
		];
		let mut table = Table::new_builder(data, DebugRenderer)
			.with_column_descs(&specs)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
write_table_start(row: None, col: None, line: None)
write_header_start(row: None, col: None, line: None)
write_header_row_start(row: None, col: None, line: None)
write_header_line_start(row: None, col: None, line: Some(0))
write_header_line_end(row: None, col: None, line: Some(0))
write_header_row_end(row: None, col: None, line: None)
write_header_end(row: None, col: None, line: None)
write_data_start(row: None, col: None, line: None)
write_data_end(row: None, col: None, line: None)
write_footer_start(row: None, col: None, line: None)
write_footer_row_start(row: None, col: None, line: None)
write_footer_line_start(row: None, col: None, line: Some(0))
write_footer_line_end(row: None, col: None, line: Some(0))
write_footer_row_end(row: None, col: None, line: None)
write_footer_end(row: None, col: None, line: None)
write_table_end(row: None, col: None, line: None)
");
	}

	#[test]
	fn tuple_table_two_rows() {
		let data: Vec<(i32, char)> = vec![
			(-17, 'b'),
			(170000, '&'),
		];

		let specs = vec![
			ColumnDesc::new()
				.with_header("H0")
				.with_footer("F0"),
			ColumnDesc::new()
				.with_header("H1")
				.with_footer("F1"),
		];
		let mut table = Table::new_builder(data, DebugRenderer)
			.with_column_descs(&specs)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
write_table_start(row: None, col: None, line: None)
write_header_start(row: None, col: None, line: None)
write_header_row_start(row: None, col: None, line: None)
write_header_line_start(row: None, col: None, line: Some(0))
write_header_cell_start(row: None, col: Some(0), line: Some(0))
write_header_cell_line_start(row: None, col: Some(0), line: Some(0))
write_data_cell_line(row: None, col: Some(0), line: Some(0), CellContext { text: \"H0\", width: 6, desc: ColumnDesc { header: \"H0\", footer: \"F0\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_header_cell_line_end(row: None, col: Some(0), line: Some(0))
write_header_cell_end(row: None, col: Some(0), line: Some(0))
write_header_cell_start(row: None, col: Some(1), line: Some(0))
write_header_cell_line_start(row: None, col: Some(1), line: Some(0))
write_data_cell_line(row: None, col: Some(1), line: Some(0), CellContext { text: \"H1\", width: 2, desc: ColumnDesc { header: \"H1\", footer: \"F1\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_header_cell_line_end(row: None, col: Some(1), line: Some(0))
write_header_cell_end(row: None, col: Some(1), line: Some(0))
write_header_line_end(row: None, col: None, line: Some(0))
write_header_row_end(row: None, col: None, line: None)
write_header_end(row: None, col: None, line: None)
write_data_start(row: None, col: None, line: None)
write_data_row_start(row: Some(0), col: None, line: None)
write_data_line_start(row: Some(0), col: None, line: Some(0))
write_data_cell_start(row: Some(0), col: Some(0), line: Some(0))
write_data_cell_line_start(row: Some(0), col: Some(0), line: Some(0))
write_data_cell_line(row: Some(0), col: Some(0), line: Some(0), CellContext { text: \"-17\", width: 6, desc: ColumnDesc { header: \"H0\", footer: \"F0\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_data_cell_line_end(row: Some(0), col: Some(0), line: Some(0))
write_data_cell_end(row: Some(0), col: Some(0), line: Some(0))
write_data_cell_start(row: Some(0), col: Some(1), line: Some(0))
write_data_cell_line_start(row: Some(0), col: Some(1), line: Some(0))
write_data_cell_line(row: Some(0), col: Some(1), line: Some(0), CellContext { text: \"b\", width: 2, desc: ColumnDesc { header: \"H1\", footer: \"F1\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_data_cell_line_end(row: Some(0), col: Some(1), line: Some(0))
write_data_cell_end(row: Some(0), col: Some(1), line: Some(0))
write_data_line_end(row: Some(0), col: None, line: Some(0))
write_data_row_end(row: Some(0), col: None, line: None)
write_data_row_start(row: Some(1), col: None, line: None)
write_data_line_start(row: Some(1), col: None, line: Some(0))
write_data_cell_start(row: Some(1), col: Some(0), line: Some(0))
write_data_cell_line_start(row: Some(1), col: Some(0), line: Some(0))
write_data_cell_line(row: Some(1), col: Some(0), line: Some(0), CellContext { text: \"170000\", width: 6, desc: ColumnDesc { header: \"H0\", footer: \"F0\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_data_cell_line_end(row: Some(1), col: Some(0), line: Some(0))
write_data_cell_end(row: Some(1), col: Some(0), line: Some(0))
write_data_cell_start(row: Some(1), col: Some(1), line: Some(0))
write_data_cell_line_start(row: Some(1), col: Some(1), line: Some(0))
write_data_cell_line(row: Some(1), col: Some(1), line: Some(0), CellContext { text: \"&\", width: 2, desc: ColumnDesc { header: \"H1\", footer: \"F1\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_data_cell_line_end(row: Some(1), col: Some(1), line: Some(0))
write_data_cell_end(row: Some(1), col: Some(1), line: Some(0))
write_data_line_end(row: Some(1), col: None, line: Some(0))
write_data_row_end(row: Some(1), col: None, line: None)
write_data_end(row: Some(1), col: None, line: None)
write_footer_start(row: Some(1), col: None, line: None)
write_footer_row_start(row: Some(1), col: None, line: None)
write_footer_line_start(row: Some(1), col: None, line: Some(0))
write_footer_cell_start(row: Some(1), col: Some(0), line: Some(0))
write_footer_cell_line_start(row: Some(1), col: Some(0), line: Some(0))
write_data_cell_line(row: Some(1), col: Some(0), line: Some(0), CellContext { text: \"F0\", width: 6, desc: ColumnDesc { header: \"H0\", footer: \"F0\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_footer_cell_line_end(row: Some(1), col: Some(0), line: Some(0))
write_footer_cell_end(row: Some(1), col: Some(0), line: Some(0))
write_footer_cell_start(row: Some(1), col: Some(1), line: Some(0))
write_footer_cell_line_start(row: Some(1), col: Some(1), line: Some(0))
write_data_cell_line(row: Some(1), col: Some(1), line: Some(0), CellContext { text: \"F1\", width: 2, desc: ColumnDesc { header: \"H1\", footer: \"F1\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_footer_cell_line_end(row: Some(1), col: Some(1), line: Some(0))
write_footer_cell_end(row: Some(1), col: Some(1), line: Some(0))
write_footer_line_end(row: Some(1), col: None, line: Some(0))
write_footer_row_end(row: Some(1), col: None, line: None)
write_footer_end(row: Some(1), col: None, line: None)
write_table_end(row: Some(1), col: None, line: None)
");
	}

	#[test]
	fn tuple_table_two_rows_short_column_descs() {
		let data: Vec<(i32, char)> = vec![
			(-17, 'b'),
			(170000, '&'),
		];

		let specs = vec![
			ColumnDesc::new()
				.with_header("H0")
				.with_footer("F0"),
		];
		let mut table = Table::new_builder(data, DebugRenderer)
			.with_column_descs(&specs)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
write_table_start(row: None, col: None, line: None)
write_header_start(row: None, col: None, line: None)
write_header_row_start(row: None, col: None, line: None)
write_header_line_start(row: None, col: None, line: Some(0))
write_header_cell_start(row: None, col: Some(0), line: Some(0))
write_header_cell_line_start(row: None, col: Some(0), line: Some(0))
write_data_cell_line(row: None, col: Some(0), line: Some(0), CellContext { text: \"H0\", width: 6, desc: ColumnDesc { header: \"H0\", footer: \"F0\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_header_cell_line_end(row: None, col: Some(0), line: Some(0))
write_header_cell_end(row: None, col: Some(0), line: Some(0))
write_header_cell_start(row: None, col: Some(1), line: Some(0))
write_header_cell_line_start(row: None, col: Some(1), line: Some(0))
write_data_cell_line(row: None, col: Some(1), line: Some(0), CellContext { text: \"\", width: 1, desc: ColumnDesc { header: \"\", footer: \"\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_header_cell_line_end(row: None, col: Some(1), line: Some(0))
write_header_cell_end(row: None, col: Some(1), line: Some(0))
write_header_line_end(row: None, col: None, line: Some(0))
write_header_row_end(row: None, col: None, line: None)
write_header_end(row: None, col: None, line: None)
write_data_start(row: None, col: None, line: None)
write_data_row_start(row: Some(0), col: None, line: None)
write_data_line_start(row: Some(0), col: None, line: Some(0))
write_data_cell_start(row: Some(0), col: Some(0), line: Some(0))
write_data_cell_line_start(row: Some(0), col: Some(0), line: Some(0))
write_data_cell_line(row: Some(0), col: Some(0), line: Some(0), CellContext { text: \"-17\", width: 6, desc: ColumnDesc { header: \"H0\", footer: \"F0\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_data_cell_line_end(row: Some(0), col: Some(0), line: Some(0))
write_data_cell_end(row: Some(0), col: Some(0), line: Some(0))
write_data_cell_start(row: Some(0), col: Some(1), line: Some(0))
write_data_cell_line_start(row: Some(0), col: Some(1), line: Some(0))
write_data_cell_line(row: Some(0), col: Some(1), line: Some(0), CellContext { text: \"b\", width: 1, desc: ColumnDesc { header: \"\", footer: \"\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_data_cell_line_end(row: Some(0), col: Some(1), line: Some(0))
write_data_cell_end(row: Some(0), col: Some(1), line: Some(0))
write_data_line_end(row: Some(0), col: None, line: Some(0))
write_data_row_end(row: Some(0), col: None, line: None)
write_data_row_start(row: Some(1), col: None, line: None)
write_data_line_start(row: Some(1), col: None, line: Some(0))
write_data_cell_start(row: Some(1), col: Some(0), line: Some(0))
write_data_cell_line_start(row: Some(1), col: Some(0), line: Some(0))
write_data_cell_line(row: Some(1), col: Some(0), line: Some(0), CellContext { text: \"170000\", width: 6, desc: ColumnDesc { header: \"H0\", footer: \"F0\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_data_cell_line_end(row: Some(1), col: Some(0), line: Some(0))
write_data_cell_end(row: Some(1), col: Some(0), line: Some(0))
write_data_cell_start(row: Some(1), col: Some(1), line: Some(0))
write_data_cell_line_start(row: Some(1), col: Some(1), line: Some(0))
write_data_cell_line(row: Some(1), col: Some(1), line: Some(0), CellContext { text: \"&\", width: 1, desc: ColumnDesc { header: \"\", footer: \"\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_data_cell_line_end(row: Some(1), col: Some(1), line: Some(0))
write_data_cell_end(row: Some(1), col: Some(1), line: Some(0))
write_data_line_end(row: Some(1), col: None, line: Some(0))
write_data_row_end(row: Some(1), col: None, line: None)
write_data_end(row: Some(1), col: None, line: None)
write_footer_start(row: Some(1), col: None, line: None)
write_footer_row_start(row: Some(1), col: None, line: None)
write_footer_line_start(row: Some(1), col: None, line: Some(0))
write_footer_cell_start(row: Some(1), col: Some(0), line: Some(0))
write_footer_cell_line_start(row: Some(1), col: Some(0), line: Some(0))
write_data_cell_line(row: Some(1), col: Some(0), line: Some(0), CellContext { text: \"F0\", width: 6, desc: ColumnDesc { header: \"H0\", footer: \"F0\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_footer_cell_line_end(row: Some(1), col: Some(0), line: Some(0))
write_footer_cell_end(row: Some(1), col: Some(0), line: Some(0))
write_footer_cell_start(row: Some(1), col: Some(1), line: Some(0))
write_footer_cell_line_start(row: Some(1), col: Some(1), line: Some(0))
write_data_cell_line(row: Some(1), col: Some(1), line: Some(0), CellContext { text: \"\", width: 1, desc: ColumnDesc { header: \"\", footer: \"\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_footer_cell_line_end(row: Some(1), col: Some(1), line: Some(0))
write_footer_cell_end(row: Some(1), col: Some(1), line: Some(0))
write_footer_line_end(row: Some(1), col: None, line: Some(0))
write_footer_row_end(row: Some(1), col: None, line: None)
write_footer_end(row: Some(1), col: None, line: None)
write_table_end(row: Some(1), col: None, line: None)
");
	}

	#[test]
	fn array_table_single_row() {
		let data: Vec<[usize; 4]> = vec![
			[1, 10, 100, 1000],
		];

		let mut table = Table::new_builder(data, DebugRenderer)
			.with_column_selection(&[0, 2, 3])
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
write_table_start(row: None, col: None, line: None)
write_data_start(row: None, col: None, line: None)
write_data_row_start(row: Some(0), col: None, line: None)
write_data_line_start(row: Some(0), col: None, line: Some(0))
write_data_cell_start(row: Some(0), col: Some(0), line: Some(0))
write_data_cell_line_start(row: Some(0), col: Some(0), line: Some(0))
write_data_cell_line(row: Some(0), col: Some(0), line: Some(0), CellContext { text: \"1\", width: 1, desc: ColumnDesc { header: \"\", footer: \"\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_data_cell_line_end(row: Some(0), col: Some(0), line: Some(0))
write_data_cell_end(row: Some(0), col: Some(0), line: Some(0))
write_data_cell_start(row: Some(0), col: Some(1), line: Some(0))
write_data_cell_line_start(row: Some(0), col: Some(1), line: Some(0))
write_data_cell_line(row: Some(0), col: Some(1), line: Some(0), CellContext { text: \"100\", width: 3, desc: ColumnDesc { header: \"\", footer: \"\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_data_cell_line_end(row: Some(0), col: Some(1), line: Some(0))
write_data_cell_end(row: Some(0), col: Some(1), line: Some(0))
write_data_cell_start(row: Some(0), col: Some(2), line: Some(0))
write_data_cell_line_start(row: Some(0), col: Some(2), line: Some(0))
write_data_cell_line(row: Some(0), col: Some(2), line: Some(0), CellContext { text: \"1000\", width: 4, desc: ColumnDesc { header: \"\", footer: \"\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_data_cell_line_end(row: Some(0), col: Some(2), line: Some(0))
write_data_cell_end(row: Some(0), col: Some(2), line: Some(0))
write_data_line_end(row: Some(0), col: None, line: Some(0))
write_data_row_end(row: Some(0), col: None, line: None)
write_data_end(row: Some(0), col: None, line: None)
write_table_end(row: Some(0), col: None, line: None)
");
	}

	#[test]
	fn tuple_table_subselect_float() {
		let data: Vec<(f32, char)> = vec![
			( 0.0, 'a'),
			(-1.9, 'b'),
			( 2.8, 'c'),
			(-3.7, 'd'),
			( 4.6, 'e'),
			(-5.5, 'f'),
			( 6.4, 'g'),
			(-7.3, 'h'),
			( 8.2, 'i'),
			(-9.1, 'j'),
		];

		let col_descs = vec![
			ColumnDesc::new()
				.with_header("<-0->")
				.with_display_fmt(DisplayFmt::new()
					.with_precision(2)
					.with_sign(Sign::Plus))
				.with_horz_align(HorzAlign::Left)
				.with_vert_align(VertAlign::Top),
			ColumnDesc::new()
				.with_header("<-1->")
				.with_horz_align(HorzAlign::Center)
				.with_vert_align(VertAlign::Center),
		];

		let mut table = Table::new_builder(data, DebugRenderer)
			.with_row_selection(3..5)
			.with_column_descs(&col_descs)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
write_table_start(row: None, col: None, line: None)
write_header_start(row: None, col: None, line: None)
write_header_row_start(row: None, col: None, line: None)
write_header_line_start(row: None, col: None, line: Some(0))
write_header_cell_start(row: None, col: Some(0), line: Some(0))
write_header_cell_line_start(row: None, col: Some(0), line: Some(0))
write_data_cell_line(row: None, col: Some(0), line: Some(0), CellContext { text: \"<-0->\", width: 5, desc: ColumnDesc { header: \"<-0->\", footer: \"\", display_fmt: DisplayFmt { precision: Some(2), sign: Some(Plus) }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_header_cell_line_end(row: None, col: Some(0), line: Some(0))
write_header_cell_end(row: None, col: Some(0), line: Some(0))
write_header_cell_start(row: None, col: Some(1), line: Some(0))
write_header_cell_line_start(row: None, col: Some(1), line: Some(0))
write_data_cell_line(row: None, col: Some(1), line: Some(0), CellContext { text: \"<-1->\", width: 5, desc: ColumnDesc { header: \"<-1->\", footer: \"\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Center, vert_align: Center } })
write_header_cell_line_end(row: None, col: Some(1), line: Some(0))
write_header_cell_end(row: None, col: Some(1), line: Some(0))
write_header_line_end(row: None, col: None, line: Some(0))
write_header_row_end(row: None, col: None, line: None)
write_header_end(row: None, col: None, line: None)
write_data_start(row: None, col: None, line: None)
write_data_row_start(row: Some(0), col: None, line: None)
write_data_line_start(row: Some(0), col: None, line: Some(0))
write_data_cell_start(row: Some(0), col: Some(0), line: Some(0))
write_data_cell_line_start(row: Some(0), col: Some(0), line: Some(0))
write_data_cell_line(row: Some(0), col: Some(0), line: Some(0), CellContext { text: \"-3.70\", width: 5, desc: ColumnDesc { header: \"<-0->\", footer: \"\", display_fmt: DisplayFmt { precision: Some(2), sign: Some(Plus) }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_data_cell_line_end(row: Some(0), col: Some(0), line: Some(0))
write_data_cell_end(row: Some(0), col: Some(0), line: Some(0))
write_data_cell_start(row: Some(0), col: Some(1), line: Some(0))
write_data_cell_line_start(row: Some(0), col: Some(1), line: Some(0))
write_data_cell_line(row: Some(0), col: Some(1), line: Some(0), CellContext { text: \"d\", width: 5, desc: ColumnDesc { header: \"<-1->\", footer: \"\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Center, vert_align: Center } })
write_data_cell_line_end(row: Some(0), col: Some(1), line: Some(0))
write_data_cell_end(row: Some(0), col: Some(1), line: Some(0))
write_data_line_end(row: Some(0), col: None, line: Some(0))
write_data_row_end(row: Some(0), col: None, line: None)
write_data_row_start(row: Some(1), col: None, line: None)
write_data_line_start(row: Some(1), col: None, line: Some(0))
write_data_cell_start(row: Some(1), col: Some(0), line: Some(0))
write_data_cell_line_start(row: Some(1), col: Some(0), line: Some(0))
write_data_cell_line(row: Some(1), col: Some(0), line: Some(0), CellContext { text: \"+4.60\", width: 5, desc: ColumnDesc { header: \"<-0->\", footer: \"\", display_fmt: DisplayFmt { precision: Some(2), sign: Some(Plus) }, min_width: 0, max_width: 18446744073709551615, horz_align: Left, vert_align: Top } })
write_data_cell_line_end(row: Some(1), col: Some(0), line: Some(0))
write_data_cell_end(row: Some(1), col: Some(0), line: Some(0))
write_data_cell_start(row: Some(1), col: Some(1), line: Some(0))
write_data_cell_line_start(row: Some(1), col: Some(1), line: Some(0))
write_data_cell_line(row: Some(1), col: Some(1), line: Some(0), CellContext { text: \"e\", width: 5, desc: ColumnDesc { header: \"<-1->\", footer: \"\", display_fmt: DisplayFmt { precision: None, sign: None }, min_width: 0, max_width: 18446744073709551615, horz_align: Center, vert_align: Center } })
write_data_cell_line_end(row: Some(1), col: Some(1), line: Some(0))
write_data_cell_end(row: Some(1), col: Some(1), line: Some(0))
write_data_line_end(row: Some(1), col: None, line: Some(0))
write_data_row_end(row: Some(1), col: None, line: None)
write_data_end(row: Some(1), col: None, line: None)
write_table_end(row: Some(1), col: None, line: None)
");
	}
}
