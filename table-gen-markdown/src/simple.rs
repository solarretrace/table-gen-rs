////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! A table renderer that renders tables in the pandoc-markdown 'simple' style.
////////////////////////////////////////////////////////////////////////////////

// Workspace library imports.
use table_gen::CellContext;
use table_gen::Features;
use table_gen::HorzAlign;
use table_gen::RenderContext;
use table_gen::Renderer;


////////////////////////////////////////////////////////////////////////////////
// MarkdownSimpleRenderer
////////////////////////////////////////////////////////////////////////////////
/// A table renderer that renders tables in the pandoc-markdown 'simple' style.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct MarkdownSimpleRenderer {
	/// The amount of space to allocate between columns.
	column_padding: u8,
	/// The amount of extra space to allocate within columns.
	extra_column_width: u8,
	/// Indicates that the last column should be padded on the right.
	pad_trailing_column: bool,
}

impl Default for MarkdownSimpleRenderer {
	fn default() -> Self {
		Self::new()
	}
}

impl MarkdownSimpleRenderer {
	/// Constructs a new `MarkdownSimpleRenderer`.
	#[must_use]
	pub fn new() -> Self {
		Self {
			column_padding: 0,
			extra_column_width: 2,
			pad_trailing_column: true,
		}
	}

	/// Sets the column padding and returns the `MarkdownSimpleRenderer`.
	#[must_use]
	pub fn with_column_padding(mut self, column_padding: u8) -> Self {
		self.column_padding = column_padding;
		self
	}

	/// Sets the extra column width and returns the `MarkdownSimpleRenderer`.
	#[must_use]
	pub fn with_extra_column_width(mut self, extra_column_width: u8) -> Self {
		self.extra_column_width = extra_column_width;
		self
	}

	/// Sets the flag for adding trailing column padding and returns the
	/// `MarkdownSimpleRenderer`.
	#[must_use]
	pub fn with_padded_trailing_column(
		mut self,
		pad_trailing_column: bool)
		-> Self
	{
		self.pad_trailing_column = pad_trailing_column;
		self
	}
}

impl Renderer for MarkdownSimpleRenderer {
	fn features(&self) -> Features {
		Features::default()
			.with_extra_column_width(self.extra_column_width.into())
	}

	fn write_data_cell_line<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>,
		cell: &CellContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		let pad = cell.padding();
		let (l_pad, r_pad) = match cell.desc.horz_align {
			HorzAlign::Left   => (0,     pad),
			HorzAlign::Center => (pad/2, pad.div_ceil(2)),
			HorzAlign::Right  => (pad,   0),
		};
		
		for _ in 0..l_pad { write!(out, " ")?; }
		write!(out, "{}", cell.text)?;
		if self.pad_trailing_column || !ctx.is_last_column() {
			for _ in 0..r_pad { write!(out, " ")?; }
		}
		Ok(())
	}

	fn write_header_cell_line_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_last_column() { return Ok(()); }
		for _ in 0..self.column_padding { write!(out, " ")?; }
		write!(out, " ")
	}

	fn write_data_start<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_empty() { return Ok(()) }
		for (col, col_width) in ctx.col_widths.iter().copied().enumerate() {
			for _ in 0..col_width { write!(out, "-")?; }
			if col + 1 == ctx.column_count() { break; }
			for _ in 0..self.column_padding { write!(out, " ")?; }
			write!(out, " ")?;
		}
		writeln!(out)
	}

	fn write_data_cell_line_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_last_column() { return Ok(()); }
		for _ in 0..self.column_padding { write!(out, " ")?; }
		write!(out, " ")
	}

	fn write_data_end<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_headerless() {
			self.write_data_start(out, ctx)?;
		}
		Ok(())
	}
}


////////////////////////////////////////////////////////////////////////////////
// Test module
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test {
	use super::*;
	use table_gen::ColumnDef;
	use table_gen::ColumnOrd;
	use table_gen::Table;

	#[test]
	fn empty_table() {
		let data: Vec<(usize, )> = vec![];

		let mut table = Table::new_builder(data, MarkdownSimpleRenderer::new())
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "");
	}

	#[test]
	fn array_table_single_column() {
		let data: Vec<[usize; 1]> = vec![
			[1],
			[2000],
			[10],
			[30000],
			[100],
		];

		let order = [ColumnOrd::new(0).with_reversed_order()];
		let mut table = Table::new_builder(data, MarkdownSimpleRenderer::new())
			.with_sort_columns(&order)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
-------
30000  
2000   
100    
10     
1      
-------
");
	}

	#[test]
	fn simple_table() {
		let data: Vec<(i64, )> = vec![
			(12,),
			(123,),
			(1,),
			(-8000,),
		];

		let column_defs = vec![
			ColumnDef::new()
				.with_header("Right")
				.with_horz_align(HorzAlign::Right),
			ColumnDef::new()
				.with_header("Left")
				.with_horz_align(HorzAlign::Left),
			ColumnDef::new()
				.with_header("Center")
				.with_horz_align(HorzAlign::Center),
		];

		let mut table = Table::new_builder(data, MarkdownSimpleRenderer::new())
			.with_column_defs(&column_defs)
			.with_column_selection(&[0, 0, 0])
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "  \
  Right Left     Center 
------- ------- --------
     12 12         12   
    123 123       123   
      1 1          1    
  -8000 -8000    -8000  
");
	}

	#[test]
	fn simple_table_no_headers() {
		let data: Vec<(i64, )> = vec![
			(12,),
			(123,),
			(1,),
			(-8000,),
		];

		let column_defs = vec![
			ColumnDef::new()
				.with_horz_align(HorzAlign::Right),
			ColumnDef::new()
				.with_horz_align(HorzAlign::Left),
			ColumnDef::new()
				.with_horz_align(HorzAlign::Center),
		];

		let mut table = Table::new_builder(data, MarkdownSimpleRenderer::new())
			.with_column_defs(&column_defs)
			.with_column_selection(&[0, 0, 0])
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
------- ------- -------
     12 12        12   
    123 123       123  
      1 1          1   
  -8000 -8000    -8000 
------- ------- -------
");
	}

	#[test]
	fn simple_table_unpad_trailing() {
		let data: Vec<(i64, )> = vec![
			(12,),
			(123,),
			(1,),
			(-8000,),
		];

		let column_defs = vec![
			ColumnDef::new()
				.with_header("Right")
				.with_horz_align(HorzAlign::Right),
			ColumnDef::new()
				.with_header("Left")
				.with_horz_align(HorzAlign::Left),
			ColumnDef::new()
				.with_header("Center")
				.with_horz_align(HorzAlign::Center),
		];

		let mut table = Table::new_builder(data, MarkdownSimpleRenderer::new()
				.with_padded_trailing_column(false))
			.with_column_defs(&column_defs)
			.with_column_selection(&[0, 0, 0])
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "  \
  Right Left     Center
------- ------- --------
     12 12         12
    123 123       123
      1 1          1
  -8000 -8000    -8000
");
	}
}
