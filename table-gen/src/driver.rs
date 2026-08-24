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
use crate::ColumnDef;
use crate::ColumnOrd;
use crate::RenderContext;
use crate::CellContext;
use crate::Diagnostic;

// Standard library imports.
use std::ops::RangeBounds;


////////////////////////////////////////////////////////////////////////////////
// TableBuilder
////////////////////////////////////////////////////////////////////////////////
/// A builder-style constructor for a `Table`.
#[allow(missing_debug_implementations)]
pub struct TableBuilder<'a, S, T> {
	/// The table data source.
	inner: Collate<'a, S>,
	/// The table renderer.
	renderer: T,
	/// The default `ColumnDef`.
	default_column_def: ColumnDef<'a>,
	/// The minimum table width.
	min_table_width: usize,
	/// The maximum table width.
	max_table_width: usize,
	/// The diagnostic sink function.
	diagnostic_sink_fn: Box<dyn FnMut(Diagnostic) + 'static>,
}

impl<'a, R, S, T> TableBuilder<'a, S, T>
	where
		R: Row,
		S: Iterator<Item=R>,
		T: Renderer,
{
	/// Constructs a new `TableBuilder`.
	#[must_use]
	pub fn new<I>(source: I, renderer: T) -> Self
		where I: IntoIterator<Item=R, IntoIter=S>
	{
		let features = renderer.features();
		Self {
			inner: Collate::new(source.into_iter()).with_features(features),
			renderer,
			default_column_def: ColumnDef::new(),
			min_table_width: 0,
			max_table_width: usize::MAX,
			diagnostic_sink_fn: Box::new(|_| {/* Do nothing. */})
		}
	}

	/// Sets the output column selection and returns the `TableBuilder`.
	#[must_use]
	pub fn with_column_selection(mut self, col_select: &'a [usize]) -> Self {
		self.inner = self.inner.with_column_selection(col_select);
		self
	}

	/// Sets th output row selection and returns the `TableBuilder`.
	#[must_use]
	pub fn with_row_selection<B>(mut self, row_select: B) -> Self 
		where B: RangeBounds<usize>
	{
		self.inner = self.inner.with_row_selection(row_select);
		self
	}

	/// Sets the column descriptors for each column and returns the
	/// `TableBuilder`.
	#[must_use]
	pub fn with_column_defs(mut self, column_defs: &'a [ColumnDef<'a>]) -> Self
	{
		self.inner = self.inner.with_column_defs(column_defs);
		self
	}

	/// Sets the default column descriptor and returns the `TableBuilder`.
	#[must_use]
	pub fn with_default_column_def(mut self, default_column_def: ColumnDef<'a>)
		-> Self
	{
		self.default_column_def = default_column_def;
		self
	}

	/// Sets the sort columns and returns the `TableBuilder`.
	#[must_use]
	pub fn with_sort_columns(mut self, col_order: &'a [ColumnOrd]) -> Self {
		self.inner = self.inner.with_sort_columns(col_order);
		self
	}

	/// Sets the minimum table width and returns the `TableBuilder`.
	#[must_use]
	pub fn with_min_table_width(mut self, min_table_width: usize) -> Self {
		self.min_table_width = min_table_width;
		self
	}

	/// Sets the maximum table width and returns the `TableBuilder`.
	#[must_use]
	pub fn with_max_table_width(mut self, max_table_width: usize) -> Self {
		self.max_table_width = max_table_width;
		self
	}

	/// Sets the maximum table width and returns the `TableBuilder`.
	#[must_use]
	pub fn with_diagnostic_sink_fn<F>(mut self, diagnostic_sink_fn: F) -> Self 
		where F: FnMut(Diagnostic) + 'static
	{
		self.diagnostic_sink_fn = Box::new(diagnostic_sink_fn);
		self
	}

	/// Finishes collation of the data source and returns a `Table` for
	/// rendering.
	#[must_use]
	pub fn finish(self) -> Table<'a, R, T> {
		assert!(self.min_table_width <= self.max_table_width,
			"invalid table width constraints: max < min");
		Table::new(
			self.inner,
			self.renderer,
			self.default_column_def,
			self.diagnostic_sink_fn,
			self.min_table_width,
			self.max_table_width)
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
#[allow(missing_debug_implementations)]
pub struct Table<'a, R, T> {
	/// The table data source.
	inner: Aggregate<'a, R>,
	/// The table renderer.
	renderer: T,
	/// The default `ColumnDef`.
	default_column_def: ColumnDef<'a>,
	/// The diagnostic sink function.
	diagnostic_sink_fn: Box<dyn FnMut(Diagnostic) + 'static>,
}

