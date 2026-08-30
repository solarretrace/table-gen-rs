////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! A table renderer that renders tables using box-drawing unicode style with
//! distinct 'tiles' for each cell.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::LineShape;
use crate::LineStyle;

// Workspace library imports.
use table_gen::CellContext;
use table_gen::Features;
use table_gen::RenderContext;
use table_gen::Renderer;
use table_gen::SupportFlags;
use table_gen::util::Style;
use table_gen::WrapOptions;
use table_gen::util::write_cell_formatted;

// Standard library imports.
use std::fmt::Display;
use std::rc::Rc;


////////////////////////////////////////////////////////////////////////////////
// BoxTileStyle
////////////////////////////////////////////////////////////////////////////////
/// The style specification for the box renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoxTileStyle {
	/// The style to use for the header borders.
	pub header: LineStyle,
	/// The style to use for the footer borders.
	pub footer: LineStyle,
	/// The style to use for the data borders.
	pub data: LineStyle,
	/// Whether to use rounded corner variants.
	pub round_corners: bool,
}

impl Default for BoxTileStyle {
	fn default() -> Self {
		Self::new()
	}
}

impl BoxTileStyle {
	/// Constructs a new `BoxTileStyle` with the default styling.
	#[must_use]
	pub fn new() -> Self {
		Self {
			header: LineShape::Double.into(),
			footer: LineShape::Double.into(),
			data: LineShape::Light.into(),
			round_corners: false,
		}
	}

	/// Sets the round corners flag for the borders and returns the
	/// `BoxTileStyle`.
	#[must_use]
	pub fn with_round_corners(mut self, round_corners: bool) -> Self {
		self.round_corners = round_corners;
		self
	}

	/// Sets the `LineShape` for the header borders and returns the
	/// `BoxTileStyle`.
	#[must_use]
	pub fn with_header_borders_shape(mut self, line_shape: LineShape) -> Self {
		self.header.shape = line_shape;
		self
	}

	/// Sets the `Style` for the header borders and returns the
	/// `BoxTileStyle`.
	#[must_use]
	pub fn with_header_borders_style(mut self, style: Style) -> Self {
		self.header.style = style;
		self
	}

	/// Sets the `LineShape` for the footer borders and returns the
	/// `BoxTileStyle`.
	#[must_use]
	pub fn with_footer_borders_shape(mut self, line_shape: LineShape) -> Self {
		self.footer.shape = line_shape;
		self
	}

	/// Sets the `Style` for the footer borders and returns the
	/// `BoxTileStyle`.
	#[must_use]
	pub fn with_footer_borders_style(mut self, style: Style) -> Self {
		self.footer.style = style;
		self
	}

	/// Sets the `LineShape` for the data borders and returns the
	/// `BoxTileStyle`.
	#[must_use]
	pub fn with_data_borders_shape(mut self, line_shape: LineShape) -> Self {
		self.data.shape = line_shape;
		self
	}

	/// Sets the `Style` for the data borders and returns the
	/// `BoxTileStyle`.
	#[must_use]
	pub fn with_data_borders_style(mut self, style: Style) -> Self {
		self.data.style = style;
		self
	}
}

////////////////////////////////////////////////////////////////////////////////
// BoxTileRenderer
////////////////////////////////////////////////////////////////////////////////
/// A table renderer that renders tables using box-drawing unicode style.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct BoxTileRenderer {
	/// The amount of space to allocate between columns.
	column_padding: u8,
	/// The amount of extra space to allocate within columns.
	extra_column_width: u8,
	/// The `BoxTileStyle` to render with.
	style: BoxTileStyle,
}

impl Default for BoxTileRenderer {
	fn default() -> Self {
		Self::new()
	}
}

