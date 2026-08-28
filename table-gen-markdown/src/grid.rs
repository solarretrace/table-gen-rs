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
use table_gen::util::fill;
use table_gen::util::WrapOptions;
use table_gen::util::write_cell_formatted;

// Standard library imports.
use std::rc::Rc;


////////////////////////////////////////////////////////////////////////////////
// MarkdownGridRenderer
////////////////////////////////////////////////////////////////////////////////
/// A table renderer that renders tables in the pandoc-markdown 'grid' style.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct MarkdownGridRenderer {
	/// The amount of space to allocate between columns.
	column_padding: u8,
	/// Indicates that text wrapping should be used.
	wrap_options: Option<WrapOptions>,
	/// Indicates which columns the wrapping should be applied to.
	wrap_columns: Option<Vec<usize>>,
}

impl Default for MarkdownGridRenderer {
	fn default() -> Self {
		Self::new()
	}
}

impl MarkdownGridRenderer {
	/// Constructs a new `MarkdownGridRenderer`.
	#[must_use]
	pub fn new() -> Self {
		Self {
			column_padding: 1,
			wrap_options: Some(WrapOptions::new()),
			wrap_columns: None,
		}
	}

	/// Sets the column padding and returns the `MarkdownGridRenderer`.
	#[must_use]
	pub fn with_column_padding(mut self, column_padding: u8) -> Self {
		self.column_padding = std::cmp::max(column_padding, 1);
		self
	}

	/// Sets the text wrap options and returns the `MarkdownGridRenderer`.
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

	/// Sets the text wrap columns and returns the `MarkdownGridRenderer`.
	///
	/// A `None` value will apply the wrapping to all columns.
	#[must_use]
	pub fn with_wrap_columns<O>(mut self, wrap_columns: O) -> Self 
		where O: Into<Option<Vec<usize>>>
	{
		self.wrap_columns = wrap_columns.into();
		self
	}

	/// Renders a row seperating line.
	fn write_row_sep<W>(&self, out: &mut W, ctx: &RenderContext<'_>, line: &str)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_column_sep(out, HorzAlign::Left, "+", line)?;
		for (col, col_width) in ctx.col_widths.iter().copied().enumerate() {
			for _ in 0..col_width { write!(out, "{}", line)?; }
			if col + 1 == ctx.column_count() { break; }
			self.write_column_sep(out, HorzAlign::Center, "+", line)?;
		}
		self.write_column_sep(out, HorzAlign::Right, "+", line)?;
		Ok(())
	}

	/// Renders a column separator.
	fn write_column_sep<W>(
		&self,
		out: &mut W,
		bias: HorzAlign,
		center: &str,
		outer: &str)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		let pad = self.column_padding;

		if bias != HorzAlign::Left {
			for _ in 0..pad { write!(out, "{}", outer)?; }
		}
		write!(out, "{}", center)?;
		if bias != HorzAlign::Right {
			for _ in 0..pad { write!(out, "{}", outer)?; }
		}
		Ok(())
	}
}

impl Renderer for MarkdownGridRenderer {
	fn features(&self) -> Features {
		let padding: usize = self.column_padding.into();
		let mut features = Features::default()
			.with_width_contribution_fn(Box::new(move |col_count| {
				// Width of dividers
				col_count + 1
				// Width of cell padding
				+ (col_count * padding * 2)
			}));
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
		self.write_row_sep(out, ctx, "-")?;
		writeln!(out)
	}

	fn write_data_cell_line<W>(
		&mut self,
		out: &mut W,
		_ctx: &RenderContext<'_>,
		cell: &CellContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		let Some(text_width) = cell.text_width else {
			return write!(out, "{}", cell.text);
		};
		write_cell_formatted(
			out,
			cell.text,
			text_width,
			cell.cell_width,
			cell.desc.horz_align,
			"…")
	}

