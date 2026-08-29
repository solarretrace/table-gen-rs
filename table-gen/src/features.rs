////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Renderer features module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::util::unicode_display_width;
use crate::util::WrapOptions;

// External library imports.
use bitflags::bitflags;

// Standard library imports.
use std::rc::Rc;


////////////////////////////////////////////////////////////////////////////////
// SupportFlags
////////////////////////////////////////////////////////////////////////////////
bitflags! {
	/// Renderer feature support flags.
	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[repr(transparent)]
	pub struct SupportFlags: u32 {
		/// Indicates that the renderer supports header rendering.
		const HEADERS                  = 1;
		/// Indictaes that the renderer supports footer rendering.
		const FOOTERS                  = 1 << 1;
		/// Indicates that the renderer supports column widths.
		const COLUMN_WIDTH             = 1 << 2;
		/// Indicates that the renderer supports column widths constraints.
		const COLUMN_WIDTH_CONSTRAINTS = 1 << 3;
		/// Indicates that the renderer supports dynamic column widths.
		const COLUMN_WIDTH_DYNAMIC     = 1 << 4;
		/// Indicates that the renderer supports horizontal column alignment.
		const HORZ_ALIGN               = 1 << 5;
		/// Indicates that the renderer supports multiline rows & cells.
		const MULTILINE                = 1 << 6;
		/// Indicates that the renderer supports vertical alignment of multiline
		/// rows and cells.
		const VERT_ALIGN               = 1 << 7;
		/// Indicates that the renderer supports text wrapping.
		const TEXT_WRAP                = 1 << 8;
		/// Indicates that the renderer supports ANSI styling.
		const ANSI_STYLE               = 1 << 9;

		const COLUMN_WIDTH_ALL = Self::COLUMN_WIDTH.bits()
			| Self::COLUMN_WIDTH_CONSTRAINTS.bits()
			| Self::COLUMN_WIDTH_DYNAMIC.bits()
			| Self::HORZ_ALIGN.bits();

		const MULTILINE_ALL = Self::MULTILINE.bits()
			| Self::VERT_ALIGN.bits()
			| Self::TEXT_WRAP.bits();
	}
}

impl Default for SupportFlags {
	fn default() -> Self {
		Self::new()
	}
}

impl SupportFlags {
	/// Constructs a new `SupportFlags` with only the `HEADERS` flag set.
	pub const fn new() -> Self {
		Self::HEADERS
	}
}

////////////////////////////////////////////////////////////////////////////////
// Features
////////////////////////////////////////////////////////////////////////////////
/// `Renderer` feature settings. Provides customized behavior for the table
/// renderer driver to support renderer-specific requirements.
#[allow(missing_copy_implementations)]
#[allow(missing_debug_implementations)]
pub struct Features {
	/// The supported driver behavior flags.
	pub flags: SupportFlags,

	/// Function to use for calculating column widths. A `None` value means
	/// column widths should not be calculated
	pub str_width_fn: Option<fn(&str) -> usize>,

	/// Function for computing the table width contribution of the renderer. It
	/// will be provided the number of columns being rendered.
	pub width_contribution_fn: Option<Rc<dyn Fn(usize) -> usize>>,

	/// Extra width to pad all columns.
	pub extra_column_width: usize,

	/// Default column width for underconstrained columns if dynamic column
	/// widths are unsupported.
	pub default_column_width: usize,

	/// The default text wrapping option.
	pub default_text_wrap: Option<WrapOptions>,
}

impl std::fmt::Debug for Features {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Features")
			.field("flags", &self.flags)
			.field("str_width_fn", &self.str_width_fn)
			.field("width_contribution_fn", 
				&self.width_contribution_fn.as_ref().map(Rc::as_ptr))
			.field("extra_column_width", &self.extra_column_width)
			.field("default_column_width", &self.default_column_width)
			.field("default_text_wrap", &self.default_text_wrap)
         	.finish()
	}
}

impl Features {
	/// Contstructs a new `Features` with the given `SupportFlags`.
	#[must_use]
	pub fn new(flags: SupportFlags) -> Self {
		Self {
			flags,
			str_width_fn: Some(unicode_display_width),
			width_contribution_fn: Some(
				Rc::new(Self::interspersed_space_width)),
			extra_column_width: 0,
			default_column_width: 15,
			default_text_wrap: flags
				.contains(SupportFlags::MULTILINE | SupportFlags::TEXT_WRAP)
				.then(|| WrapOptions::new())
		}
	}

	/// Sets the function to use for calculating column widths and returns the 
	/// `Features`.
	///
	/// The following functions are available as suitable arguments:
	/// + `Features::unicode_display_width` is the default value.
	/// + `str::len` is suitable if the inputs are always ASCII.
	#[must_use]
	pub fn with_str_width_fn(mut self, str_width_fn: Option<fn(&str) -> usize>)
		-> Self
	{
		self.str_width_fn = str_width_fn;
		self
	}

	/// Sets the function to use for computing the irreducable portion of the
	/// table width, then returns the `Features`. This should match to total all
	/// border widths across the provided number of rows.
	#[must_use]
	pub fn with_width_contribution_fn(
		mut self,
		width_contribution_fn: Rc<dyn Fn(usize) -> usize>)
		-> Self
	{
		self.width_contribution_fn = Some(width_contribution_fn);
		self
	}

	/// Sets the extra column width for each table column and returns the
	/// `Features`.
	#[must_use]
	pub fn with_extra_column_width(mut self, extra_column_width: usize) -> Self
	{
		self.extra_column_width = extra_column_width;
		self
	}

	/// Sets the default column width for under-constrained columns if dynamic
	/// column widths are unsupported, then returns the `Features`.
	#[must_use]
	pub fn with_default_column_width(mut self, default_column_width: usize)
		-> Self
	{
		self.default_column_width = default_column_width;
		self
	}

	/// Sets the default text wrapping options, then returns the `Features`.
	#[must_use]
	pub fn with_default_text_wrap<W>(
		mut self,
		default_text_wrap: W)
		-> Self
		where W: Into<Option<WrapOptions>>
	{
		self.default_text_wrap = default_text_wrap.into();
		self
	}

	fn interspersed_space_width(col_count: usize) -> usize {
		col_count.saturating_sub(1)
	}
}
