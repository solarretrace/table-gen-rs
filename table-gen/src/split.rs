////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator row splitting module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::Cell;
use crate::FormatRow;
use crate::Row;
use crate::util::Style;
use crate::TextWrap;
use crate::WrapOptions;
use crate::VertAlign;

// External library imports.
use smallvec::SmallVec;
use textwrap::fill;

// Standard library imports.
use std::cell::OnceCell;
use std::rc::Rc;
use std::str::Lines;
use std::fmt::Write as _;


////////////////////////////////////////////////////////////////////////////////
// SplitRowStyle
////////////////////////////////////////////////////////////////////////////////
/// Post-aggregation column styling information.
#[derive(Clone)]
pub (in crate) struct SplitRowStyle {
	/// The final column widths.
	pub col_widths: Vec<usize>,
	/// The column text wrap settings.
	pub col_text_wraps: Vec<TextWrap>,
	/// The column text styling function.
	pub col_text_style_fn: Vec<Option<Rc<dyn Fn(&dyn Cell, usize) -> Style>>>,
	/// The default renderer text wrapping.
	pub default_renderer_wrap: Option<WrapOptions>,
	/// The default column text wraping.
	pub default_column_wrap: TextWrap,
	/// The default column text styling.
	pub default_text_style_fn: Option<Rc<dyn Fn(&dyn Cell, usize) -> Style>>,
}

impl std::fmt::Debug for SplitRowStyle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let col_text_style_fn_debug: Vec<_> = self.col_text_style_fn
			.iter()
			.map(|f| f.as_ref().map(Rc::as_ptr))
			.collect();
		f.debug_struct("SplitRowStyle")
			.field("col_widths", &self.col_widths)
			.field("col_text_wraps", &self.col_text_wraps)
			.field("col_text_style_fn", &col_text_style_fn_debug)
			.field("default_renderer_wrap", &self.default_renderer_wrap)
			.field("default_column_wrap", &self.default_column_wrap)
			.field("default_text_style_fn",
				&self.default_text_style_fn.as_ref().map(Rc::as_ptr))
         	.finish()
	}
}


impl SplitRowStyle {
	/// Returns `true` if any of the columns have wrapping or text styling
	/// enabled.
	#[must_use]
	pub (in crate) fn any_formatting_enabled(&self) -> bool {
		self.any_wrapping_enabled() || self.any_text_style_enabled()
	}

	/// Returns `true` if any of the columns have wrapping enabled.
	#[must_use]
	pub (in crate) fn any_wrapping_enabled(&self) -> bool {
		self.col_text_wraps
			.iter()
			.any(|wrap| wrap.is_enabled(
				self.default_renderer_wrap.as_ref(),
				&self.default_column_wrap))
			|| (self.col_text_wraps.len() < self.col_widths.len()
				&& self.default_column_wrap.is_enabled(
					self.default_renderer_wrap.as_ref(),
					&TextWrap::RendererDefault))
	}

	/// Returns `true` if any of the columns text styling enabled.
	#[must_use]
	pub (in crate) fn any_text_style_enabled(&self) -> bool {
		self.col_text_style_fn.iter().any(|ts| ts.is_some())
			|| (self.col_text_style_fn.len() < self.col_widths.len()
				&& self.default_text_style_fn.is_some())
	}

	/// Returns `true` if any of the columns have wrapping enabled.
	#[must_use]
	pub (in crate) fn apply(
		&self,
		text: &str,
		idx: usize,
		cell: Option<&dyn Cell>)
		-> Option<String>
	{
		// Apply line wrapping to the text, if needed.
		let width = self.col_widths[idx];
		let wrap = self.col_text_wraps.get(idx)
			.unwrap_or(&self.default_column_wrap);
		let opts = wrap.as_options(
			self.default_renderer_wrap.as_ref(),
			&self.default_column_wrap,
			width);
		let wrapped = opts.map(|o| fill(text, o));

		// Apply styling to each line if possible.
		match (
			cell,
			self.col_text_style_fn
				.get(idx)
				.and_then(Option::as_deref)
				.or(self.default_text_style_fn.as_deref())) 
		{
			(Some(cell), Some(style_fn)) => {
				let text = wrapped.as_deref().unwrap_or(text);
				if text.is_empty() { return None; }

				let mut out = String::with_capacity(text.len() + 16);
				let style = (style_fn)(cell, idx);
				for line in text.lines() {
					write!(&mut out, "{}{}{}",
							style.render(),
							line,
							style.render_reset())
						.expect("write styled line");
					println!("{:?}", out);
				}
				Some(out)
			},
			_ => wrapped,
		}
	}

}


////////////////////////////////////////////////////////////////////////////////
// SplitRow
////////////////////////////////////////////////////////////////////////////////
/// A single table row with line splitting.
#[allow(missing_debug_implementations)]
pub (in crate) struct SplitRow<'a, R> {
	/// The row to format.
	inner: FormatRow<'a, R>,
	/// The cached wrapped cell texts.
	cache: Vec<OnceCell<Box<str>>>,
	/// The maximum number of lines in the row.
	height: usize,
}

impl<R> Row for SplitRow<'_, R>
	where R: Row
{
	fn len(&self) -> usize {
		self.inner.len()
	}

	fn cell(&self, col_idx: usize) -> Option<&dyn Cell> {
		self.inner.cell(col_idx)
	}
}

