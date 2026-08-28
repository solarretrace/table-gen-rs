////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! A table renderer that renders tables in the pandoc-markdown 'multiline'
//! style.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::TrailingWsTrimWriter;

// Workspace library imports.
use table_gen::CellContext;
use table_gen::Features;
use table_gen::HorzAlign;
use table_gen::RenderContext;
use table_gen::Renderer;
use table_gen::util::fill;
use table_gen::util::WrapOptions;
use table_gen::util::write_cell_formatted;

// Standard library imports.
use std::rc::Rc;


////////////////////////////////////////////////////////////////////////////////
// MarkdownMultilineRenderer
////////////////////////////////////////////////////////////////////////////////
/// A table renderer that renders tables in the pandoc-markdown 'multiline'
/// style.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct MarkdownMultilineRenderer {
	/// The amount of space to allocate between columns.
	column_padding: u8,
	/// The amount of extra space to allocate within columns.
	extra_column_width: u8,
	/// Indicates that the last column should be padded on the right.
	padded_trailing_column: bool,
	/// Indicates that text wrapping should be used.
	wrap_options: Option<WrapOptions>,
	/// Indicates which columns the wrapping should be applied to.
	wrap_columns: Option<Vec<usize>>,
}

impl Default for MarkdownMultilineRenderer {
	fn default() -> Self {
		Self::new()
	}
}

impl MarkdownMultilineRenderer {
	/// Constructs a new `MarkdownMultilineRenderer`.
	#[must_use]
	pub fn new() -> Self {
		Self {
			column_padding: 0,
			extra_column_width: 2,
			padded_trailing_column: true,
			wrap_options: Some(WrapOptions::new()),
			wrap_columns: None,
		}
	}

	/// Sets the column padding and returns the `MarkdownMultilineRenderer`.
	#[must_use]
	pub fn with_column_padding(mut self, column_padding: u8) -> Self {
		self.column_padding = column_padding;
		self
	}

	/// Sets the extra column width and returns the `MarkdownMultilineRenderer`.
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
		padded_trailing_column: bool)
		-> Self
	{
		self.padded_trailing_column = padded_trailing_column;
		self
	}

	/// Sets the text wrap options and returns the `MarkdownMultilineRenderer`.
	///
	/// Note: this method overrides any value previously specified with
	/// `with_late_format_fn`.
	#[must_use]
	pub fn with_wrap_options<O>(mut self, wrap_options: O) -> Self 
		where O: Into<Option<WrapOptions>>
	{
		self.wrap_options = wrap_options.into();
		self
	}

	/// Sets the text wrap columns and returns the `MarkdownMultilineRenderer`.
	///
	/// A `None` value will apply the wrapping to all columns.
	#[must_use]
	pub fn with_wrap_columns<O>(mut self, wrap_columns: O) -> Self 
		where O: Into<Option<Vec<usize>>>
	{
		self.wrap_columns = wrap_columns.into();
		self
	}
}

impl Renderer for MarkdownMultilineRenderer {
	fn features(&self) -> Features {
		let mut features = Features::default()
			.with_extra_column_width(self.extra_column_width.into());
		if let Some(wrap_options) = self.wrap_options.clone() {
			let wrap_columns = self.wrap_columns.clone();
			features = features
				.with_late_format_fn(Rc::new(move |s, idx, w| if wrap_columns
					.as_ref()
					.map(|v| v.contains(&idx))
					.unwrap_or(true)
				{
					fill(s, wrap_options.as_options(w))
				} else {
					s.to_string()
				}))
		}
		features
	}

