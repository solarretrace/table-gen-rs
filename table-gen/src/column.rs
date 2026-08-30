////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator column definition types.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::Cell;
use crate::DisplayFmt;
use crate::util::Style;
use crate::Wrap;

// Standard library imports.
use std::rc::Rc;


////////////////////////////////////////////////////////////////////////////////
// Alignment
////////////////////////////////////////////////////////////////////////////////
/// Horizontal alignment specifier for cell text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HorzAlign {
	/// Align to the left of the cell.
	Left,
	/// Align to the center of the cell.
	Center,
	/// Align to the right of the cell.
	Right,
}

/// Vertical alignment specifier for cell text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertAlign {
	/// Align to the top of the cell.
	Top,
	/// Align to the center of the cell.
	Center,
	/// Align to the bottom of the cell.
	Bottom,
}

////////////////////////////////////////////////////////////////////////////////
// ColumnDef
////////////////////////////////////////////////////////////////////////////////
/// Provides behavior for formatting and handling table columns.
///
/// The effectiveness of the available options depends on the specific
/// `Renderer` used: renderers must provide the table driver with the
/// appropriate `SupportFlags` to ensure the `ColumnDef` options are exposed.
/// E.g., if the renderer does not provide the `MULTILINE` flag, then cell text
/// will have its line endings stripped before being provided to the renderer
/// for output.
#[derive(Clone)]
pub struct ColumnDef<'a> {
	/// The `DisplayFmt` to use for cells in this column.
	pub display_fmt: DisplayFmt,
	/// The column header text.
	pub header: &'a str,
	/// The column footer text.
	pub footer: &'a str,
	/// The minimum width of the column.
	pub min_width: usize,
	/// The maximum width of the column.
	pub max_width: usize,
	/// The quantile of the widest cell values to ignore for computing dynamic
	/// column widths.
	pub dynamic_width_quantile: f64,
	/// The relative weight of the column in allocating width under constraint.
	pub dynamic_width_weight: f64,
	/// The horizontal alignment of text in the column.
	pub horz_align: HorzAlign,
	/// The vertical alignment of text in the column.
	pub vert_align: VertAlign,
	/// The text wrap options.
	pub text_wrap: Wrap,
	/// The text styling function for the cell values.
	pub text_style_fn: Option<Rc<dyn Fn(&dyn Cell, usize) -> Style>>,
}

impl std::fmt::Debug for ColumnDef<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ColumnDef")
			.field("header", &self.header)
			.field("footer", &self.footer)
			.field("min_width", &self.min_width)
			.field("max_width", &self.max_width)
			.field("dynamic_width_quantile", &self.dynamic_width_quantile)
			.field("dynamic_width_weight", &self.dynamic_width_weight)
			.field("horz_align", &self.horz_align)
			.field("vert_align", &self.vert_align)
			.field("text_wrap", &self.text_wrap)
			.field("text_style_fn",
				&self.text_style_fn.as_ref().map(Rc::as_ptr))
         	.finish()
	}
}
	
impl Default for ColumnDef<'_> {
	fn default() -> Self {
		Self::new()
	}
}

impl<'a> ColumnDef<'a> {
	/// Constructs a new `ColumnDef` with a default value.
	#[must_use]
	pub fn new() -> Self {
		Self {
			display_fmt: DisplayFmt::new(),
			header: "",
			footer: "",
			min_width: 0,
			max_width: usize::MAX,
			dynamic_width_quantile: 1.0,
			dynamic_width_weight: 1.0,
			horz_align: HorzAlign::Left,
			vert_align: VertAlign::Top,
			text_wrap: Wrap::default(),
			text_style_fn: None,
		}
	}
	
	/// Sets the header text and returns the `ColumnDef`.
	#[must_use]
	pub fn with_header(mut self, header: &'a str) -> Self {
		self.header = header;
		self
	}
	
	/// Sets the footer text and returns the `ColumnDef`.
	#[must_use]
	pub fn with_footer(mut self, footer: &'a str) -> Self {
		self.footer = footer;
		self
	}
	
	/// Sets the `DisplayFmt` and returns the `ColumnDef`.
	#[must_use]
	pub fn with_display_fmt(mut self, display_fmt: DisplayFmt) -> Self {
		self.display_fmt = display_fmt;
		self
	}
	
	/// Sets the minimum and maximum column widths and returns the `ColumnDef`.
	#[must_use]
	pub fn with_width(mut self, width: usize) -> Self {
		self.min_width = width;
		self.max_width = width;
		self
	}
	
	/// Sets the minimum column widths and returns the `ColumnDef`.
	#[must_use]
	pub fn with_min_width(mut self, min_width: usize) -> Self {
		self.min_width = min_width;
		self
	}
	
	/// Sets the maximum column widths and returns the `ColumnDef`.
	#[must_use]
	pub fn with_max_width(mut self, max_width: usize) -> Self {
		self.max_width = max_width;
		self
	}