impl<'a, R> SplitRow<'a, R>
	where R: Row,
{
	/// Returns a new `SplitRow` over the given `FormatRow`.
	#[must_use]
	pub (in crate) fn new(inner: FormatRow<'a, R>, style: &SplitRowStyle)
		-> Self
	{
		let cache = if style.any_formatting_enabled() {
			vec![OnceCell::new(); inner.len()]
		} else {
			Vec::new()
		};
		let mut height = 0;
		for idx in 0..inner.len() {
			let text = if let Some(formatted) = style
				.apply(inner.text(idx), idx, inner.cell(idx))
			{
				cache[idx].get_or_init(|| formatted.into_boxed_str())
			} else {
				inner.text(idx)
			};
			height = std::cmp::max(height, text.lines().count());
		}
		Self {
			inner,
			cache,
			height,
		}
	}

	/// Returns the maximum number of lines to render in this row.
	#[must_use]
	pub (in crate) fn height(&self) -> usize {
		self.height
	}

	/// Returns the inner text value of the cell with the given column index.
	#[must_use]
	pub (in crate) fn text_inner(&self, col_idx: usize) -> &str {
		self.inner.text(col_idx)
	}

	/// Returns the post-width-formatted text of the cell with the given column
	/// index.
	#[must_use]
	pub (in crate) fn text(&self, col_idx: usize) -> &str {
		self.cache
			.get(col_idx)
			.and_then(OnceCell::get)
			.map(std::ops::Deref::deref)
			.unwrap_or_else(|| self.text_inner(col_idx))
	}

	/// Returns an iterator over the lines of the cell with the given column
	/// index.
	pub (in crate) fn lines(&self, col_idx: usize) -> Lines<'_> {
		self.text(col_idx).lines()
	}
	
	/// Returns the text of the line at the given column & line index, after
	/// vertically aligning it as specified.
	#[must_use]
	pub (in crate) fn line_vert_aligned(
		&self,
		col_idx: usize,
		line_idx: usize,
		vert_align: VertAlign)
		-> &str
	{
		vert_align_from_iter(
			self.lines(col_idx),
			line_idx,
			self.height,
			vert_align)
	}
}


////////////////////////////////////////////////////////////////////////////////
// TextRow
////////////////////////////////////////////////////////////////////////////////
/// A single table row containing text cells with line splitting.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub (in crate) struct TextRow<'a> {
	/// The text of the row's columns.
	inner: Vec<&'a str>,
	/// The maximum number of lines in the row.
	height: usize,
	/// The maximum number of columns in the row.
	len: usize,
}

impl<'a> TextRow<'a> {
	/// Returns a new `TextRow` over the given strings.
	#[must_use]
	pub (in crate) fn new(inner: Vec<&'a str>) -> Self {
		let len = inner.len();
		let height = (0..len)
			.map(|c| inner[c].lines().count())
			.max()
			.unwrap_or(0);
		Self {
			inner,
			height,
			len,
		}
	}

	/// Sets the overall length of the row, ensuring empty strings are returned
	/// for column indices beyond those already provided.
	#[must_use]
	pub (in crate) fn with_len(mut self, len: usize) -> Self {
		self.len = len;
		self
	}

	/// Returns the maximum number of lines to render in this row.
	#[must_use]
	pub (in crate) fn height(&self) -> usize {
		self.height
	}
	
	/// Returns the number of columns in the row.
	#[must_use]
	pub (in crate) fn len(&self) -> usize {
		self.len
	}
	
	/// Returns the text of the cell with the given column index.
	#[must_use]
	pub (in crate) fn text(&self, col_idx: usize) -> &str {
		self.inner
			.get(col_idx)
			.map_or("", |t| t)
	}

	/// Returns an iterator over the lines of the cell with the given column
	/// index.
	pub (in crate) fn lines(&self, col_idx: usize) -> Lines<'_> {
		self.text(col_idx).lines()
	}
	
	/// Returns the text of the line at the given column & line index, after
	/// vertically aligning it as specified.
	#[must_use]
	pub (in crate) fn line_vert_aligned(
		&self,
		col_idx: usize,
		line_idx: usize,
		vert_align: VertAlign)
		-> &str
	{
		vert_align_from_iter(
			self.lines(col_idx),
			line_idx,
			self.height,
			vert_align)
	}
}


////////////////////////////////////////////////////////////////////////////////
// SplitRow
////////////////////////////////////////////////////////////////////////////////
/// Vertically aligns text from a `Lines` iterator.
fn vert_align_from_iter(
	mut lines: Lines<'_>,
	line_idx: usize,
	height: usize,
	vert_align: VertAlign)
	-> &str
{
	match vert_align {
		VertAlign::Top    => lines.nth(line_idx),
		VertAlign::Center => {
			let lines: SmallVec<[&str; 3]> = lines.collect();
			let offset = height.saturating_sub(lines.len()) / 2;
			line_idx.checked_sub(offset)
				.and_then(|idx| lines.get(idx))
				.map(|v| &**v)
		},
		VertAlign::Bottom => {
			let lines: SmallVec<[&str; 3]> = lines.collect();
			let offset = height.saturating_sub(lines.len());
			line_idx.checked_sub(offset)
				.and_then(|idx| lines.get(idx))
				.map(|v| &**v)
		},
	}.unwrap_or("")
}
