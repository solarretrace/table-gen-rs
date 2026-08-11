////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! A table renderer that renders tables in the pandoc-markdown 'grid' style.
////////////////////////////////////////////////////////////////////////////////

// Workspace library imports.
use table_gen::CellContext;
use table_gen::Features;
use table_gen::HorzAlign;
use table_gen::RenderContext;
use table_gen::Renderer;


// External library imports.
use bitflags::bitflags;


////////////////////////////////////////////////////////////////////////////////
// Flags
////////////////////////////////////////////////////////////////////////////////
bitflags! {
	/// Renderer feature flags.
	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	struct Flags: u8 {
		/// Whether alignment markers should be omitted.
		const ALIGN_MARKERS       = 0b_0000_0001;
		/// Whether pipes should used in the header divider separator.
		const HEADER_PIPES        = 0b_0000_0010;
		/// Indicates that the last column should be padded on the right.
		const PAD_TRAILING_COLUMN = 0b_0000_0100;

		/// All default flags set.
		const DEFAULT = Self::ALIGN_MARKERS.bits()
			| Self::HEADER_PIPES.bits()
			| Self::PAD_TRAILING_COLUMN.bits();
	}
}


////////////////////////////////////////////////////////////////////////////////
// MarkdownPipeRenderer
////////////////////////////////////////////////////////////////////////////////
/// A table renderer that renders tables in the pandoc-markdown 'grid' style.
#[derive(Debug, Clone)]
pub struct MarkdownPipeRenderer {
	/// The amount of space to allocate between columns.
	column_padding: u8,
	/// The amount of extra space to allocate within columns.
	extra_width: u8,
	/// Style flags.
	flags: Flags,
}

impl Default for MarkdownPipeRenderer {
	fn default() -> Self {
		Self::new()
	}
}

impl MarkdownPipeRenderer {
	/// Constructs a new `MarkdownPipeRenderer`.
	pub const fn new() -> Self {
		Self {
			column_padding: 0,
			extra_width: 0,
			flags: Flags::DEFAULT,
		}
	}

	/// Sets the column padding and returns the `MarkdownPipeRenderer`.
	pub const fn with_column_padding(mut self, column_padding: u8) -> Self {
		self.column_padding = column_padding;
		self
	}

	/// Sets the extra column width and returns the `MarkdownPipeRenderer`.
	pub const fn with_extra_width(mut self, extra_width: u8) -> Self {
		self.extra_width = extra_width;
		self
	}

	/// Sets the alignment markers usage flag and returns the
	/// `MarkdownPipeRenderer`.
	pub fn with_alignment_markers(mut self, align_markers: bool) -> Self {
		self.flags.set(Flags::ALIGN_MARKERS, align_markers);
		self
	}

	/// Sets the flag to use pipe symbols in the header divider separator and
	/// returns the `MarkdownPipeRenderer`.
	pub fn with_header_div_pipes(mut self, header_pipes: bool)
		-> Self
	{
		self.flags.set(Flags::HEADER_PIPES, header_pipes);
		self
	}

	/// Sets the flag for adding trailing column padding and returns the
	/// `MarkdownSimpleRenderer`.
	pub fn with_padded_trailing_column(
		mut self,
		pad_trailing_column: bool)
		-> Self
	{
		self.flags.set(Flags::PAD_TRAILING_COLUMN, pad_trailing_column);
		self
	}

	/// Renders an empty row.
	fn write_empty_row<W>(&self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_column_sep(out, HorzAlign::Left, "|", " ", " ", " ")?;
		for (col, col_width) in ctx.col_widths.iter().copied().enumerate() {
			if self.flags.contains(Flags::PAD_TRAILING_COLUMN) 
				|| col + 1 < ctx.column_count()
			{
				for _ in 0..col_width { write!(out, " ")?; }
				for _ in 0..self.extra_width { write!(out, " ")?; }
			} 
			if col + 1 == ctx.column_count() { break; }
			self.write_column_sep(out, HorzAlign::Center, "|", " ", " ", " ")?;
		}
		self.write_column_sep(out, HorzAlign::Right, "|", " ", " ", " ")?;
		Ok(())
	}