impl BoxTileRenderer {
	/// Constructs a new `BoxTileRenderer`.
	#[must_use]
	pub fn new() -> Self {
		Self {
			column_padding: 1,
			extra_column_width: 0,
			style: BoxTileStyle::new(),
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

	/// Sets the style and returns the `BoxTileRenderer`.
	#[must_use]
	pub fn with_style(mut self, style: BoxTileStyle) -> Self {
		self.style = style;
		self
	}

	/// Writes a row divider line.
	fn write_div<W, L, H, R>(
		&self, 
		out: &mut W,
		ctx: &RenderContext<'_>,
		left: L,
		horz: H,
		right: R)
		-> std::io::Result<()>
		where
			W: std::io::Write,
			L: Display,
			H: Display,
			R: Display,
	{
		let pad = self.column_padding;

		write!(out, "{}", left)?;
		for (col, col_width) in ctx.col_widths.iter().copied().enumerate() {
			for _ in 0..col_width { write!(out, "{}", horz)?; }
			for _ in 0..pad { write!(out, "{}{}", horz, horz)?; }
			write!(out, "{}", right)?;
			if col + 1 == ctx.column_count() { break; }
			write!(out, "{}", left)?;
		}
		Ok(())
	}

	/// Writes the left border of a line.
	fn write_border_left<W>(&self, out: &mut W, style: LineStyle)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		let pad = self.column_padding;
		
		write!(out, "{}", style.vert())?;
		for _ in 0..pad { write!(out, " ")?; }
		Ok(())
	}

	/// Writes the right border of a line.
	fn write_border_right<W>(&self, out: &mut W, style: LineStyle)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		let pad = self.column_padding;

		for _ in 0..pad { write!(out, " ")?; }
		write!(out, "{}", style.vert())?;
		Ok(())
	}

}

impl Renderer for BoxTileRenderer {
	fn features(&self) -> Features {
		let padding: usize = self.column_padding.into();
		Features::new(SupportFlags::new()
				| SupportFlags::FOOTERS
				| SupportFlags::COLUMN_WIDTH_ALL
				| SupportFlags::MULTILINE_ALL
				| SupportFlags::ANSI_STYLE)
			.with_extra_column_width(self.extra_column_width.into())
			.with_width_contribution_fn(Rc::new(move |col_count| {
				// Width of dividers
				(col_count * 2) 
				// Width of cell padding
				+ (col_count * padding * 2)
			}))
			.with_default_text_wrap(WrapOptions::new())
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

	fn write_header_start<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_empty() { return Ok(()) }
		self.write_div(
			out,
			ctx,
			self.style.header.corner_top_left(
				self.style.header,
				self.style.round_corners),
			self.style.header.horz(),
			self.style.header.corner_top_right(
				self.style.header,
				self.style.round_corners))?;
		writeln!(out)
	}