	/// Sets the quantile of the widest cell values to ignore for computing
	/// dynamic column widths and returns the `ColumnDef`.
	///
	/// I.e., a value of `0.9` means that approximately the longest 10% of
	/// column values will be ignored for purposes of computing column width.
	#[must_use]
	pub fn with_dynamic_width_quantile(mut self, dynamic_width_quantile: f64)
		-> Self
	{
		self.dynamic_width_quantile = dynamic_width_quantile;
		self
	}

	/// Sets the relative weight of the column when computing dynamic column
	/// widths and returns the `ColumnDef`.
	///
	/// The default weight is 1.0. If columns need to have their width reduced,
	/// columns with higher weight will have less width removed.
	#[must_use]
	pub fn with_dynamic_width_weight(mut self, dynamic_width_weight: f64)
		-> Self
	{
		self.dynamic_width_weight = dynamic_width_weight;
		self
	}
	
	/// Sets the horizontal text alignment and returns the `ColumnDef`.
	#[must_use]
	pub fn with_horz_align(mut self, horz_align: HorzAlign) -> Self {
		self.horz_align = horz_align;
		self
	}
	
	/// Sets the vertical text alignment and returns the `ColumnDef`.
	#[must_use]
	pub fn with_vert_align(mut self, vert_align: VertAlign) -> Self {
		self.vert_align = vert_align;
		self
	}
	
	/// Sets text wrap mode and returns the `ColumnDef`.
	#[must_use]
	pub fn with_text_wrap(mut self, text_wrap: Wrap) -> Self {
		self.text_wrap = text_wrap;
		self
	}
	
	/// Sets text style function and returns the `ColumnDef`.
	#[must_use]
	pub fn with_text_style_fn(
		mut self,
		text_style_fn: Option<Rc<dyn Fn(&dyn Cell, usize) -> Style>>)
		-> Self
	{
		self.text_style_fn = text_style_fn;
		self
	}

	/// Returns `true` if the column width is fully constrained.
	pub fn is_fixed_width(&self) -> bool {
		self.min_width >= self.max_width
	}

	/// Clamps the given value between the min and max width allowed for the
	/// column.
	pub fn clamp_to_valid_width(&self, value: usize) -> usize {
		value.clamp(self.min_width, self.max_width)
	}
}


////////////////////////////////////////////////////////////////////////////////
// ColumnDefs
////////////////////////////////////////////////////////////////////////////////
/// A collection of `ColumnDef`s.
#[derive(Debug, Clone)]
pub struct ColumnDefs<'a> {
	/// Extra width 
	extra_column_width: usize,
	/// The user-provided `ColumnDef`s list.
	columns: &'a [ColumnDef<'a>],
	/// The `ColumnDef` to use for columns not provided in the above list.
	column_default: ColumnDef<'a>,
}

impl Default for ColumnDefs<'_> {
	fn default() -> Self {
		Self::new()
	}
}

impl<'a> ColumnDefs<'a>  {
	/// Constructs a new `ColumnDefs`.
	#[must_use]
	pub fn new() -> Self {
		Self {
			extra_column_width: 0,
			columns: &[],
			column_default: ColumnDef::new()
				.with_text_wrap(Wrap::RendererDefault),
		}
	}

