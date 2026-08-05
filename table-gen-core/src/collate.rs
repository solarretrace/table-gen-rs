////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator column collation module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::Cell;
use crate::ColumnOrd;
use crate::DisplayFmt;
use crate::Row;

// Standard library imports.
use std::ops::RangeBounds;
use std::ops::Bound;


////////////////////////////////////////////////////////////////////////////////
// Collate
////////////////////////////////////////////////////////////////////////////////
/// Table collater. Responsible for specifiying column ordering, headers,
/// footers, and defaults.
#[derive(Debug, Clone)]
pub struct Collate<'a, S> {
    /// The table data source.
    inner: S,
    /// The table output columns, in output order.
    col_select: &'a [usize],
    /// The table output rows.
    row_select: (Bound<usize>, Bound<usize>),
    /// The column output specifications.
    col_descs: &'a [ColumnDesc<'a>],
    /// The sort parameters for columns, in order of sort priority.
    col_order: &'a [ColumnOrd],

}

impl<'a, R, S, I> From<I> for Collate<'a, S>
    where
        R: Row,
        S: Iterator<Item=R>,
        I: IntoIterator<Item=R, IntoIter=S>,
{
    fn from(inner: I) -> Self {
        Collate::new(inner.into_iter())
    }
}

impl<'a, R, S> Collate<'a, S>
    where
        R: Row,
        S: Iterator<Item=R>,
{
    /// Constructs a new `Collate` for the given data source.
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            col_select: &[],
            row_select: (Bound::Included(0), Bound::Unbounded),
            col_descs: &[],
            col_order: &[],
        }
    }

    /// Returns the `Collate` with the given columns selected for output in the
    /// given order.
    pub fn with_column_selection(mut self, col_select: &'a [usize]) -> Self {
        self.col_select = col_select;
        self
    }

    /// Prepares the table builder such that only the given rows will be
    /// rendered.
    pub fn with_row_selection<B>(mut self, row_select: B) -> Self 
        where B: RangeBounds<usize>
    {
        self.row_select = (
            row_select.start_bound().cloned(),
            row_select.end_bound().cloned());
        self
    }

    /// Returns the `Collate` with the given column output specifications.
    pub fn with_column_descs(mut self, col_descs: &'a [ColumnDesc]) -> Self {
        self.col_descs = col_descs;
        self
    }

    /// Returns the `Collate` with the given column orderings.
    pub fn with_column_order(mut self, col_order: &'a [ColumnOrd]) -> Self {
        self.col_order = col_order;
        self
    }

    /// The table output columns, in output order.
    pub fn column_selection(&self) -> &'a [usize] { &self.col_select[..] }

    /// The column output specifications.
    pub fn column_descs(&self) -> &'a [ColumnDesc<'a>] { &self.col_descs[..] }

    /// The sort parameters for columns, in order of sort priority.
    pub fn column_order(&self) -> &'a [ColumnOrd] { &self.col_order[..] }
}


impl<'a, R, S> Iterator for Collate<'a, S>
    where
        S: Iterator<Item=R>,
        R: Row,
{
    type Item = CollateRow<'a, R>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|r| CollateRow { inner: r, col_select: &self.col_select[..], })
    }
}


////////////////////////////////////////////////////////////////////////////////
// CollateRow
////////////////////////////////////////////////////////////////////////////////
/// A single table row with collated column selection and ordering.
pub struct CollateRow<'a, R> {
    /// The row to collate.
    inner: R,
    /// The column selection and order.
    col_select: &'a [usize]
}

impl<'a, R> Row for CollateRow<'a, R>
    where R: Row
{
    fn len(&self) -> usize {
        if self.col_select.is_empty() {
            self.inner.len()
        } else {
            self.col_select.len()
        }
    }

    fn cell(&self, col_idx: usize) -> Option<&dyn Cell> {
        if self.col_select.is_empty() {
            self.inner.cell(col_idx)
        } else {
            self.inner.cell(self.col_select[col_idx])
        }
    }
}



////////////////////////////////////////////////////////////////////////////////
// ColumnDesc
////////////////////////////////////////////////////////////////////////////////
/// Provides formatting and metadata for a table column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColumnDesc<'a> {
    pub header: &'a str,
    pub footer: &'a str,
    pub display_fmt: DisplayFmt,
    pub min_width: usize,
    pub max_width: usize,
    pub horz_align: HorzAlign,
    pub vert_align: VertAlign,
}

impl<'a> Default for ColumnDesc<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ColumnDesc<'a> {
    pub const fn new() -> Self {
        Self {
            header: "",
            footer: "",
            display_fmt: DisplayFmt::new(),
            min_width: 0,
            max_width: usize::MAX,
            horz_align: HorzAlign::Left,
            vert_align: VertAlign::Top,
        }
    }
    
    pub const fn with_header(mut self, header: &'a str) -> Self {
        self.header = header;
        self
    }
    
    pub const fn with_footer(mut self, footer: &'a str) -> Self {
        self.footer = footer;
        self
    }
    
    pub const fn with_display_fmt(mut self, display_fmt: DisplayFmt) -> Self {
        self.display_fmt = display_fmt;
        self
    }
    
    pub const fn with_width(mut self, width: usize) -> Self {
        self.min_width = width;
        self.max_width = width;
        self
    }
    
    pub const fn with_min_width(mut self, min_width: usize) -> Self {
        self.min_width = min_width;
        self
    }
    
    pub const fn with_max_width(mut self, max_width: usize) -> Self {
        self.max_width = max_width;
        self
    }
    
    pub const fn with_horz_align(mut self, horz_align: HorzAlign) -> Self {
        self.horz_align = horz_align;
        self
    }
    
    pub const fn with_vert_align(mut self, vert_align: VertAlign) -> Self {
        self.vert_align = vert_align;
        self
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HorzAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertAlign {
    Top,
    Center,
    Bottom,
}