	fn write_header_end<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_empty() { return Ok(()) }
		self.write_div(
			out,
			ctx,
			self.style.header.corner_bottom_left(
				self.style.header,
				self.style.round_corners),
			self.style.header.horz(),
			self.style.header.corner_bottom_right(
				self.style.header,
				self.style.round_corners))?;
		writeln!(out)
	}
	
	fn write_header_cell_line_start<W>(
		&mut self,
		out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_border_left(out, self.style.header)
	}

	fn write_header_cell_line_end<W>(
		&mut self,
		out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_border_right(out, self.style.header)
	}

	fn write_data_cell_line_start<W>(
		&mut self,
		out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_border_left(out, self.style.data)
	}

	fn write_data_cell_line_end<W>(
		&mut self,
		out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_border_right(out, self.style.data)
	}

	fn write_footer_cell_line_start<W>(
		&mut self,
		out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_border_left(out, self.style.footer)
	}

	fn write_footer_cell_line_end<W>(
		&mut self,
		out: &mut W,
		_ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		self.write_border_right(out, self.style.footer)
	}


	fn write_data_row_start<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_empty() { return Ok(()) }
		self.write_div(
			out,
			ctx,
			self.style.data.corner_top_left(
				self.style.data,
				self.style.round_corners),
			self.style.data.horz(),
			self.style.data.corner_top_right(
				self.style.data,
				self.style.round_corners))?;
		writeln!(out)
	}

	fn write_data_row_end<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_empty() { return Ok(()) }
		self.write_div(
			out,
			ctx,
			self.style.data.corner_bottom_left(
				self.style.data,
				self.style.round_corners),
			self.style.data.horz(),
			self.style.data.corner_bottom_right(
				self.style.data,
				self.style.round_corners))?;
		writeln!(out)
	}

	fn write_footer_start<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_empty() { return Ok(()) }
		self.write_div(
			out,
			ctx,
			self.style.footer.corner_top_left(
				self.style.footer,
				self.style.round_corners),
			self.style.footer.horz(),
			self.style.footer.corner_top_right(
				self.style.footer,
				self.style.round_corners))?;
		writeln!(out)
	}

	fn write_footer_end<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_empty() { return Ok(()) }
		self.write_div(
			out,
			ctx,
			self.style.footer.corner_bottom_left(
				self.style.footer,
				self.style.round_corners),
			self.style.footer.horz(),
			self.style.footer.corner_bottom_right(
				self.style.footer,
				self.style.round_corners))?;
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
	use table_gen::TextWrap;

	#[test]
	fn empty_table() {
		let data: Vec<(usize, )> = vec![];

		let mut table = Table::new_builder(data, BoxTileRenderer::new())
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
		let mut table = Table::new_builder(data, BoxTileRenderer::new())
			.with_sort_columns(&order)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
┌───────┐
│ 30000 │
└───────┘
┌───────┐
│ 2000  │
└───────┘
┌───────┐
│ 100   │
└───────┘
┌───────┐
│ 10    │
└───────┘
┌───────┐
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

		let mut table = Table::new_builder(data, BoxTileRenderer::new())
			.with_column_defs(&column_defs)
			.with_column_selection(&[0, 0, 0])
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
╔═══════╗╔═══════╗╔════════╗
║ Right ║║ Left  ║║ Center ║
╚═══════╝╚═══════╝╚════════╝
┌───────┐┌───────┐┌────────┐
│    12 ││ 12    ││   12   │
└───────┘└───────┘└────────┘
┌───────┐┌───────┐┌────────┐
│   123 ││ 123   ││  123   │
└───────┘└───────┘└────────┘
┌───────┐┌───────┐┌────────┐
│     1 ││ 1     ││   1    │
└───────┘└───────┘└────────┘
┌───────┐┌───────┐┌────────┐
│ -8000 ││ -8000 ││ -8000  │
└───────┘└───────┘└────────┘
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

		let mut table = Table::new_builder(data, BoxTileRenderer::new())
			.with_column_defs(&column_defs)
			.with_column_selection(&[0, 0, 0])
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
┌───────┐┌───────┐┌───────┐
│    12 ││ 12    ││  12   │
└───────┘└───────┘└───────┘
┌───────┐┌───────┐┌───────┐
│   123 ││ 123   ││  123  │
└───────┘└───────┘└───────┘
┌───────┐┌───────┐┌───────┐
│     1 ││ 1     ││   1   │
└───────┘└───────┘└───────┘
┌───────┐┌───────┐┌───────┐
│ -8000 ││ -8000 ││ -8000 │
└───────┘└───────┘└───────┘
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

		let mut table = Table::new_builder(data, BoxTileRenderer::new())
			.with_column_defs(&column_defs)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
╔══════════╗╔═════════╗╔═════════╗╔══════════════════════════╗
║ Centered ║║ Left    ║║   Right ║║ Left                     ║
║  Header  ║║ Aligned ║║ Aligned ║║ Aligned                  ║
╚══════════╝╚═════════╝╚═════════╝╚══════════════════════════╝
┌──────────┐┌─────────┐┌─────────┐┌──────────────────────────┐
│  First   ││ row     ││      12 ││ Example of a row that    │
│          ││         ││         ││ spans multiple lines.    │
└──────────┘└─────────┘└─────────┘└──────────────────────────┘
┌──────────┐┌─────────┐┌─────────┐┌──────────────────────────┐
│  Second  ││ row     ││       5 ││ Here's another one. Note │
│          ││         ││         ││ the blank line between   │
│          ││         ││         ││ rows                     │
└──────────┘└─────────┘└─────────┘└──────────────────────────┘
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

		let mut table = Table::new_builder(data, BoxTileRenderer::new()
				.with_style(BoxTileStyle {
					header: LineShape::Heavy.into(),
					footer: LineShape::Light.into(),
					data: LineShape::LightDash4.into(),
					round_corners: true,
				}))
			.with_column_defs(&column_defs)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
┏━━━━━━━━━━┓┏━━━━━━━━━┓┏━━━━━━━━━┓┏━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ Centered ┃┃ Left    ┃┃   Right ┃┃ Left                     ┃
┃  Header  ┃┃ Aligned ┃┃ Aligned ┃┃ Aligned                  ┃
┗━━━━━━━━━━┛┗━━━━━━━━━┛┗━━━━━━━━━┛┗━━━━━━━━━━━━━━━━━━━━━━━━━━┛
╭┈┈┈┈┈┈┈┈┈┈╮╭┈┈┈┈┈┈┈┈┈╮╭┈┈┈┈┈┈┈┈┈╮╭┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈╮
┊  First   ┊┊ row     ┊┊      12 ┊┊ Example of a row that    ┊
┊          ┊┊         ┊┊         ┊┊ spans multiple lines.    ┊
╰┈┈┈┈┈┈┈┈┈┈╯╰┈┈┈┈┈┈┈┈┈╯╰┈┈┈┈┈┈┈┈┈╯╰┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈╯
╭┈┈┈┈┈┈┈┈┈┈╮╭┈┈┈┈┈┈┈┈┈╮╭┈┈┈┈┈┈┈┈┈╮╭┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈╮
┊  Second  ┊┊ row     ┊┊       5 ┊┊ Here's another one. Note ┊
┊          ┊┊         ┊┊         ┊┊ the blank line between   ┊
┊          ┊┊         ┊┊         ┊┊ rows                     ┊
╰┈┈┈┈┈┈┈┈┈┈╯╰┈┈┈┈┈┈┈┈┈╯╰┈┈┈┈┈┈┈┈┈╯╰┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈╯
");
	}

	#[test]
	fn expanded_width_table() {
		let data: Vec<[usize; 8]> = vec![
			[2, 3, 4, 5, 6, 7, 8, 9],
			[22, 43, 54, 85, 96, 907, 8, 19],
			[23, 35, 46, 58, 69, 709, 8, 19],
		];

		let mut table = Table::new_builder(data, BoxTileRenderer::new())
			.with_min_table_width(80)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
┌────────┐┌────────┐┌────────┐┌────────┐┌────────┐┌─────────┐┌───────┐┌────────┐
│ 2      ││ 3      ││ 4      ││ 5      ││ 6      ││ 7       ││ 8     ││ 9      │
└────────┘└────────┘└────────┘└────────┘└────────┘└─────────┘└───────┘└────────┘
┌────────┐┌────────┐┌────────┐┌────────┐┌────────┐┌─────────┐┌───────┐┌────────┐
│ 22     ││ 43     ││ 54     ││ 85     ││ 96     ││ 907     ││ 8     ││ 19     │
└────────┘└────────┘└────────┘└────────┘└────────┘└─────────┘└───────┘└────────┘
┌────────┐┌────────┐┌────────┐┌────────┐┌────────┐┌─────────┐┌───────┐┌────────┐
│ 23     ││ 35     ││ 46     ││ 58     ││ 69     ││ 709     ││ 8     ││ 19     │
└────────┘└────────┘└────────┘└────────┘└────────┘└─────────┘└───────┘└────────┘
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

		let mut table = Table::new_builder(data, BoxTileRenderer::new())
			.with_default_column_def(ColumnDef::new()
				.with_text_wrap(TextWrap::Disabled))
			.with_column_defs(&column_defs)
			.with_max_table_width(80)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
╔════════╗╔══════════════════╗╔═════════╗╔═════════════════════════════════════╗
║ COLUM… ║║ COLUMN B         ║║ COLUMN… ║║ COLUMN D                            ║
╚════════╝╚══════════════════╝╚═════════╝╚═════════════════════════════════════╝
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ Sed u… ││ unde omnis iste… ││ sit vo… ││ doloremque laudantium, totam rem a… │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ eaque… ││ ab illo invento… ││ quasi … ││ vitae dicta sunt explicabo. Nemo e… │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ volup… ││ sit aspernatur … ││ fugit,… ││ quia consequuntur magni dolores eo… │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ volup… ││ Neque porro qui… ││ qui do… ││ quia dolor sit amet, consectetur, … │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ velit… ││ quia non numqua… ││ tempor… ││ labore et dolore magnam aliquam qu… │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ enim … ││ veniam, quis no… ││ ullam … ││ laboriosam, nisi ut aliquid ex ea   │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ commo… ││ Quis autem vel … ││ repreh… ││ ea voluptate velit esse quam nihil… │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ conse… ││ illum qui dolor… ││ quo vo… ││ pariatur?                           │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
╔════════╗╔══════════════════╗╔═════════╗╔═════════════════════════════════════╗
║ COLUM… ║║ COLUMN B         ║║ COLUMN… ║║ COLUMN D                            ║
╚════════╝╚══════════════════╝╚═════════╝╚═════════════════════════════════════╝
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

		let mut table = Table::new_builder(data, BoxTileRenderer::new())
			.with_column_defs(&column_defs)
			.with_max_table_width(80)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
╔════════╗╔══════════════════╗╔═════════╗╔═════════════════════════════════════╗
║ COLUM… ║║ COLUMN B         ║║ COLUMN… ║║ COLUMN D                            ║
╚════════╝╚══════════════════╝╚═════════╝╚═════════════════════════════════════╝
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ Sed ut ││ unde omnis iste  ││ sit     ││ doloremque laudantium, totam rem    │
│ persp… ││ natus error      ││ volupt… ││ aperiam,                            │
│        ││                  ││ accusa… ││                                     │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ eaque  ││ ab illo          ││ quasi   ││ vitae dicta sunt explicabo. Nemo    │
│ ipsa   ││ inventore        ││ archit… ││ enim ipsam                          │
│ quae   ││ veritatis et     ││ beatae  ││                                     │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ volup… ││ sit aspernatur   ││ fugit,  ││ quia consequuntur magni dolores eos │
│ quia   ││ aut odit aut     ││ sed     ││ qui ratione                         │
│ volup… ││                  ││         ││                                     │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ volup… ││ Neque porro      ││ qui     ││ quia dolor sit amet, consectetur,   │
│ sequi  ││ quisquam est,    ││ dolorem ││ adipisci                            │
│ nesci… ││                  ││ ipsum   ││                                     │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ velit, ││ quia non numquam ││ tempora ││ labore et dolore magnam aliquam     │
│ sed    ││ eius modi        ││ incidu… ││ quaerat voluptatem. Ut              │
│        ││                  ││ ut      ││                                     │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ enim   ││ veniam,          ││ ullam   ││ laboriosam, nisi ut aliquid ex ea   │
│ ad     ││ quis nostrum     ││ corpor… ││                                     │
│ minima ││ exercitationem   ││ suscip… ││                                     │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ commo… ││ Quis autem vel   ││ repreh… ││ ea voluptate velit esse quam nihil  │
│ conse… ││ eum iure         ││ qui in  ││ molestiae                           │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ conse… ││ illum qui        ││ quo     ││ pariatur?                           │
│ vel    ││ dolorem eum      ││ volupt… ││                                     │
│        ││ fugiat           ││ nulla   ││                                     │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
╔════════╗╔══════════════════╗╔═════════╗╔═════════════════════════════════════╗
║ COLUM… ║║ COLUMN B         ║║ COLUMN… ║║ COLUMN D                            ║
╚════════╝╚══════════════════╝╚═════════╝╚═════════════════════════════════════╝
");
	}
}