	/// Renders a column separator.
	fn write_column_sep<W>(
		&self,
		out: &mut W,
		bias: HorzAlign,
		center: &str,
		outer: &str,
		inner_left: &str,
		inner_right: &str)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		debug_assert!(inner_left.len() < 2);
		debug_assert_eq!(inner_left.len(), inner_right.len());
		let inner_pad = inner_left.len() as u8;
		let mut pad = std::cmp::max(self.column_padding / 2, inner_pad * 2) / 2;
		pad -= inner_pad;

		if bias != HorzAlign::Left {
			for _ in 0..pad { write!(out, "{}", outer)?; }
			write!(out, "{}", inner_left)?;
		}
		write!(out, "{}", center)?;
		if bias != HorzAlign::Right {
			write!(out, "{}", inner_right)?;
			for _ in 0..pad { write!(out, "{}", outer)?; }
		}
		Ok(())
	}
}

impl Renderer for MarkdownPipeRenderer {
	fn features(&self) -> Features {
		Features::empty()
	}

	fn write_data_cell_line<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>,
		cell: &CellContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		let mut pad = cell.padding();
		pad += self.extra_width as usize;
		let (l_pad, r_pad) = match cell.desc.horz_align {
			HorzAlign::Left   => (0,     pad),
			HorzAlign::Center => (pad/2, pad.div_ceil(2)),
			HorzAlign::Right  => (pad,   0),
		};
		
		for _ in 0..l_pad { write!(out, " ")?; }
		write!(out, "{}", cell.text)?;
		if self.flags.contains(Flags::PAD_TRAILING_COLUMN) 
			|| !ctx.is_last_column()
		{
			for _ in 0..r_pad { write!(out, " ")?; }
		}
		Ok(())
	}

	fn write_data_start<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_empty() { return Ok(()) }
		if ctx.is_headerless() {
			self.write_empty_row(out, ctx)?;
			writeln!(out)?;
		}

		// Define style markers.
		let align_markers = self.flags.contains(Flags::ALIGN_MARKERS);
		let header_pipes = self.flags.contains(Flags::HEADER_PIPES);
		let dm = "-"; // divider marker
		let am = if align_markers { ":" } else { dm }; // align marker
		let pm = "|"; // pipe marker
		let im = if header_pipes { pm } else { "+" }; // internal div marker

		use HorzAlign::*;
		for (col, col_width) in ctx.col_widths.iter().copied().enumerate() {
			let cur_align = ctx.col_descs.get(col).map(|d| d.horz_align);
			if col == 0 {
				let r = if matches!(cur_align, Some(Right|Center)) {
					am
				} else {
					dm
				};
				self.write_column_sep(out, Left, pm, dm, " ", r)?;
			}
			for _ in 0..col_width { write!(out, "{}", dm)?; }
			for _ in 0..self.extra_width { write!(out, "{}", dm)?; }

			let (l, r) = match (
				cur_align,
				ctx.col_descs.get(col + 1).map(|d| d.horz_align))
			{
				(Some(Right|Center), Some(Left|Center)) => (am, am),
				(Some(Right|Center), _)                 => (am, dm),
				(_,                  Some(Left|Center)) => (dm, am),
				_                                       => (dm, dm),
			};
			if col + 1 == ctx.column_count() { 
				self.write_column_sep(out, Right, pm, dm, l, " ")?;
				break;
			}
			self.write_column_sep(out, Center, im, dm, l, r)?;
		}
		writeln!(out)
	}

	fn write_data_cell_line_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		let pm = "|"; // pipe marker
		if ctx.is_first_column() {
			self.write_column_sep(out, HorzAlign::Left, pm, " ", " ", " ")?;
		}
		Ok(())
	}

	fn write_data_cell_line_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		let pm = "|"; // pipe marker
		if ctx.is_last_column() {
			self.write_column_sep(out, HorzAlign::Right, pm, " ", " ", " ")
		} else {
			self.write_column_sep(out, HorzAlign::Center, pm, " ", " ", " ")
		}
	}
}


