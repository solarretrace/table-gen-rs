////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! A table renderer that prints called methods args.
////////////////////////////////////////////////////////////////////////////////

// Workspace library imports.
use table_gen_core::Features;
use table_gen_core::HorzAlign;
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
		row: usize,
		col: usize,
		line: usize,
		text: &str,
		width: usize,
		align: HorzAlign)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_cell_line(row: {}, col: {}, line: {}, \
				width: {}, align: {:?}, text: {:?})",
			row, col, line, width, align, text)
	}

	fn write_header_cell_line<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize,
		line: usize,
		text: &str,
		width: usize,
		align: HorzAlign)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_cell_line(row: {}, col: {}, line: {}, \
				width: {}, align: {:?}, text: {:?})",
			row, col, line, width, align, text)
	}

	fn write_footer_cell_line<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize,
		line: usize,
		text: &str,
		width: usize,
		align: HorzAlign)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_cell_line(row: {}, col: {}, line: {}, \
				width: {}, align: {:?}, text: {:?})",
			row, col, line, width, align, text)
	}

	// Section hooks
	///////////////////////////////////////////////
	fn write_table_start<W>(&mut self, out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_table_start()")
	}

	fn write_table_end<W>(&mut self, out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_table_end()")
	}

	fn write_header_start<W>(&mut self, out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_start()")
	}

	fn write_header_end<W>(&mut self, out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_end()")
	}

	fn write_data_start<W>(&mut self, out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_start()")
	}

	fn write_data_end<W>(&mut self, out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_end()")
	}

	fn write_footer_start<W>(&mut self, out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_start()")
	}

	fn write_footer_end<W>(&mut self, out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_end()")
	}

	// Row-based hooks
	////////////////////////////////////////////////////////////////////////////

	fn write_header_row_start<W>(&mut self, out: &mut W, row: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_row_start(row: {})", row)
	}

	fn write_header_row_end<W>(&mut self, out: &mut W, row: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_row_end(row: {})", row)
	}


	fn write_data_row_start<W>(&mut self, out: &mut W, row: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_row_start(row: {})", row)
	}

	fn write_data_row_end<W>(&mut self, out: &mut W, row: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_row_end(row: {})", row)
	}

	fn write_footer_row_start<W>(&mut self, out: &mut W, row: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_row_start(row: {})", row)
	}

	fn write_footer_row_end<W>(&mut self, out: &mut W, row: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_row_end(row: {})", row)
	}


	// Cell-based hooks
	////////////////////////////////////////////////////////////////////////////

	fn write_header_cell_start<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_cell_start(row: {}, col: {}",
			row, col)
	}

	fn write_header_cell_end<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_cell_end(row: {}, col: {}",
			row, col)
	}

	fn write_data_cell_start<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_cell_start(row: {}, col: {})",
			row, col)
	}

	fn write_data_cell_end<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_cell_end(row: {}, col: {})",
			row, col)
	}

	fn write_footer_cell_start<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_cell_start(row: {}, col: {}",
			row, col)
	}

	fn write_footer_cell_end<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_cell_end(row: {}, col: {}",
			row, col)
	}


	// Line-based hooks
	///////////////////////////////////////////////
	fn write_header_line_start<W>(
		&mut self,
		out: &mut W,
		row: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_line_start(row: {}, line: {})",
			row, line)
	}

	fn write_header_line_end<W>(
		&mut self,
		out: &mut W,
		row: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_line_end(row: {}, line: {})",
			row, line)
	}

	fn write_data_line_start<W>(
		&mut self,
		out: &mut W,
		row: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_line_start(row: {}, line: {})",
			row, line)
	}

	fn write_data_line_end<W>(
		&mut self,
		out: &mut W,
		row: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_line_end(row: {}, line: {})",
			row, line)
	}

	fn write_footer_line_start<W>(
		&mut self,
		out: &mut W,
		row: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_line_start(row: {}, line: {})",
			row, line)
	}

	fn write_footer_line_end<W>(
		&mut self,
		out: &mut W,
		row: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_line_end(row: {}, line: {})",
			row, line)
	}

	// Cell line-based hooks
	////////////////////////////////////////////////////////////////////////////

	fn write_header_cell_line_start<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_cell_line_start(row: {}, col: {}, line: {})",
			row, col, line)
	}

	fn write_header_cell_line_end<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_header_cell_line_end(row: {}, col: {}, line: {})",
			row, col, line)
	}

	fn write_data_cell_line_start<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_cell_line_start(row: {}, col: {}, line: {})",
			row, col, line)
	}

	fn write_data_cell_line_end<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_data_cell_line_end(row: {}, col: {}, line: {})",
			row, col, line)
	}

	fn write_footer_cell_line_start<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_cell_line_start(row: {}, col: {}, line: {})",
			row, col, line)
	}

	fn write_footer_cell_line_end<W>(
		&mut self,
		out: &mut W,
		row: usize,
		col: usize,
		line: usize)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		writeln!(out, "write_footer_cell_line_end(row: {}, col: {}, line: {})",
			row, col, line)
	}
}




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
write_table_start()
write_data_start()
write_data_end()
write_table_end()
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
write_table_start()
write_header_start()
write_header_row_start(row: 0)
write_header_line_start(row: 0, line: 0)
write_header_line_end(row: 0, line: 0)
write_header_row_end(row: 0)
write_header_end()
write_data_start()
write_data_end()
write_footer_start()
write_footer_row_start(row: 0)
write_footer_line_start(row: 0, line: 0)
write_footer_line_end(row: 0, line: 0)
write_footer_row_end(row: 0)
write_footer_end()
write_table_end()
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
write_table_start()
write_header_start()
write_header_row_start(row: 0)
write_header_line_start(row: 0, line: 0)
write_header_cell_start(row: 0, col: 0
write_header_cell_line_start(row: 0, col: 0, line: 0)
write_header_cell_line(row: 0, col: 0, line: 0, width: 6, align: Left, text: \"H0\")
write_header_cell_line_end(row: 0, col: 0, line: 0)
write_header_cell_end(row: 0, col: 0
write_header_cell_start(row: 0, col: 1
write_header_cell_line_start(row: 0, col: 1, line: 0)
write_header_cell_line(row: 0, col: 1, line: 0, width: 2, align: Left, text: \"H1\")
write_header_cell_line_end(row: 0, col: 1, line: 0)
write_header_cell_end(row: 0, col: 1
write_header_line_end(row: 0, line: 0)
write_header_row_end(row: 0)
write_header_end()
write_data_start()
write_data_row_start(row: 0)
write_data_line_start(row: 0, line: 0)
write_data_cell_start(row: 0, col: 0)
write_data_cell_line_start(row: 0, col: 0, line: 0)
write_data_cell_line(row: 0, col: 0, line: 0, width: 6, align: Left, text: \"-17\")
write_data_cell_line_end(row: 0, col: 0, line: 0)
write_data_cell_end(row: 0, col: 0)
write_data_cell_start(row: 0, col: 1)
write_data_cell_line_start(row: 0, col: 1, line: 0)
write_data_cell_line(row: 0, col: 1, line: 0, width: 2, align: Left, text: \"b\")
write_data_cell_line_end(row: 0, col: 1, line: 0)
write_data_cell_end(row: 0, col: 1)
write_data_line_end(row: 0, line: 0)
write_data_row_end(row: 0)
write_data_row_start(row: 1)
write_data_line_start(row: 1, line: 0)
write_data_cell_start(row: 1, col: 0)
write_data_cell_line_start(row: 1, col: 0, line: 0)
write_data_cell_line(row: 1, col: 0, line: 0, width: 6, align: Left, text: \"170000\")
write_data_cell_line_end(row: 1, col: 0, line: 0)
write_data_cell_end(row: 1, col: 0)
write_data_cell_start(row: 1, col: 1)
write_data_cell_line_start(row: 1, col: 1, line: 0)
write_data_cell_line(row: 1, col: 1, line: 0, width: 2, align: Left, text: \"&\")
write_data_cell_line_end(row: 1, col: 1, line: 0)
write_data_cell_end(row: 1, col: 1)
write_data_line_end(row: 1, line: 0)
write_data_row_end(row: 1)
write_data_end()
write_footer_start()
write_footer_row_start(row: 0)
write_footer_line_start(row: 0, line: 0)
write_footer_cell_start(row: 0, col: 0
write_footer_cell_line_start(row: 0, col: 0, line: 0)
write_footer_cell_line(row: 0, col: 0, line: 0, width: 6, align: Left, text: \"F0\")
write_footer_cell_line_end(row: 0, col: 0, line: 0)
write_footer_cell_end(row: 0, col: 0
write_footer_cell_start(row: 0, col: 1
write_footer_cell_line_start(row: 0, col: 1, line: 0)
write_footer_cell_line(row: 0, col: 1, line: 0, width: 2, align: Left, text: \"F1\")
write_footer_cell_line_end(row: 0, col: 1, line: 0)
write_footer_cell_end(row: 0, col: 1
write_footer_line_end(row: 0, line: 0)
write_footer_row_end(row: 0)
write_footer_end()
write_table_end()
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
write_table_start()
write_header_start()
write_header_row_start(row: 0)
write_header_line_start(row: 0, line: 0)
write_header_cell_start(row: 0, col: 0
write_header_cell_line_start(row: 0, col: 0, line: 0)
write_header_cell_line(row: 0, col: 0, line: 0, width: 6, align: Left, text: \"H0\")
write_header_cell_line_end(row: 0, col: 0, line: 0)
write_header_cell_end(row: 0, col: 0
write_header_cell_start(row: 0, col: 1
write_header_cell_line_start(row: 0, col: 1, line: 0)
write_header_cell_line(row: 0, col: 1, line: 0, width: 1, align: Left, text: \"\")
write_header_cell_line_end(row: 0, col: 1, line: 0)
write_header_cell_end(row: 0, col: 1
write_header_line_end(row: 0, line: 0)
write_header_row_end(row: 0)
write_header_end()
write_data_start()
write_data_row_start(row: 0)
write_data_line_start(row: 0, line: 0)
write_data_cell_start(row: 0, col: 0)
write_data_cell_line_start(row: 0, col: 0, line: 0)
write_data_cell_line(row: 0, col: 0, line: 0, width: 6, align: Left, text: \"-17\")
write_data_cell_line_end(row: 0, col: 0, line: 0)
write_data_cell_end(row: 0, col: 0)
write_data_cell_start(row: 0, col: 1)
write_data_cell_line_start(row: 0, col: 1, line: 0)
write_data_cell_line(row: 0, col: 1, line: 0, width: 1, align: Left, text: \"b\")
write_data_cell_line_end(row: 0, col: 1, line: 0)
write_data_cell_end(row: 0, col: 1)
write_data_line_end(row: 0, line: 0)
write_data_row_end(row: 0)
write_data_row_start(row: 1)
write_data_line_start(row: 1, line: 0)
write_data_cell_start(row: 1, col: 0)
write_data_cell_line_start(row: 1, col: 0, line: 0)
write_data_cell_line(row: 1, col: 0, line: 0, width: 6, align: Left, text: \"170000\")
write_data_cell_line_end(row: 1, col: 0, line: 0)
write_data_cell_end(row: 1, col: 0)
write_data_cell_start(row: 1, col: 1)
write_data_cell_line_start(row: 1, col: 1, line: 0)
write_data_cell_line(row: 1, col: 1, line: 0, width: 1, align: Left, text: \"&\")
write_data_cell_line_end(row: 1, col: 1, line: 0)
write_data_cell_end(row: 1, col: 1)
write_data_line_end(row: 1, line: 0)
write_data_row_end(row: 1)
write_data_end()
write_footer_start()
write_footer_row_start(row: 0)
write_footer_line_start(row: 0, line: 0)
write_footer_cell_start(row: 0, col: 0
write_footer_cell_line_start(row: 0, col: 0, line: 0)
write_footer_cell_line(row: 0, col: 0, line: 0, width: 6, align: Left, text: \"F0\")
write_footer_cell_line_end(row: 0, col: 0, line: 0)
write_footer_cell_end(row: 0, col: 0
write_footer_cell_start(row: 0, col: 1
write_footer_cell_line_start(row: 0, col: 1, line: 0)
write_footer_cell_line(row: 0, col: 1, line: 0, width: 1, align: Left, text: \"\")
write_footer_cell_line_end(row: 0, col: 1, line: 0)
write_footer_cell_end(row: 0, col: 1
write_footer_line_end(row: 0, line: 0)
write_footer_row_end(row: 0)
write_footer_end()
write_table_end()
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
write_table_start()
write_data_start()
write_data_row_start(row: 0)
write_data_line_start(row: 0, line: 0)
write_data_cell_start(row: 0, col: 0)
write_data_cell_line_start(row: 0, col: 0, line: 0)
write_data_cell_line(row: 0, col: 0, line: 0, width: 1, align: Left, text: \"1\")
write_data_cell_line_end(row: 0, col: 0, line: 0)
write_data_cell_end(row: 0, col: 0)
write_data_cell_start(row: 0, col: 1)
write_data_cell_line_start(row: 0, col: 1, line: 0)
write_data_cell_line(row: 0, col: 1, line: 0, width: 3, align: Left, text: \"100\")
write_data_cell_line_end(row: 0, col: 1, line: 0)
write_data_cell_end(row: 0, col: 1)
write_data_cell_start(row: 0, col: 2)
write_data_cell_line_start(row: 0, col: 2, line: 0)
write_data_cell_line(row: 0, col: 2, line: 0, width: 4, align: Left, text: \"1000\")
write_data_cell_line_end(row: 0, col: 2, line: 0)
write_data_cell_end(row: 0, col: 2)
write_data_line_end(row: 0, line: 0)
write_data_row_end(row: 0)
write_data_end()
write_table_end()
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
write_table_start()
write_header_start()
write_header_row_start(row: 0)
write_header_line_start(row: 0, line: 0)
write_header_cell_start(row: 0, col: 0
write_header_cell_line_start(row: 0, col: 0, line: 0)
write_header_cell_line(row: 0, col: 0, line: 0, width: 5, align: Left, text: \"<-0->\")
write_header_cell_line_end(row: 0, col: 0, line: 0)
write_header_cell_end(row: 0, col: 0
write_header_cell_start(row: 0, col: 1
write_header_cell_line_start(row: 0, col: 1, line: 0)
write_header_cell_line(row: 0, col: 1, line: 0, width: 5, align: Center, text: \"<-1->\")
write_header_cell_line_end(row: 0, col: 1, line: 0)
write_header_cell_end(row: 0, col: 1
write_header_line_end(row: 0, line: 0)
write_header_row_end(row: 0)
write_header_end()
write_data_start()
write_data_row_start(row: 0)
write_data_line_start(row: 0, line: 0)
write_data_cell_start(row: 0, col: 0)
write_data_cell_line_start(row: 0, col: 0, line: 0)
write_data_cell_line(row: 0, col: 0, line: 0, width: 5, align: Left, text: \"-3.70\")
write_data_cell_line_end(row: 0, col: 0, line: 0)
write_data_cell_end(row: 0, col: 0)
write_data_cell_start(row: 0, col: 1)
write_data_cell_line_start(row: 0, col: 1, line: 0)
write_data_cell_line(row: 0, col: 1, line: 0, width: 5, align: Center, text: \"d\")
write_data_cell_line_end(row: 0, col: 1, line: 0)
write_data_cell_end(row: 0, col: 1)
write_data_line_end(row: 0, line: 0)
write_data_row_end(row: 0)
write_data_row_start(row: 1)
write_data_line_start(row: 1, line: 0)
write_data_cell_start(row: 1, col: 0)
write_data_cell_line_start(row: 1, col: 0, line: 0)
write_data_cell_line(row: 1, col: 0, line: 0, width: 5, align: Left, text: \"+4.60\")
write_data_cell_line_end(row: 1, col: 0, line: 0)
write_data_cell_end(row: 1, col: 0)
write_data_cell_start(row: 1, col: 1)
write_data_cell_line_start(row: 1, col: 1, line: 0)
write_data_cell_line(row: 1, col: 1, line: 0, width: 5, align: Center, text: \"e\")
write_data_cell_line_end(row: 1, col: 1, line: 0)
write_data_cell_end(row: 1, col: 1)
write_data_line_end(row: 1, line: 0)
write_data_row_end(row: 1)
write_data_end()
write_table_end()
");
	}
}
