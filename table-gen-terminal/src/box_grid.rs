////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! A table renderer that renders tables using box-drawing unicode style with
//! grid-like dividers between each cell.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::LineShape;
use crate::LineStyle;
use crate::Style;

// Workspace library imports.
use table_gen::CellContext;
use table_gen::Features;
use table_gen::RenderContext;
use table_gen::Renderer;
use table_gen::util::fill;
use table_gen::util::WrapOptions;
use table_gen::util::write_cell_formatted;

// Standard library imports.
use std::fmt::Display;
use std::rc::Rc;


////////////////////////////////////////////////////////////////////////////////
// BoxGridStyle
////////////////////////////////////////////////////////////////////////////////
/// The style specification for the box renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoxGridStyle {
	/// The style to use for the left table border.
	pub border_left: LineStyle,
	/// The style to use for the right table border.
	pub border_right: LineStyle,
	/// The style to use for the top table border.
	pub border_top: LineStyle,
	/// The style to use for the bottom table border.
	pub border_bottom: LineStyle,
	/// The style to use for the row separators.
	pub row_sep: LineStyle,
	/// The style to use for the column separators.
	pub col_sep: LineStyle,
	/// The style to use for the section divide separators.
	pub div_sep: LineStyle,
	/// Whether to use rounded corner variants.
	pub round_corners: bool,
}

impl Default for BoxGridStyle {
	fn default() -> Self {
		Self::new()
	}
}

impl BoxGridStyle {
	/// Constructs a new `BoxGridStyle` with the default styling.
	#[must_use]
	pub fn new() -> Self {
		Self {
			border_left: LineShape::Light.into(),
			border_right: LineShape::Light.into(),
			border_top: LineShape::Light.into(),
			border_bottom: LineShape::Light.into(),
			row_sep: LineShape::Light.into(),
			col_sep: LineShape::LightDash3.into(),
			div_sep: LineShape::Double.into(),
			round_corners: false,
		}
	}

    /// Sets the round corners flag for the borders and returns the
    /// `BoxGridStyle`.
    #[must_use]
    pub fn with_round_corners(mut self, round_corners: bool) -> Self {
        self.round_corners = round_corners;
        self
    }

	/// Sets the `LineShape` for the outer borders and returns the
	/// `BoxGridStyle`.
	#[must_use]
	pub fn with_borders_shape(mut self, line_shape: LineShape) -> Self {
		self.border_left.shape = line_shape;
		self.border_right.shape = line_shape;
		self.border_top.shape = line_shape;
		self.border_bottom.shape = line_shape;
		self
	}

	/// Sets the `Style` for the outer borders and returns the `BoxGridStyle`.
	#[must_use]
	pub fn with_borders_style(mut self, style: Style) -> Self {
		self.border_left.style = style;
		self.border_right.style = style;
		self.border_top.style = style;
		self.border_bottom.style = style;
		self
	}

	/// Sets the `LineShape` for the inner separators and returns the
	/// `BoxGridStyle`.
	#[must_use]
	pub fn with_separators_shape(mut self, line_shape: LineShape) -> Self {
		self.row_sep.shape = line_shape;
		self.col_sep.shape = line_shape;
		self.div_sep.shape = line_shape;
		self
	}

	/// Sets the `Style` for the inner separators and returns the
	/// `BoxGridStyle`.
	#[must_use]
	pub fn with_separators_style(mut self, style: Style) -> Self {
		self.row_sep.style = style;
		self.col_sep.style = style;
		self.div_sep.style = style;
		self
	}
}

////////////////////////////////////////////////////////////////////////////////
// BoxGridRenderer
////////////////////////////////////////////////////////////////////////////////
/// A table renderer that renders tables using box-drawing unicode style.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct BoxGridRenderer {
	/// The amount of space to allocate on each side of column dividers.
	column_padding: u8,
	/// The amount of extra space to allocate within columns.
	extra_column_width: u8,
	/// The `BoxGridStyle` to render with.
	style: BoxGridStyle,
	/// Indicates that text wrapping should be used.
	wrap_options: Option<WrapOptions>,
	/// Indicates which columns the wrapping should be applied to.
	wrap_columns: Option<Vec<usize>>,
}