	fn write_header_start<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_empty() { return Ok(()) }
		for (col, col_width) in ctx.col_widths.iter().copied().enumerate() {
			for _ in 0..col_width { write!(out, "-")?; }
			if col + 1 == ctx.column_count() { break; }
			for _ in 0..self.column_padding { write!(out, "-")?; }
			write!(out, "-")?;
		}
		writeln!(out)
	}

	fn write_data_cell_line<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>,
		cell: &CellContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		let Some(text_width) = cell.text_width else {
			return write!(out, "{}", cell.text);
		};
		if ctx.is_last_column() 
			&& cell.desc.horz_align != HorzAlign::Right
			&& !self.padded_trailing_column
		{
			// We need to prevent writing trailing whitespace on the last
			// column.
			let mut writer = TrailingWsTrimWriter::new(out);
			write_cell_formatted(
				&mut writer,
				cell.text,
				text_width,
				cell.cell_width,
				cell.desc.horz_align,
				"…")?;
			// Discard pending whitespace writes, since they will write if we
			// drop the writer without writing a newline.
			writer.clear_pending();
			Ok(())
		} else {
			write_cell_formatted(
				out,
				cell.text,
				text_width,
				cell.cell_width,
				cell.desc.horz_align,
				"…")
		}
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

	fn write_data_start<W>(&mut self, out: &mut W,ctx: &RenderContext<'_>)
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

	fn write_data_row_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if !ctx.is_first_row() { writeln!(out)?; }
		Ok(())
	}

	fn write_data_end<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_headerless() {
			self.write_data_start(out, ctx)
		} else {
			self.write_header_start(out, ctx)?;
			writeln!(out)
		}
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

		let mut table = Table::new_builder(data, MarkdownMultilineRenderer::new())
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
		let mut table = Table::new_builder(data, MarkdownMultilineRenderer::new())
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

		let mut table = Table::new_builder(data, MarkdownMultilineRenderer::new())
			.with_column_defs(&column_defs)
			.with_column_selection(&[0, 0, 0])
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
------------------------
  Right Left     Center 
------- ------- --------
     12 12         12   

    123 123       123   

      1 1          1    

  -8000 -8000    -8000  
