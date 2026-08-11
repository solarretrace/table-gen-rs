////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator driver module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::Row;
use crate::Renderer;
use crate::Collate;
use crate::Aggregate;
use crate::TextRow;
use crate::SplitRow;
use crate::ColumnDesc;
use crate::ColumnOrd;
use crate::RenderContext;
use crate::CellContext;

// Standard library imports.
use std::ops::RangeBounds;


////////////////////////////////////////////////////////////////////////////////
// TableBuilder
////////////////////////////////////////////////////////////////////////////////
/// A builder-style constructor for a `Table`.
#[derive(Debug, Clone)]
pub struct TableBuilder<'a, S, T> {
	/// The table data source.
	inner: Collate<'a, S>,
	/// The table renderer.
	renderer: T,
	/// The default ColumnDesc.
	default_col_desc: ColumnDesc<'a>,
}

impl<'a, R, S, T> TableBuilder<'a, S, T>
	where
		R: Row,
		S: Iterator<Item=R>,
		T: Renderer,
{
	/// Constructs a new `TableBuilder`.
	pub fn new<I>(source: I, renderer: T) -> TableBuilder<'a, S, T>
		where I: IntoIterator<Item=R, IntoIter=S>
	{
		Self {
			inner: Collate::new(source.into_iter())
				.with_features(renderer.features()),
			renderer,
			default_col_desc: ColumnDesc::new(),
		}
	}

	/// Prepares the table builder with the given columns selected for output in
	/// the given order.
	pub fn with_column_selection(mut self, col_select: &'a [usize]) -> Self {
		self.inner = self.inner.with_column_selection(col_select);
		self
	}

	/// Prepares the table builder such that only the given rows will be
	/// rendered.
	pub fn with_row_selection<B>(mut self, row_select: B) -> Self 
		where B: RangeBounds<usize>
	{
		self.inner = self.inner.with_row_selection(row_select);
		self
	}

	/// Prepares the table builder with the given column output specifications.
	pub fn with_column_descs(mut self, col_descs: &'a [ColumnDesc<'a>]) -> Self
	{
		self.inner = self.inner.with_column_descs(col_descs);
		self
	}

	/// Prepares the table builder with the given default column output
	/// specification.
	pub fn with_default_col_desc(mut self, default_col_desc: ColumnDesc<'a>)
		-> Self
	{
		self.default_col_desc = default_col_desc;
		self
	}

	/// Prepares the table builder with the given output column orderings.
	pub fn with_sort_columns(mut self, col_order: &'a [ColumnOrd]) -> Self {
		self.inner = self.inner.with_sort_columns(col_order);
		self
	}

	/// Finishes collation of the data source and returns a `Table` for
	/// rendering.
	pub fn finish(self) -> Table<'a, R, T> {
		Table::new(self.inner, self.default_col_desc, self.renderer)
	}
}

impl<'a, R, S, T> From<TableBuilder<'a, S, T>> for Table<'a, R, T>
	where
		R: Row,
		S: Iterator<Item=R>,
		T: Renderer,
{
	fn from(builder: TableBuilder<'a, S, T>) -> Self {
		builder.finish()
	}
}


////////////////////////////////////////////////////////////////////////////////
// Table
////////////////////////////////////////////////////////////////////////////////
/// A driver for a table renderer operating on a data source.
#[derive(Debug, Clone)]
pub struct Table<'a, R, T> {
	/// The table data source.
	inner: Aggregate<'a, R>,
	/// The table renderer.
	renderer: T,
	/// The default ColumnDesc.
	default_col_desc: ColumnDesc<'a>,
}