impl<'a, R, T> Table<'a, R, T>
	where
		R: Row,
		T: Renderer,
{
	/// Constructs a new `TableBuilder` suitable for building this type of
	/// table.
	#[must_use]
	pub fn new_builder<I, S>(source: I, renderer: T) -> TableBuilder<'a, S, T>
		where
			I: IntoIterator<Item=R, IntoIter=S>,
			S: Iterator<Item=R>
	{
		TableBuilder::new(source, renderer)
	}

	/// Constructs a new `Table` from a collated data source and renderer.
	#[must_use]
	pub (in crate) fn new<S>(
		source: Collate<'a, S>,
		renderer: T,
		default_column_def: ColumnDef<'a>,
		mut diagnostic_sink_fn: Box<dyn FnMut(Diagnostic) + 'static>,
		min_table_width: usize,
		max_table_width: usize) -> Self
		where S: Iterator<Item=R>
	{
		let inner = Aggregate::new(
			source,
			&default_column_def,
			min_table_width,
			max_table_width,
			&mut diagnostic_sink_fn);
		Self {
			inner,
			renderer,
			default_column_def,
			diagnostic_sink_fn,
		}
	}

	/// Renders the table output.
	///
	/// # Errors
	///
	/// Writing to the provided output may generate I/O errors. See the 
	/// [std library docs](https://doc.rust-lang.org/std/io/trait.Write.html#errors)
	/// for details.
	pub fn render<W>(&mut self, out: &mut W)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		let str_width_fn = self.inner.str_width_fn();
		let rows = self.inner.rows();
		let mut ctx = RenderContext {
			default_column_def: &self.default_column_def,
			column_defs: self.inner.column_defs(),
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
				row,
				str_width_fn)?;
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
				row,
				str_width_fn)?;
		}
		self.renderer.write_data_end(out, &ctx)?;

		// Write the footer.
		if let Some(row) = self.inner.footer_row().as_ref() {
			self.renderer.write_footer_start(out, &ctx)?;
			Self::render_footer_row(
				&mut self.renderer,
				&mut ctx,
				out,
				row,
				str_width_fn)?;
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
		row: &TextRow<'_>,
		str_width_fn: Option<fn(&str) -> usize>)
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

				let desc = ctx.column_defs
					.get(col_idx)
					.unwrap_or(ctx.default_column_def);
				let text = row.line_vert_aligned(
					col_idx,
					line_idx,
					desc.vert_align);
				let cell = CellContext {
					text,
					text_width: str_width_fn.map(|f| (f)(text)),
					cell_width: ctx.col_widths[col_idx],
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
		row: &SplitRow<'_, R>,
		str_width_fn: Option<fn(&str) -> usize>)
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

				let desc = ctx.column_defs
					.get(col_idx)
					.unwrap_or(ctx.default_column_def);
				let text = row.line_vert_aligned(
					col_idx,
					line_idx,
					desc.vert_align);
				let cell = CellContext {
					text,
					text_width: str_width_fn.map(|f| (f)(text)),
					cell_width: ctx.col_widths[col_idx],
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
		row: &TextRow<'_>,
		str_width_fn: Option<fn(&str) -> usize>)
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

				let desc = ctx.column_defs
					.get(col_idx)
					.unwrap_or(ctx.default_column_def);
				let text = row.line_vert_aligned(
					col_idx,
					line_idx,
					desc.vert_align);
				let cell = CellContext {
					text,
					text_width: str_width_fn.map(|f| (f)(text)),
					cell_width: ctx.col_widths[col_idx],
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