	fn write_data_start<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_empty() { return Ok(()) }
		let c = if ctx.is_headerless() { "-" } else {"=" };
		self.write_row_sep(out, ctx, c)?;
		writeln!(out)
	}

	fn write_data_cell_line_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_first_column() {
			self.write_column_sep(out, HorzAlign::Left, "|", " ")?;
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
		let align = if ctx.is_last_column() {
			HorzAlign::Right
		} else {
			HorzAlign::Center
		};

		self.write_column_sep(out, align, "|", " ")
	}

	fn write_data_row_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if !ctx.is_first_row() {
			self.write_row_sep(out, ctx, "-")?;
			writeln!(out)?;
		}
		Ok(())
	}

	fn write_data_end<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_headerless() {
			self.write_data_start(out, ctx)
		} else {
			self.write_header_start(out, ctx)
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
	use table_gen::Table;

	#[test]
	fn empty_table() {
		let data: Vec<(usize, )> = vec![];

		let mut table = Table::new_builder(data, MarkdownGridRenderer::new())
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "");
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

		let mut table = Table::new_builder(data, MarkdownGridRenderer::new())
			.with_column_defs(&column_defs)
			.with_column_selection(&[0, 0, 0])
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
+-------+-------+--------+
| Right | Left  | Center |
+=======+=======+========+
|    12 | 12    |   12   |
+-------+-------+--------+
|   123 | 123   |  123   |
+-------+-------+--------+
|     1 | 1     |   1    |
+-------+-------+--------+
| -8000 | -8000 | -8000  |
+-------+-------+--------+
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

		let mut table = Table::new_builder(data, MarkdownGridRenderer::new())
			.with_column_defs(&column_defs)
			.with_column_selection(&[0, 0, 0])
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
+-------+-------+-------+
|    12 | 12    |  12   |
+-------+-------+-------+
|   123 | 123   |  123  |
+-------+-------+-------+
|     1 | 1     |   1   |
+-------+-------+-------+
| -8000 | -8000 | -8000 |
+-------+-------+-------+
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

		let mut table = Table::new_builder(data, MarkdownGridRenderer::new())
			.with_column_defs(&column_defs)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
+----------+---------+---------+--------------------------+
| Centered | Left    |   Right | Left                     |
|  Header  | Aligned | Aligned | Aligned                  |
+==========+=========+=========+==========================+
|  First   | row     |      12 | Example of a row that    |
|          |         |         | spans multiple lines.    |
+----------+---------+---------+--------------------------+
|  Second  | row     |       5 | Here's another one. Note |
|          |         |         | the blank line between   |
|          |         |         | rows                     |
+----------+---------+---------+--------------------------+
");
	}

	#[test]
	fn expanded_width_table() {
		let data: Vec<[usize; 8]> = vec![
			[2, 3, 4, 5, 6, 7, 8, 9],
			[22, 43, 54, 85, 96, 907, 8, 19],
			[23, 35, 46, 58, 69, 709, 8, 19],
		];

		let mut table = Table::new_builder(data, MarkdownGridRenderer::new())
			.with_min_table_width(80)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
+---------+---------+---------+---------+---------+----------+--------+--------+
| 2       | 3       | 4       | 5       | 6       | 7        | 8      | 9      |
+---------+---------+---------+---------+---------+----------+--------+--------+
| 22      | 43      | 54      | 85      | 96      | 907      | 8      | 19     |
+---------+---------+---------+---------+---------+----------+--------+--------+
| 23      | 35      | 46      | 58      | 69      | 709      | 8      | 19     |
+---------+---------+---------+---------+---------+----------+--------+--------+
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

		let mut table = Table::new_builder(data, MarkdownGridRenderer::new()
				.with_wrap_options(None))
			.with_column_defs(&column_defs)
			.with_max_table_width(80)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
+---------+------------------+----------+--------------------------------------+
| COLUMN… | COLUMN B         | COLUMN C | COLUMN D                             |
+=========+==================+==========+======================================+
| Sed ut… | unde omnis iste… | sit vol… | doloremque laudantium, totam rem ap… |
+---------+------------------+----------+--------------------------------------+
| eaque … | ab illo invento… | quasi a… | vitae dicta sunt explicabo. Nemo en… |
+---------+------------------+----------+--------------------------------------+
| volupt… | sit aspernatur … | fugit, … | quia consequuntur magni dolores eos… |
+---------+------------------+----------+--------------------------------------+
| volupt… | Neque porro qui… | qui dol… | quia dolor sit amet, consectetur, a… |
+---------+------------------+----------+--------------------------------------+
| velit,… | quia non numqua… | tempora… | labore et dolore magnam aliquam qua… |
+---------+------------------+----------+--------------------------------------+
| enim a… | veniam, quis no… | ullam c… | laboriosam, nisi ut aliquid ex ea    |
+---------+------------------+----------+--------------------------------------+
| commod… | Quis autem vel … | reprehe… | ea voluptate velit esse quam nihil … |
+---------+------------------+----------+--------------------------------------+
| conseq… | illum qui dolor… | quo vol… | pariatur?                            |
+---------+------------------+----------+--------------------------------------+
| COLUMN… | COLUMN B         | COLUMN C | COLUMN D                             |
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

		let mut table = Table::new_builder(data, MarkdownGridRenderer::new()
				.with_wrap_options(WrapOptions::new()
					.with_break_words(false)))
			.with_column_defs(&column_defs)
			.with_max_table_width(80)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
+---------+------------------+----------+--------------------------------------+
| COLUMN… | COLUMN B         | COLUMN C | COLUMN D                             |
+=========+==================+==========+======================================+
| Sed ut  | unde omnis iste  | sit      | doloremque laudantium, totam rem     |
| perspi… | natus error      | volupta… | aperiam,                             |
|         |                  | accusan… |                                      |
+---------+------------------+----------+--------------------------------------+
| eaque   | ab illo          | quasi    | vitae dicta sunt explicabo. Nemo     |
| ipsa    | inventore        | archite… | enim ipsam                           |
| quae    | veritatis et     | beatae   |                                      |
+---------+------------------+----------+--------------------------------------+
| volupt… | sit aspernatur   | fugit,   | quia consequuntur magni dolores eos  |
| quia    | aut odit aut     | sed      | qui ratione                          |
| volupt… |                  |          |                                      |
+---------+------------------+----------+--------------------------------------+
| volupt… | Neque porro      | qui      | quia dolor sit amet, consectetur,    |
| sequi   | quisquam est,    | dolorem  | adipisci                             |
| nesciu… |                  | ipsum    |                                      |
+---------+------------------+----------+--------------------------------------+
| velit,  | quia non numquam | tempora  | labore et dolore magnam aliquam      |
| sed     | eius modi        | incidunt | quaerat voluptatem. Ut               |
|         |                  | ut       |                                      |
+---------+------------------+----------+--------------------------------------+
| enim ad | veniam,          | ullam    | laboriosam, nisi ut aliquid ex ea    |
| minima  | quis nostrum     | corporis |                                      |
|         | exercitationem   | suscipit |                                      |
+---------+------------------+----------+--------------------------------------+
| commodi | Quis autem vel   | reprehe… | ea voluptate velit esse quam nihil   |
| conseq… | eum iure         | qui in   | molestiae                            |
+---------+------------------+----------+--------------------------------------+
| conseq… | illum qui        | quo      | pariatur?                            |
| vel     | dolorem eum      | voluptas |                                      |
|         | fugiat           | nulla    |                                      |
+---------+------------------+----------+--------------------------------------+
| COLUMN… | COLUMN B         | COLUMN C | COLUMN D                             |
");
	}
}