impl<'a, R, T> Table<'a, R, T>
	where
		R: Row,
		T: Renderer,
{
	/// Constructs a new `TableBuilder` suitable for building this type of
	/// table.
	pub fn new_builder<I, S>(source: I, renderer: T) -> TableBuilder<'a, S, T>
		where
			I: IntoIterator<Item=R, IntoIter=S>,
			S: Iterator<Item=R>
	{
		TableBuilder::new(source, renderer)
	}

	/// Constructs a new `Table` from a collated data source and renderer.
	pub (in crate) fn new<S>(
		source: Collate<'a, S>,
		default_col_desc: ColumnDesc<'a>,
		renderer: T) -> Self
		where S: Iterator<Item=R>
	{
		let inner = Aggregate::new(source, &default_col_desc);
		Self {
			inner,
			renderer,
			default_col_desc,
		}
	}

	/// Renders the table output.
	pub fn render<W>(&mut self, out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		let rows = self.inner.rows();
		let mut ctx = RenderContext {
			default_col_desc: &self.default_col_desc,
			col_descs: self.inner.column_descs(),
			col_widths: self.inner.col_widths(),
			row_count: rows.len(),
			row: None,
			col: None,
			line: None,
		};

		self.renderer.init(&ctx);

		self.renderer.write_table_start(out, &ctx)?;
		// Write the header.
		if let Some(row) = self.inner.header_row().as_ref() {
			self.renderer.write_header_start(out, &ctx)?;
			Self::render_header_row(
				&mut self.renderer,
				&mut ctx,
				out,
				row)?;
			self.renderer.write_header_end(out, &ctx)?;
		}

		// Write the data.
		self.renderer.write_data_start(out, &ctx)?;
		for (row_idx, row) in rows.iter().enumerate() {
			ctx.row = Some(row_idx);
			Self::render_data_row(
				&mut self.renderer,
				&mut ctx,
				out,
				row)?;
		}
		self.renderer.write_data_end(out, &ctx)?;

		// Write the footer.
		if let Some(row) = self.inner.footer_row().as_ref() {
			self.renderer.write_footer_start(out, &ctx)?;
			Self::render_footer_row(
				&mut self.renderer,
				&mut ctx,
				out,
				row)?;
			self.renderer.write_footer_end(out, &ctx)?;
		}
		self.renderer.write_table_end(out, &ctx)?;

		Ok(())
	}

	/// Renders a row of the header.
	fn render_header_row<W>(
		renderer: &mut T,
		ctx: &mut RenderContext<'_>,
		out: &mut W,
		row: &TextRow<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		renderer.write_header_row_start(out, ctx)?;
		
		for line_idx in 0..row.height() {
			ctx.line = Some(line_idx);
			renderer.write_header_line_start(out, ctx)?;
			for col_idx in 0..row.len() {
				ctx.col = Some(col_idx);
				if line_idx == 0 {
					renderer.write_header_cell_start(out, ctx)?;
				}

				let desc = ctx.col_descs
					.get(col_idx)
					.unwrap_or(&ctx.default_col_desc);
				let text = row.line_vert_aligned(
					col_idx,
					line_idx,
					desc.vert_align);
				let cell = CellContext {
					text,
					width: ctx.col_widths[col_idx],
					desc,
				};
				renderer.write_header_cell_line_start(out, ctx)?;
				renderer.write_header_cell_line(out, ctx, &cell)?;
				renderer.write_header_cell_line_end(out, ctx)?;
				if line_idx == row.height() - 1 {
					renderer.write_header_cell_end(out, ctx)?;
				}
			}
			ctx.col = None;
			renderer.write_header_line_end(out, ctx)?;
		}
		ctx.line = None;

		renderer.write_header_row_end(out, ctx)?;
		Ok(())
	}

	/// Renders a row of the table data.
	fn render_data_row<W>(
		renderer: &mut T,
		ctx: &mut RenderContext<'_>,
		out: &mut W,
		row: &SplitRow<'_, R>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		renderer.write_data_row_start(out, ctx)?;
		
		for line_idx in 0..row.height() {
			ctx.line = Some(line_idx);
			renderer.write_data_line_start(out, ctx)?;
			for col_idx in 0..row.len() {
				ctx.col = Some(col_idx);
				if line_idx == 0 {
					renderer.write_data_cell_start(out, ctx)?;
				}

				let desc = ctx.col_descs
					.get(col_idx)
					.unwrap_or(&ctx.default_col_desc);
				let text = row.line_vert_aligned(
					col_idx,
					line_idx,
					desc.vert_align);
				let cell = CellContext {
					text,
					width: ctx.col_widths[col_idx],
					desc,
				};
				renderer.write_data_cell_line_start(out, ctx)?;
				renderer.write_data_cell_line(out, ctx, &cell)?;
				renderer.write_data_cell_line_end(out, ctx)?;
				if line_idx == row.height() - 1 {
					renderer.write_data_cell_end(out, ctx)?;
				}
			}
			ctx.col = None;
			renderer.write_data_line_end(out, ctx)?;
		}
		ctx.line = None;

		renderer.write_data_row_end(out, ctx)?;
		Ok(())
	}

	/// Renders a row of the footer.
	fn render_footer_row<W>(
		renderer: &mut T,
		ctx: &mut RenderContext<'_>,
		out: &mut W,
		row: &TextRow<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		renderer.write_footer_row_start(out, ctx)?;
		
		for line_idx in 0..row.height() {
			ctx.line = Some(line_idx);
			renderer.write_footer_line_start(out, ctx)?;
			for col_idx in 0..row.len() {
				ctx.col = Some(col_idx);
				if line_idx == 0 {
					renderer.write_footer_cell_start(out, ctx)?;
				}

				let desc = ctx.col_descs
					.get(col_idx)
					.unwrap_or(&ctx.default_col_desc);
				let text = row.line_vert_aligned(
					col_idx,
					line_idx,
					desc.vert_align);
				let cell = CellContext {
					text,
					width: ctx.col_widths[col_idx],
					desc,
				};
				renderer.write_footer_cell_line_start(out, ctx)?;
				renderer.write_footer_cell_line(out, ctx, &cell)?;
				renderer.write_footer_cell_line_end(out, ctx)?;
				if line_idx == row.height() - 1 {
					renderer.write_footer_cell_end(out, ctx)?;
				}
			}
			ctx.col = None;
			renderer.write_footer_line_end(out, ctx)?;
		}
		ctx.line = None;

		renderer.write_footer_row_end(out, ctx)?;
		Ok(())
	}
}