impl Default for BoxGridRenderer {
	fn default() -> Self {
		Self::new()
	}
}

impl BoxGridRenderer {
	/// Constructs a new `BoxGridRenderer`.
	#[must_use]
	pub fn new() -> Self {
		Self {
			column_padding: 1,
			extra_column_width: 0,
			style: BoxGridStyle::new(),
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

	/// Sets the extra column width and returns the `MarkdownGridRenderer`.
	#[must_use]
	pub fn with_extra_column_width(mut self, extra_column_width: u8) -> Self {
		self.extra_column_width = extra_column_width;
		self
	}

	/// Sets the style and returns the `BoxGridRenderer`.
	#[must_use]
	pub fn with_style(mut self, style: BoxGridStyle) -> Self {
		self.style = style;
		self
	}

	/// Sets the text wrap options and returns the `BoxGridRenderer`.
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

	/// Sets the text wrap columns and returns the `BoxGridRenderer`.
	///
	/// A `None` value will apply the wrapping to all columns.
	#[must_use]
	pub fn with_wrap_columns<O>(mut self, wrap_columns: O) -> Self 
		where O: Into<Option<Vec<usize>>>
	{
		self.wrap_columns = wrap_columns.into();
		self
	}

	/// Writes a row divider.
	fn write_div<W, L, H, C, R>(
		&self, 
		out: &mut W,
		ctx: &RenderContext<'_>,
		left: L,
		horz: H,
		cross: C,
		right: R)
		-> std::io::Result<()>
		where
			W: std::io::Write,
			L: Display,
			H: Display,
			C: Display,
			R: Display,
	{
		let pad = self.column_padding;

		write!(out, "{}", left)?;
		for (col, col_width) in ctx.col_widths.iter().copied().enumerate() {
			for _ in 0..col_width { write!(out, "{}", horz)?; }
			for _ in 0..pad { write!(out, "{}{}", horz, horz)?; }
			if col + 1 == ctx.column_count() { break; }
			write!(out, "{}", cross)?;
		}
		write!(out, "{}", right)?;
		Ok(())
	}

	/// Writes the top border line.
	fn write_border_top<W>(&self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_div(
			out,
			ctx,
			self.style.border_top.corner_top_left(
				self.style.border_left,
				self.style.round_corners),
			self.style.border_top.horz(),
			self.style.border_top.horz_with_bottom(
				self.style.col_sep),
			self.style.border_top.corner_top_right(
				self.style.border_right,
				self.style.round_corners))
	}

	/// Writes a section separator.
	fn write_section_sep<W>(&self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_div(
			out,
			ctx,
			self.style.border_left.vert_with_right(
				self.style.div_sep),
			self.style.div_sep.horz(),
			self.style.div_sep.cross(
				self.style.col_sep),
			self.style.border_right.vert_with_left(
				self.style.div_sep))
	}

	/// Writes a data row separator.
	fn write_row_sep<W>(&self, out: &mut W,ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_div(
			out,
			ctx,
			self.style.border_left.vert_with_right(
				self.style.row_sep),
			self.style.row_sep.horz(),
			self.style.row_sep.cross(
				self.style.col_sep),
			self.style.border_right.vert_with_left(
				self.style.row_sep))
	}

	/// Renders a column separator.
	fn write_column_sep<W>(&self, out: &mut W) -> std::io::Result<()>
		where W: std::io::Write
	{
		let pad = self.column_padding;

		for _ in 0..pad { write!(out, " ")?; }
		write!(out, "{}", self.style.col_sep.vert())?;
		for _ in 0..pad { write!(out, " ")?; }
		Ok(())
	}

	/// Writes the left border of a line.
	fn write_border_left<W>(&self, out: &mut W) -> std::io::Result<()>
		where W: std::io::Write
	{
		let pad = self.column_padding;
		
		write!(out, "{}", self.style.border_left.vert())?;
		for _ in 0..pad { write!(out, " ")?; }
		Ok(())
	}

	/// Writes the right border of a line.
	fn write_border_right<W>(&self, out: &mut W) -> std::io::Result<()>
		where W: std::io::Write
	{
		let pad = self.column_padding;

		for _ in 0..pad { write!(out, " ")?; }
		write!(out, "{}", self.style.border_right.vert())?;
		Ok(())
	}

	/// Writes the bottom border line.
	fn write_border_bottom<W>(&self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_div(
			out,
			ctx,
			self.style.border_bottom.corner_bottom_left(
				self.style.border_left,
				self.style.round_corners),
			self.style.border_bottom.horz(),
			self.style.border_bottom.horz_with_top(
				self.style.col_sep),
			self.style.border_bottom.corner_bottom_right(
				self.style.border_right,
				self.style.round_corners))
	}
}

impl Renderer for BoxGridRenderer {
	fn features(&self) -> Features {
		let padding: usize = self.column_padding.into();
		let mut features = Features::default()
			.with_extra_column_width(self.extra_column_width.into())
			.with_width_contribution_fn(Box::new(move |col_count| {
				// Width of dividers
				(col_count + 1) 
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

	fn write_table_start<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_empty() { return Ok(()) }
		self.write_border_top(out, ctx)?;
		writeln!(out)
	}

	fn write_header_end<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_empty() { return Ok(()) }
		self.write_section_sep(out, ctx)?;
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
			self.write_border_left(out,)?;
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
		if ctx.is_last_column() {
			self.write_border_right(out,)
		} else {
			self.write_column_sep(out)
		}
	}

	fn write_data_row_start<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_empty() || ctx.is_first_row() { return Ok(()) }
		self.write_row_sep(out, ctx)?;
		writeln!(out)
	}

	fn write_footer_start<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_header_end(out, ctx)
	}

	fn write_table_end<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_empty() { return Ok(()) }
		self.write_border_bottom(out, ctx)?;
		writeln!(out)
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
	use table_gen::HorzAlign;
	use table_gen::Table;

	#[test]
	fn empty_table() {
		let data: Vec<(usize, )> = vec![];

		let mut table = Table::new_builder(data, BoxGridRenderer::new())
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
		let mut table = Table::new_builder(data, BoxGridRenderer::new())
			.with_sort_columns(&order)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
┌───────┐
│ 30000 │
├───────┤
│ 2000  │
├───────┤
│ 100   │
├───────┤
│ 10    │
├───────┤
│ 1     │
└───────┘
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

		let mut table = Table::new_builder(data, BoxGridRenderer::new())
			.with_column_defs(&column_defs)
			.with_column_selection(&[0, 0, 0])
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		println!("{}", out);

		assert_eq!(out, "\
┌───────┬───────┬────────┐
│ Right ┆ Left  ┆ Center │
╞═══════╪═══════╪════════╡
│    12 ┆ 12    ┆   12   │
├───────┼───────┼────────┤
│   123 ┆ 123   ┆  123   │
├───────┼───────┼────────┤
│     1 ┆ 1     ┆   1    │
├───────┼───────┼────────┤
│ -8000 ┆ -8000 ┆ -8000  │
└───────┴───────┴────────┘
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

		let mut table = Table::new_builder(data, BoxGridRenderer::new())
			.with_column_defs(&column_defs)
			.with_column_selection(&[0, 0, 0])
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
┌───────┬───────┬───────┐
│    12 ┆ 12    ┆  12   │
├───────┼───────┼───────┤
│   123 ┆ 123   ┆  123  │
├───────┼───────┼───────┤
│     1 ┆ 1     ┆   1   │
├───────┼───────┼───────┤
│ -8000 ┆ -8000 ┆ -8000 │
└───────┴───────┴───────┘
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

		let mut table = Table::new_builder(data, BoxGridRenderer::new())
			.with_column_defs(&column_defs)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
┌──────────┬─────────┬─────────┬──────────────────────────┐
│ Centered ┆ Left    ┆   Right ┆ Left                     │
│  Header  ┆ Aligned ┆ Aligned ┆ Aligned                  │
╞══════════╪═════════╪═════════╪══════════════════════════╡
│  First   ┆ row     ┆      12 ┆ Example of a row that    │
│          ┆         ┆         ┆ spans multiple lines.    │
├──────────┼─────────┼─────────┼──────────────────────────┤
│  Second  ┆ row     ┆       5 ┆ Here's another one. Note │
│          ┆         ┆         ┆ the blank line between   │
│          ┆         ┆         ┆ rows                     │
└──────────┴─────────┴─────────┴──────────────────────────┘
");
	}

	#[test]
	fn multiline_table_alt() {
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

		let mut table = Table::new_builder(data, BoxGridRenderer::new()
				.with_style(BoxGridStyle {
					border_left: LineShape::Light.into(),
					border_right: LineShape::Light.into(),
					border_top: LineShape::Double.into(),
					border_bottom: LineShape::Light.into(),
					row_sep: LineShape::Empty.into(),
					col_sep: LineShape::Heavy.into(),
					div_sep: LineShape::HeavyDash2.into(),
					round_corners: true,
				}))
			.with_column_defs(&column_defs)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
╒══════════╦═════════╦═════════╦══════════════════════════╕
│ Centered ┃ Left    ┃   Right ┃ Left                     │
│  Header  ┃ Aligned ┃ Aligned ┃ Aligned                  │
┝╍╍╍╍╍╍╍╍╍╍╋╍╍╍╍╍╍╍╍╍╋╍╍╍╍╍╍╍╍╍╋╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍┥
│  First   ┃ row     ┃      12 ┃ Example of a row that    │
│          ┃         ┃         ┃ spans multiple lines.    │
│          ┃         ┃         ┃                          │
│  Second  ┃ row     ┃       5 ┃ Here's another one. Note │
│          ┃         ┃         ┃ the blank line between   │
│          ┃         ┃         ┃ rows                     │
╰──────────┸─────────┸─────────┸──────────────────────────╯
");
	}

	#[test]
	fn expanded_width_table() {
		let data: Vec<[usize; 8]> = vec![
			[2, 3, 4, 5, 6, 7, 8, 9],
			[22, 43, 54, 85, 96, 907, 8, 19],
			[23, 35, 46, 58, 69, 709, 8, 19],
		];

		let mut table = Table::new_builder(data, BoxGridRenderer::new())
			.with_min_table_width(80)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
┌─────────┬─────────┬─────────┬─────────┬─────────┬──────────┬────────┬────────┐
│ 2       ┆ 3       ┆ 4       ┆ 5       ┆ 6       ┆ 7        ┆ 8      ┆ 9      │
├─────────┼─────────┼─────────┼─────────┼─────────┼──────────┼────────┼────────┤
│ 22      ┆ 43      ┆ 54      ┆ 85      ┆ 96      ┆ 907      ┆ 8      ┆ 19     │
├─────────┼─────────┼─────────┼─────────┼─────────┼──────────┼────────┼────────┤
│ 23      ┆ 35      ┆ 46      ┆ 58      ┆ 69      ┆ 709      ┆ 8      ┆ 19     │
└─────────┴─────────┴─────────┴─────────┴─────────┴──────────┴────────┴────────┘
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

		let mut table = Table::new_builder(data, BoxGridRenderer::new()
				.with_wrap_options(None))
			.with_column_defs(&column_defs)
			.with_max_table_width(80)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
┌─────────┬──────────────────┬──────────┬──────────────────────────────────────┐
│ COLUMN… ┆ COLUMN B         ┆ COLUMN C ┆ COLUMN D                             │
╞═════════╪══════════════════╪══════════╪══════════════════════════════════════╡
│ Sed ut… ┆ unde omnis iste… ┆ sit vol… ┆ doloremque laudantium, totam rem ap… │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ eaque … ┆ ab illo invento… ┆ quasi a… ┆ vitae dicta sunt explicabo. Nemo en… │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ volupt… ┆ sit aspernatur … ┆ fugit, … ┆ quia consequuntur magni dolores eos… │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ volupt… ┆ Neque porro qui… ┆ qui dol… ┆ quia dolor sit amet, consectetur, a… │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ velit,… ┆ quia non numqua… ┆ tempora… ┆ labore et dolore magnam aliquam qua… │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ enim a… ┆ veniam, quis no… ┆ ullam c… ┆ laboriosam, nisi ut aliquid ex ea    │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ commod… ┆ Quis autem vel … ┆ reprehe… ┆ ea voluptate velit esse quam nihil … │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ conseq… ┆ illum qui dolor… ┆ quo vol… ┆ pariatur?                            │
╞═════════╪══════════════════╪══════════╪══════════════════════════════════════╡
│ COLUMN… ┆ COLUMN B         ┆ COLUMN C ┆ COLUMN D                             │
└─────────┴──────────────────┴──────────┴──────────────────────────────────────┘
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

		let mut table = Table::new_builder(data, BoxGridRenderer::new())
			.with_column_defs(&column_defs)
			.with_max_table_width(80)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
┌─────────┬──────────────────┬──────────┬──────────────────────────────────────┐
│ COLUMN… ┆ COLUMN B         ┆ COLUMN C ┆ COLUMN D                             │
╞═════════╪══════════════════╪══════════╪══════════════════════════════════════╡
│ Sed ut  ┆ unde omnis iste  ┆ sit      ┆ doloremque laudantium, totam rem     │
│ perspi… ┆ natus error      ┆ volupta… ┆ aperiam,                             │
│         ┆                  ┆ accusan… ┆                                      │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ eaque   ┆ ab illo          ┆ quasi    ┆ vitae dicta sunt explicabo. Nemo     │
│ ipsa    ┆ inventore        ┆ archite… ┆ enim ipsam                           │
│ quae    ┆ veritatis et     ┆ beatae   ┆                                      │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ volupt… ┆ sit aspernatur   ┆ fugit,   ┆ quia consequuntur magni dolores eos  │
│ quia    ┆ aut odit aut     ┆ sed      ┆ qui ratione                          │
│ volupt… ┆                  ┆          ┆                                      │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ volupt… ┆ Neque porro      ┆ qui      ┆ quia dolor sit amet, consectetur,    │
│ sequi   ┆ quisquam est,    ┆ dolorem  ┆ adipisci                             │
│ nesciu… ┆                  ┆ ipsum    ┆                                      │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ velit,  ┆ quia non numquam ┆ tempora  ┆ labore et dolore magnam aliquam      │
│ sed     ┆ eius modi        ┆ incidunt ┆ quaerat voluptatem. Ut               │
│         ┆                  ┆ ut       ┆                                      │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ enim ad ┆ veniam,          ┆ ullam    ┆ laboriosam, nisi ut aliquid ex ea    │
│ minima  ┆ quis nostrum     ┆ corporis ┆                                      │
│         ┆ exercitationem   ┆ suscipit ┆                                      │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ commodi ┆ Quis autem vel   ┆ reprehe… ┆ ea voluptate velit esse quam nihil   │
│ conseq… ┆ eum iure         ┆ qui in   ┆ molestiae                            │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ conseq… ┆ illum qui        ┆ quo      ┆ pariatur?                            │
│ vel     ┆ dolorem eum      ┆ voluptas ┆                                      │
│         ┆ fugiat           ┆ nulla    ┆                                      │
╞═════════╪══════════════════╪══════════╪══════════════════════════════════════╡
│ COLUMN… ┆ COLUMN B         ┆ COLUMN C ┆ COLUMN D                             │
└─────────┴──────────────────┴──────────┴──────────────────────────────────────┘
");
	}
}