	/// Constructs a new `ColumnDefs` from its components.
	#[must_use]
	pub fn from_parts(
		column_default: ColumnDef<'a>,
		columns: &'a [ColumnDef<'a>],
		extra_column_width: usize)
		-> Self
	{
		Self {
			column_default,
			columns,
			extra_column_width,
		}
	}

	/// Sets the column_default `ColumnDef` and returns the `ColumnDefs`.
	#[must_use]
	pub fn with_column_default(
		mut self,
		column_default: ColumnDef<'a>)
		-> Self
	{
		self.column_default = column_default;
		self
	}

	/// Sets the `ColumnDef` for each column and returns the `ColumnDefs`.
	#[must_use]
	pub fn with_columns(
		mut self,
		columns: &'a [ColumnDef<'a>])
		-> Self
	{
		self.columns = columns;
		self
	}

	/// Sets the extra column width and returns the `ColumnDefs`.
	#[must_use]
	pub fn with_extra_column_width(
		mut self,
		extra_column_width: usize)
		-> Self
	{
		self.extra_column_width = extra_column_width;
		self
	}

	/// Consumes the `ColumnDefs` and returns its components.
	#[must_use]
	pub fn into_parts(self)
		-> (ColumnDef<'a>, &'a [ColumnDef<'a>], usize)
	{
		(
			self.column_default,
			self.columns,
			self.extra_column_width,
		)
	}

	/// The number of non-column_default `ColumnDef`s defined.
	#[must_use]
	pub fn len(&self) -> usize {
		self.columns.len()
	}

	/// Returns a reference to the column_default `ColumnDef`.
	#[must_use]
	pub fn column_default(&self) -> &ColumnDef<'a> {
		&self.column_default
	}

	/// Returns a mutable reference to the column_default `ColumnDef`.
	#[must_use]
	pub fn column_default_mut(&mut self) -> &mut ColumnDef<'a> {
		&mut self.column_default
	}

	/// Returns a reference to the `ColumnDef` for the column at the given
	/// index.
	#[must_use]
	pub fn get(&self, idx: usize) -> &ColumnDef<'a> {
		self.columns.get(idx).unwrap_or(&self.column_default)
	}

	/// Returns a reference to the `ColumnDef`s array.
	#[must_use]
	pub fn columns(&self) -> &'a [ColumnDef<'a>] {
		self.columns
	}

	/// Returns a mutable reference to the `ColumnDef`s array.
	#[must_use]
	pub fn columns_mut(&mut self) -> &mut &'a [ColumnDef<'a>] {
		&mut self.columns
	}

	/// Returns the extra column width.
	#[must_use]
	pub fn extra_column_width(&self) -> usize {
		self.extra_column_width
	}

	/// Returns `true` if there are no headers to render for column indices up
	/// to the given index.
	#[must_use]
	pub fn is_headerless(&self, upto: usize) -> bool {
		self.columns
				.iter()
				.take(upto)
				.all(|column_def| column_def.header.is_empty())
			&& upto <= self.len()
			|| ( upto > self.len() && self.column_default.header.is_empty())
	}

	/// Returns `true` if there are no footers to render for column indices up
	/// to the given index.
	#[must_use]
	pub fn is_footerless(&self, upto: usize) -> bool {
		self.columns
				.iter()
				.take(upto)
				.all(|column_def| column_def.footer.is_empty())
			&& upto <= self.len()
			|| ( upto > self.len() && self.column_default.footer.is_empty())
	}

	/// Returns the column header text for the column at the given index.
	#[must_use]
	pub fn header(&self, idx: usize) -> &'a str {
		self.columns
			.get(idx)
			.unwrap_or(&self.column_default)
			.header
	}

	/// Returns the column footer text for the column at the given index.
	#[must_use]
	pub fn footer(&self, idx: usize) -> &'a str {
		self.columns
			.get(idx)
			.unwrap_or(&self.column_default)
			.footer
	}

	/// Returns the `DisplayFmt` to use for cells in the column at the given
	/// index.
	#[must_use]
	pub fn display_fmt(&self, idx: usize) -> DisplayFmt {
		self.columns
			.get(idx)
			.unwrap_or(&self.column_default)
			.display_fmt
	}

	/// Returns the minimum width of the column at the given index.
	#[must_use]
	pub fn min_width(&self, idx: usize) -> usize {
		let w = self.columns
			.get(idx)
			.unwrap_or(&self.column_default)
			.min_width;
		self.extra_column_width.saturating_add(w)
	}

	/// Returns the maximum width of the column at the given index.
	#[must_use]
	pub fn max_width(&self, idx: usize) -> usize {
		let w = self.columns
			.get(idx)
			.unwrap_or(&self.column_default)
			.max_width;

		self.extra_column_width.saturating_add(w)
	}

	/// Returns the quantile of the widest cell values to ignore for computing
	/// dynamic column widths for the column at the given index.
	#[must_use]
	pub fn dynamic_width_quantile(&self, idx: usize) -> f64 {
		self.columns
			.get(idx)
			.unwrap_or(&self.column_default)
			.dynamic_width_quantile
	}

	/// Returns the relative weight of the column in allocating width under
	/// constraint for the column at the given index.
	#[must_use]
	pub fn dynamic_width_weight(&self, idx: usize) -> f64 {
		self.columns
			.get(idx)
			.unwrap_or(&self.column_default)
			.dynamic_width_weight
	}

	/// Returns the horizontal alignment of text in the column at the given
	/// index.
	#[must_use]
	pub fn horz_align(&self, idx: usize) -> HorzAlign {
		self.columns
			.get(idx)
			.unwrap_or(&self.column_default)
			.horz_align
	}

	/// Returns the vertical alignment of text in the column at the given index.
	#[must_use]
	pub fn vert_align(&self, idx: usize) -> VertAlign {
		self.columns
			.get(idx)
			.unwrap_or(&self.column_default)
			.vert_align
	}

	/// Returns `true` if the column width is fully constrained for the column
	/// at the given index.
	#[must_use]
	pub fn is_fixed_width(&self, idx: usize) -> bool {
		self.columns
			.get(idx)
			.unwrap_or(&self.column_default)
			.is_fixed_width()
	}

	/// Clamps the given value between the min and max width allowed for the
	/// column at the given index.
	#[must_use]
	pub fn clamp_to_valid_width(&self, idx: usize, value: usize)
		-> usize
	{
		self.columns
			.get(idx)
			.unwrap_or(&self.column_default)
			.clamp_to_valid_width(value)
	}
}