------------------------

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

		let mut table = Table::new_builder(data, MarkdownMultilineRenderer::new())
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
	fn multiline_table() {
		let data: Vec<(&str, &str, f64, &str)> = vec![
			("First", "row",
				12.0, "Example of a row that\nspans multiple lines."),
			("Second", "row",
				5.0, "Here's another one. Note\nthe blank line between\nrows"),
		];

		let column_defs = vec![
			ColumnDef::new()
				.with_header("Centered\nHeader")
				.with_horz_align(HorzAlign::Center),
			ColumnDef::new()
				.with_header("Left\nAligned")
				.with_horz_align(HorzAlign::Left),
			ColumnDef::new()
				.with_header("Right\nAligned")
				.with_horz_align(HorzAlign::Right),
			ColumnDef::new()
				.with_header("Left\nAligned")
				.with_horz_align(HorzAlign::Left),
		];

		let mut table = Table::new_builder(data, MarkdownMultilineRenderer::new())
			.with_column_defs(&column_defs)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
---------------------------------------------------------
 Centered  Left          Right Left                      
  Header   Aligned     Aligned Aligned                   
---------- --------- --------- --------------------------
  First    row              12 Example of a row that     
                               spans multiple lines.     

  Second   row               5 Here's another one. Note  
                               the blank line between    
                               rows                      
---------------------------------------------------------

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

		let column_defs = vec![
			ColumnDef::new()
				.with_horz_align(HorzAlign::Right),
			ColumnDef::new()
				.with_horz_align(HorzAlign::Left),
			ColumnDef::new()
				.with_horz_align(HorzAlign::Center),
		];

		let mut table = Table::new_builder(data, MarkdownMultilineRenderer::new()
				.with_padded_trailing_column(false))
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
	fn multiline_table_unpad_trailing() {
		let data: Vec<(&str, &str, f64, &str)> = vec![
			("First", "row",
				12.0, "Example of a row that\nspans multiple lines."),
			("Second", "row",
				5.0, "Here's another one. Note\nthe blank line between\nrows"),
		];

		let column_defs = vec![
			ColumnDef::new()
				.with_header("Centered\nHeader")
				.with_horz_align(HorzAlign::Center),
			ColumnDef::new()
				.with_header("Left\nAligned")
				.with_horz_align(HorzAlign::Left),
			ColumnDef::new()
				.with_header("Right\nAligned")
				.with_horz_align(HorzAlign::Right),
			ColumnDef::new()
				.with_header("Left\nAligned")
				.with_horz_align(HorzAlign::Left),
		];

		let mut table = Table::new_builder(data, MarkdownMultilineRenderer::new()
				.with_padded_trailing_column(false))
			.with_column_defs(&column_defs)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
---------------------------------------------------------
 Centered  Left          Right Left
  Header   Aligned     Aligned Aligned
---------- --------- --------- --------------------------
  First    row              12 Example of a row that
                               spans multiple lines.

  Second   row               5 Here's another one. Note
                               the blank line between
                               rows
---------------------------------------------------------

");
	}

	#[test]
	fn expanded_width_table() {
		let data: Vec<[usize; 8]> = vec![
			[2, 3, 4, 5, 6, 7, 8, 9],
			[22, 43, 54, 85, 96, 907, 8, 19],
			[23, 35, 46, 58, 69, 709, 8, 19],
		];

		let mut table = Table::new_builder(data, MarkdownMultilineRenderer::new())
			.with_min_table_width(80)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
--------- --------- --------- --------- --------- ---------- --------- ---------
2         3         4         5         6         7          8         9        

22        43        54        85        96        907        8         19       

23        35        46        58        69        709        8         19       
--------- --------- --------- --------- --------- ---------- --------- ---------
");
	}

	#[test]
	fn cicero() {
		let data: Vec<[&str; 4]> = vec![
			[
				"Sed ut perspiciatis",
				"unde omnis iste natus error",
				"sit voluptatem accusantium",
				"doloremque laudantium, totam rem aperiam,",
			],
			[
				"eaque ipsa quae",
				"ab illo inventore veritatis et",
				"quasi architecto beatae",
				"vitae dicta sunt explicabo. Nemo enim ipsam",
			],
			[
				"voluptatem quia voluptas",
				"sit aspernatur aut odit aut",
				"fugit, sed",
				"quia consequuntur magni dolores eos qui ratione",
			],
			[
				"voluptatem sequi nesciunt",
				"Neque porro quisquam est,",
				"qui dolorem ipsum",
				"quia dolor sit amet, consectetur, adipisci",
			],
			[
				"velit, sed",
				"quia non numquam eius modi",
				"tempora incidunt ut",
				"labore et dolore magnam aliquam quaerat voluptatem. Ut",
			],
			[
				"enim ad minima",
				"veniam, quis nostrum exercitationem",
				"ullam corporis suscipit",
				"laboriosam, nisi ut aliquid ex ea",
			],
			[
				"commodi consequatur?",
				"Quis autem vel eum iure",
				"reprehenderit qui in",
				"ea voluptate velit esse quam nihil molestiae",
			],
			[
				"consequatur, vel",
				"illum qui dolorem eum fugiat",
				"quo voluptas nulla",
				"pariatur?",
			],
		];
		
		let column_defs = vec![
			ColumnDef::new()
				.with_header("COLUMN A")
				.with_footer("COLUMN A"),
			ColumnDef::new()
				.with_header("COLUMN B")
				.with_footer("COLUMN B"),
			ColumnDef::new()
				.with_header("COLUMN C")
				.with_footer("COLUMN C"),
			ColumnDef::new()
				.with_header("COLUMN D")
				.with_footer("COLUMN D"),
		];

		let mut table = Table::new_builder(data, MarkdownMultilineRenderer::new()
				.with_wrap_options(None))
			.with_column_defs(&column_defs)
			.with_max_table_width(80)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
--------------------------------------------------------------------------------
COLUMN A  COLUMN B            COLUMN C   COLUMN D                               
--------- ------------------- ---------- ---------------------------------------
Sed ut p… unde omnis iste na… sit volup… doloremque laudantium, totam rem aperi…

eaque ip… ab illo inventore … quasi arc… vitae dicta sunt explicabo. Nemo enim …

voluptat… sit aspernatur aut… fugit, sed quia consequuntur magni dolores eos qu…

voluptat… Neque porro quisqu… qui dolor… quia dolor sit amet, consectetur, adip…

velit, s… quia non numquam e… tempora i… labore et dolore magnam aliquam quaera…

enim ad … veniam, quis nostr… ullam cor… laboriosam, nisi ut aliquid ex ea      

commodi … Quis autem vel eum… reprehend… ea voluptate velit esse quam nihil mol…

consequa… illum qui dolorem … quo volup… pariatur?                              
--------------------------------------------------------------------------------

COLUMN A  COLUMN B            COLUMN C   COLUMN D                               
");
	}

	#[test]
	fn cicero_wrap() {
		let data: Vec<[&str; 4]> = vec![
			[
				"Sed ut perspiciatis",
				"unde omnis iste natus error",
				"sit voluptatem accusantium",
				"doloremque laudantium, totam rem aperiam,",
			],
			[
				"eaque ipsa quae",
				"ab illo inventore veritatis et",
				"quasi architecto beatae",
				"vitae dicta sunt explicabo. Nemo enim ipsam",
			],
			[
				"voluptatem quia voluptas",
				"sit aspernatur aut odit aut",
				"fugit, sed",
				"quia consequuntur magni dolores eos qui ratione",
			],
			[
				"voluptatem sequi nesciunt",
				"Neque porro quisquam est,",
				"qui dolorem ipsum",
				"quia dolor sit amet, consectetur, adipisci",
			],
			[
				"velit, sed",
				"quia non numquam eius modi",
				"tempora incidunt ut",
				"labore et dolore magnam aliquam quaerat voluptatem. Ut",
			],
			[
				"enim ad minima",
				"veniam, quis nostrum exercitationem",
				"ullam corporis suscipit",
				"laboriosam, nisi ut aliquid ex ea",
			],
			[
				"commodi consequatur?",
				"Quis autem vel eum iure",
				"reprehenderit qui in",
				"ea voluptate velit esse quam nihil molestiae",
			],
			[
				"consequatur, vel",
				"illum qui dolorem eum fugiat",
				"quo voluptas nulla",
				"pariatur?",
			],
		];
		
		let column_defs = vec![
			ColumnDef::new()
				.with_header("COLUMN A")
				.with_footer("COLUMN A"),
			ColumnDef::new()
				.with_header("COLUMN B")
				.with_footer("COLUMN B"),
			ColumnDef::new()
				.with_header("COLUMN C")
				.with_footer("COLUMN C"),
			ColumnDef::new()
				.with_header("COLUMN D")
				.with_footer("COLUMN D"),
		];

		let mut table = Table::new_builder(data, MarkdownMultilineRenderer::new()
				.with_padded_trailing_column(false))
			.with_column_defs(&column_defs)
			.with_max_table_width(80)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
--------------------------------------------------------------------------------
COLUMN A  COLUMN B            COLUMN C   COLUMN D
--------- ------------------- ---------- ---------------------------------------
Sed ut    unde omnis iste     sit        doloremque laudantium, totam rem
perspici… natus error         voluptatem aperiam,
                              accusanti… 

eaque     ab illo inventore   quasi      vitae dicta sunt explicabo. Nemo enim
ipsa quae veritatis et        architecto ipsam
                              beatae     

voluptat… sit aspernatur aut  fugit, sed quia consequuntur magni dolores eos
quia      odit aut                       qui ratione
voluptas                                 

voluptat… Neque porro         qui        quia dolor sit amet, consectetur,
sequi     quisquam est,       dolorem    adipisci
nesciunt                      ipsum      

velit,    quia non numquam    tempora    labore et dolore magnam aliquam quaerat
sed       eius modi           incidunt   voluptatem. Ut
                              ut         

enim ad   veniam,             ullam      laboriosam, nisi ut aliquid ex ea
minima    quis nostrum        corporis   
          exercitationem      suscipit   

commodi   Quis autem vel      reprehend… ea voluptate velit esse quam nihil
consequa… eum iure            qui in     molestiae

consequa… illum qui dolorem   quo        pariatur?
vel       eum fugiat          voluptas   
                              nulla      
--------------------------------------------------------------------------------

COLUMN A  COLUMN B            COLUMN C   COLUMN D
");
	}
}