////////////////////////////////////////////////////////////////////////////////
// Test module
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test {
	use super::*;
	use table_gen::ColumnDesc;
	use table_gen::ColumnOrd;
	use table_gen::HorzAlign;
	use table_gen::Table;

	#[test]
	fn empty_table() {
		let data: Vec<(usize, )> = vec![];

		let mut table = Table::new_builder(data, MarkdownPipeRenderer::new())
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

		let order = [ColumnOrd::new(0).reverse()];
		let mut table = Table::new_builder(data, MarkdownPipeRenderer::new())
			.with_sort_columns(&order)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
|       |
|-------|
| 30000 |
| 2000  |
| 100   |
| 10    |
| 1     |
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

		let col_descs = vec![
			ColumnDesc::new()
				.with_header("Right")
				.with_horz_align(HorzAlign::Right),
			ColumnDesc::new()
				.with_header("Left")
				.with_horz_align(HorzAlign::Left),
			ColumnDesc::new()
				.with_header("Center")
				.with_horz_align(HorzAlign::Center),
		];

		let mut table = Table::new_builder(data, MarkdownPipeRenderer::new())
			.with_column_descs(&col_descs)
			.with_column_selection(&[0, 0, 0])
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
| Right | Left  | Center |
|:-----:|:------|:------:|
|    12 | 12    |   12   |
|   123 | 123   |  123   |
|     1 | 1     |   1    |
| -8000 | -8000 | -8000  |
");
	}

	#[test]
	fn simple_table_alt() {
		let data: Vec<(i64, )> = vec![
			(12,),
			(123,),
			(1,),
			(-8000,),
		];

		let col_descs = vec![
			ColumnDesc::new()
				.with_header("Right")
				.with_horz_align(HorzAlign::Right),
			ColumnDesc::new()
				.with_header("Left")
				.with_horz_align(HorzAlign::Left),
			ColumnDesc::new()
				.with_header("Center")
				.with_horz_align(HorzAlign::Center),
		];

		let mut table = Table::new_builder(data, MarkdownPipeRenderer::new()
				.with_alignment_markers(false)
				.with_header_div_pipes(false))
			.with_column_descs(&col_descs)
			.with_column_selection(&[0, 0, 0])
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
| Right | Left  | Center |
|-------+-------+--------|
|    12 | 12    |   12   |
|   123 | 123   |  123   |
|     1 | 1     |   1    |
| -8000 | -8000 | -8000  |
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

		let col_descs = vec![
			ColumnDesc::new()
				.with_horz_align(HorzAlign::Right),
			ColumnDesc::new()
				.with_horz_align(HorzAlign::Left),
			ColumnDesc::new()
				.with_horz_align(HorzAlign::Center),
		];

		let mut table = Table::new_builder(data, MarkdownPipeRenderer::new())
			.with_column_descs(&col_descs)
			.with_column_selection(&[0, 0, 0])
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
|       |       |       |
|:-----:|:------|:-----:|
|    12 | 12    |  12   |
|   123 | 123   |  123  |
|     1 | 1     |   1   |
| -8000 | -8000 | -8000 |
");
	}

	#[test]
	fn multiline_table() {
		let data: Vec<(&str, &str, f64, &str)> = vec![
			("First", "row",
				12.0, "Example of a row that\nspans multiple lines."),
			("Second", "row",
				5.0, "Here's another one. Note\nthe blank line between\nrows"),
		];

		let col_descs = vec![
			ColumnDesc::new()
				.with_header("Centered\nHeader")
				.with_horz_align(HorzAlign::Center),
			ColumnDesc::new()
				.with_header("Left\nAligned")
				.with_horz_align(HorzAlign::Left),
			ColumnDesc::new()
				.with_header("Right\nAligned")
				.with_horz_align(HorzAlign::Right),
			ColumnDesc::new()
				.with_header("Left\nAligned")
				.with_horz_align(HorzAlign::Left),
		];

		let mut table = Table::new_builder(data, MarkdownPipeRenderer::new())
			.with_column_descs(&col_descs)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
| Centered | Left    |   Right | Left                                                 |
|  Header  | Aligned | Aligned | Aligned                                              |
|:--------:|:--------|--------:|:-----------------------------------------------------|
|  First   | row     |      12 | Example of a row that spans multiple lines.          |
|  Second  | row     |       5 | Here's another one. Note the blank line between rows |
");
	}

	#[test]
	fn simple_table_alt_unpad_trailing() {
		let data: Vec<(i64, )> = vec![
			(12,),
			(123,),
			(1,),
			(-8000,),
		];

		let col_descs = vec![
			ColumnDesc::new()
				.with_header("Right")
				.with_horz_align(HorzAlign::Right),
			ColumnDesc::new()
				.with_header("Left")
				.with_horz_align(HorzAlign::Left),
			ColumnDesc::new()
				.with_header("Center")
				.with_horz_align(HorzAlign::Center),
		];

		let mut table = Table::new_builder(data, MarkdownPipeRenderer::new()
				.with_padded_trailing_column(false)
				.with_alignment_markers(false)
				.with_header_div_pipes(false))
			.with_column_descs(&col_descs)
			.with_column_selection(&[0, 0, 0])
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
| Right | Left  | Center |
|-------+-------+--------|
|    12 | 12    |   12 |
|   123 | 123   |  123 |
|     1 | 1     |   1 |
| -8000 | -8000 | -8000 |
");
	}

	#[test]
	fn simple_table_no_headers_unpad_trailing() {
		let data: Vec<(i64, )> = vec![
			(12,),
			(123,),
			(1,),
			(-8000,),
		];

		let col_descs = vec![
			ColumnDesc::new()
				.with_horz_align(HorzAlign::Right),
			ColumnDesc::new()
				.with_horz_align(HorzAlign::Left),
			ColumnDesc::new()
				.with_horz_align(HorzAlign::Center),
		];

		let mut table = Table::new_builder(data, MarkdownPipeRenderer::new()
				.with_padded_trailing_column(false))
			.with_column_descs(&col_descs)
			.with_column_selection(&[0, 0, 0])
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
|       |       |  |
|:-----:|:------|:-----:|
|    12 | 12    |  12 |
|   123 | 123   |  123 |
|     1 | 1     |   1 |
| -8000 | -8000 | -8000 |
");
	}

	#[test]
	fn multiline_table_unpad_trailing() {
		let data: Vec<(&str, &str, f64, &str)> = vec![
			("First", "row",
				12.0, "Example of a row that\nspans multiple lines."),
			("Second", "row",
				5.0, "Here's another one. Note\nthe blank line between\nrows"),
		];

		let col_descs = vec![
			ColumnDesc::new()
				.with_header("Centered\nHeader")
				.with_horz_align(HorzAlign::Center),
			ColumnDesc::new()
				.with_header("Left\nAligned")
				.with_horz_align(HorzAlign::Left),
			ColumnDesc::new()
				.with_header("Right\nAligned")
				.with_horz_align(HorzAlign::Right),
			ColumnDesc::new()
				.with_header("Left\nAligned")
				.with_horz_align(HorzAlign::Left),
		];

		let mut table = Table::new_builder(data, MarkdownPipeRenderer::new()
				.with_padded_trailing_column(false))
			.with_column_descs(&col_descs)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
| Centered | Left    |   Right | Left |
|  Header  | Aligned | Aligned | Aligned |
|:--------:|:--------|--------:|:-----------------------------------------------------|
|  First   | row     |      12 | Example of a row that spans multiple lines. |
|  Second  | row     |       5 | Here's another one. Note the blank line between rows |
");
	}
}
