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
use table_gen::util::write_cell_formatted;

// Standard library imports.
use std::fmt::Display;


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
	/// The amount of space to allocate between columns.
	column_padding: u8,
	/// The amount of extra space to allocate within columns.
	extra_column_width: u8,
	/// The `BoxGridStyle` to render with.
	style: BoxGridStyle,
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
			column_padding: 0,
			extra_column_width: 0,
			style: BoxGridStyle::new(),
		}
	}

	/// Sets the column padding and returns the `MarkdownGridRenderer`.
	#[must_use]
	pub fn with_column_padding(mut self, column_padding: u8) -> Self {
		self.column_padding = column_padding;
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
		let pad = std::cmp::max(self.column_padding / 2, 2);

		write!(out, "{}", left)?;
		for (col, col_width) in ctx.col_widths.iter().copied().enumerate() {
			for _ in 0..col_width { write!(out, "{}", horz)?; }
			for _ in 0..pad { write!(out, "{}", horz)?; }
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
		let pad = std::cmp::max(self.column_padding / 2, 2) / 2;

		for _ in 0..pad { write!(out, " ")?; }
		write!(out, "{}", self.style.col_sep.vert())?;
		for _ in 0..pad { write!(out, " ")?; }
		Ok(())
	}

	/// Writes the left border of a line.
	fn write_border_left<W>(&self, out: &mut W) -> std::io::Result<()>
		where W: std::io::Write
	{
		let pad = std::cmp::max(self.column_padding / 2, 2) / 2;
		
		write!(out, "{}", self.style.border_left.vert())?;
		for _ in 0..pad { write!(out, " ")?; }
		Ok(())
	}

	/// Writes the right border of a line.
	fn write_border_right<W>(&self, out: &mut W) -> std::io::Result<()>
		where W: std::io::Write
	{
		let pad = std::cmp::max(self.column_padding / 2, 2) / 2;

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
		Features::default()
			.with_extra_column_width(self.extra_column_width.into())
			.with_width_contribution_fn(Box::new(move |col_count| {
				col_count * padding
			}))
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
			"…",
			self.column_padding.into())
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
		//println!("{}", out);

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
}
