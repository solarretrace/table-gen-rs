////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator column collation module.
////////////////////////////////////////////////////////////////////////////////


// Internal library imports.
use crate::Row;
use crate::Cell;
use crate::ColFmt;
use crate::ColOrd;


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
    cols: &'a [usize],
    /// The column output specifications.
    col_specs: &'a [ColSpec<'a>],
    /// The sort parameters for columns, in order of sort priority.
    col_ords: &'a [ColOrd],

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
            cols: &[],
            col_specs: &[],
            col_ords: &[],
        }
    }

    /// Returns the `Collate` with the given columns selected for output in the
    /// given order.
    pub fn with_columns(mut self, cols: &'a [usize]) -> Self {
        self.cols = cols;
        self
    }

    /// Returns the `Collate` with the given column output specifications.
    pub fn with_col_specs(mut self, col_specs: &'a [ColSpec]) -> Self {
        self.col_specs = col_specs;
        self
    }

    /// Returns the `Collate` with the given column orderings.
    pub fn with_col_ords(mut self, col_ords: &'a [ColOrd]) -> Self {
        self.col_ords = col_ords;
        self
    }

    /// The table output columns, in output order.
    pub fn cols(&self) -> &'a [usize] { &self.cols[..] }

    /// The column output specifications.
    pub fn col_specs(&self) -> &'a [ColSpec<'a>] { &self.col_specs[..] }

    /// The sort parameters for columns, in order of sort priority.
    pub fn col_ords(&self) -> &'a [ColOrd] { &self.col_ords[..] }
}


impl<'a, R, S> Iterator for Collate<'a, S>
    where
        S: Iterator<Item=R>,
        R: Row,
{
    type Item = CollatedRow<'a, R>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|r| CollatedRow { inner: r, cols: &self.cols[..], })
    }
}


////////////////////////////////////////////////////////////////////////////////
// CollatedRow
////////////////////////////////////////////////////////////////////////////////
/// A single table row with collated column selection and ordering.
pub struct CollatedRow<'a, R> {
    /// The row to collate.
    inner: R,
    /// The column selection and order.
    cols: &'a [usize]
}

impl<'a, R> Row for CollatedRow<'a, R>
    where R: Row
{
    fn len(&self) -> usize {
        if self.cols.is_empty() {
            self.inner.len()
        } else {
            self.cols.len()
        }
    }

    fn cell(&self, col_idx: usize) -> Option<&dyn Cell> {
        if self.cols.is_empty() {
            self.inner.cell(col_idx)
        } else {
            self.inner.cell(self.cols[col_idx])
        }
    }
}



////////////////////////////////////////////////////////////////////////////////
// ColSpec
////////////////////////////////////////////////////////////////////////////////
/// A column ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColSpec<'a> {
    pub header: &'a str,
    pub footer: &'a str,
    pub col_fmt: ColFmt,
    pub min_width: usize,
    pub max_width: usize,
    pub horz_align: HorzAlign,
    pub vert_align: VertAlign,
}

impl<'a> Default for ColSpec<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ColSpec<'a> {
    pub const fn new() -> Self {
        Self {
            header: "",
            footer: "",
            col_fmt: ColFmt::new(),
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
    
    pub const fn with_col_fmt(mut self, col_fmt: ColFmt) -> Self {
        self.col_fmt = col_fmt;
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
