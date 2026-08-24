////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator column definition types.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::DisplayFmt;


////////////////////////////////////////////////////////////////////////////////
// ColumnDef
////////////////////////////////////////////////////////////////////////////////
/// Provides formatting and metadata for a table column.
#[derive(Debug, Clone, Copy)]
pub struct ColumnDef<'a> {
    /// The column header text.
    pub header: &'a str,
    /// The column footer text.
    pub footer: &'a str,
    /// The `DisplayFmt` to use for cells in this column.
    pub display_fmt: DisplayFmt,
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
            header: "",
            footer: "",
            display_fmt: DisplayFmt::new(),
            min_width: 0,
            max_width: usize::MAX,
            dynamic_width_quantile: 1.0,
            dynamic_width_weight: 1.0,
            horz_align: HorzAlign::Left,
            vert_align: VertAlign::Top,
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
// ColumnDefs
////////////////////////////////////////////////////////////////////////////////
pub (in crate) struct ColumnDefs<'a, 'b> {
    default_column_def: &'a ColumnDef<'b>,
    column_defs: &'a [ColumnDef<'b>],
    extra_column_width: usize,
}

impl<'a, 'b> ColumnDefs<'a, 'b> 
    where 'b: 'a
{
    #[must_use]
    pub (in crate) fn new(
        default_column_def: &'a ColumnDef<'b>,
        column_defs: &'a [ColumnDef<'b>],
        extra_column_width: usize)
        -> Self
    {
        Self {
            default_column_def,
            column_defs,
            extra_column_width,
        }
    }

    #[must_use]
    pub (in crate) fn into_parts(self)
        -> (&'a ColumnDef<'b>, &'a [ColumnDef<'b>], usize)
    {
        (
            self.default_column_def,
            self.column_defs,
            self.extra_column_width,
        )
    }

    /// The column header text.
    #[must_use]
    pub (in crate) fn header(&self, idx: usize) -> &'b str {
        self.column_defs
            .get(idx)
            .unwrap_or(self.default_column_def)
            .header
    }

    /// The column footer text.
    #[must_use]
    pub (in crate) fn footer(&self, idx: usize) -> &'b str {
        self.column_defs
            .get(idx)
            .unwrap_or(self.default_column_def)
            .footer
    }

    /// The `DisplayFmt` to use for cells in this column.
    #[must_use]
    pub (in crate) fn display_fmt(&self, idx: usize) -> DisplayFmt {
        self.column_defs
            .get(idx)
            .unwrap_or(self.default_column_def)
            .display_fmt
    }

    /// The minimum width of the column.
    #[must_use]
    pub (in crate) fn min_width(&self, idx: usize) -> usize {
        let w = self.column_defs
            .get(idx)
            .unwrap_or(self.default_column_def)
            .min_width;
        self.extra_column_width.saturating_add(w)
    }

    /// The maximum width of the column.
    #[must_use]
    pub (in crate) fn max_width(&self, idx: usize) -> usize {
        let w = self.column_defs
            .get(idx)
            .unwrap_or(self.default_column_def)
            .max_width;

        self.extra_column_width.saturating_add(w)
    }

    /// The  quantile of the widest cell values to ignore for computing dynamic
    /// column widths.
    #[must_use]
    pub (in crate) fn dynamic_width_quantile(&self, idx: usize) -> f64 {
        self.column_defs
            .get(idx)
            .unwrap_or(self.default_column_def)
            .dynamic_width_quantile
    }

    /// The relative weight of the column in allocating width under constraint.
    #[must_use]
    pub (in crate) fn dynamic_width_weight(&self, idx: usize) -> f64 {
        self.column_defs
            .get(idx)
            .unwrap_or(self.default_column_def)
            .dynamic_width_weight
    }

    /// The horizontal alignment of text in the column.
    #[must_use]
    pub (in crate) fn horz_align(&self, idx: usize) -> HorzAlign {
        self.column_defs
            .get(idx)
            .unwrap_or(self.default_column_def)
            .horz_align
    }

    /// The vertical alignment of text in the column.
    #[must_use]
    pub (in crate) fn vert_align(&self, idx: usize) -> VertAlign {
        self.column_defs
            .get(idx)
            .unwrap_or(self.default_column_def)
            .vert_align
    }

    /// Returns `true` if the column width is fully constrained.
    #[must_use]
    pub (in crate) fn is_fixed_width(&self, idx: usize) -> bool {
        self.column_defs
            .get(idx)
            .unwrap_or(self.default_column_def)
            .is_fixed_width()
    }

    /// Clamps the given value between the min and max width allowed for the
    /// column.
    #[must_use]
    pub (in crate) fn clamp_to_valid_width(&self, idx: usize, value: usize)
        -> usize
    {
        self.column_defs
            .get(idx)
            .unwrap_or(self.default_column_def)
            .clamp_to_valid_width(value)
    }
}
