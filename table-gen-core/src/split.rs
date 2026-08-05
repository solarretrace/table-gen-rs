////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator cell formatting module.
////////////////////////////////////////////////////////////////////////////////


// Internal library imports.
use crate::Cell;
use crate::Collate;
use crate::ColumnDesc;
use crate::Format;
use crate::FormatRow;
use crate::Row;
use crate::Sort;
use crate::VertAlign;

// External library imports.
use smallvec::SmallVec;

// Standard library imports.
use std::ops::Bound;
use std::str::Lines;


////////////////////////////////////////////////////////////////////////////////
// Split
////////////////////////////////////////////////////////////////////////////////
/// Table cell line splitter.
pub (in crate) struct Split<'a, R, S> {
    /// The table data source.
    inner: Sort<'a, R, S>,
}

impl<R, S, I> From<I> for Split<'_, R, S>
    where
        R: Row,
        S: Iterator<Item=R>,
        I: IntoIterator<Item=R, IntoIter=S>,
{
    fn from(inner: I) -> Self {
        Split::new(inner.into())
    }
}

impl<'a, R, S> From<Collate<'a, S>> for Split<'a, R, S>
    where
        R: Row,
        S: Iterator<Item=R>,
{
    fn from(inner: Collate<'a, S>) -> Self {
        Split::new(inner.into())
    }
}

impl<'a, R, S> From<Format<'a, S>> for Split<'a, R, S>
    where
        R: Row,
        S: Iterator<Item=R>,
{
    fn from(inner: Format<'a, S>) -> Self {
        Split::new(inner.into())
    }
}

impl<'a, R, S> From<Sort<'a, R, S>> for Split<'a, R, S>
    where
        R: Row,
        S: Iterator<Item=R>,
{
    fn from(inner: Sort<'a, R, S>) -> Self {
        Split::new(inner)
    }
}

impl<'a, R, S> Split<'a, R, S>
    where
        R: Row,
        S: Iterator<Item=R>,
{
    /// Constructs a new `Split` for the given data source.
    pub (in crate) fn new(inner: Sort<'a, R, S>) -> Self {
        Self {
            inner,
        }
    }

    /// The row selection bounds.
    pub (in crate) fn row_selection(&self) -> &(Bound<usize>, Bound<usize>) {
        self.inner.row_selection()
    }

    /// The column output specifications.
    pub (in crate) fn column_descs(&self) -> &'a [ColumnDesc<'a>] {
        self.inner.column_descs()
    }
}

impl<'a, R, S> Iterator for Split<'a, R, S>
    where
        R: Row,
        S: Iterator<Item=R>,
{
    type Item = SplitRow<'a, R>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|r| SplitRow::new(r))
    }
}


////////////////////////////////////////////////////////////////////////////////
// SplitRow
////////////////////////////////////////////////////////////////////////////////
/// A single table row with line splitting.
#[derive(Debug, Clone)]
pub (in crate) struct SplitRow<'a, R> {
    /// The row to format.
    inner: FormatRow<'a, R>,
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
    pub (in crate) fn new(inner: FormatRow<'a, R>) -> Self {
        let height = (0..inner.len())
            .map(|c| inner.text(c).lines().count())
            .max()
            .unwrap_or(0);
        Self {
            inner,
            height,
        }
    }

    pub (in crate) fn height(&self) -> usize {
        self.height
    }

    pub (in crate) fn text(&self, col_idx: usize) -> &str {
        self.inner.text(col_idx)
    }

    pub (in crate) fn lines(&self, col_idx: usize) -> Lines<'_> {
        self.text(col_idx).lines()
    }
    
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
    inner: Vec<&'a str>,
    /// The maximum number of lines in the row.
    height: usize,
    /// The maximum number of columns in the row.
    len: usize,
}

impl<'a> TextRow<'a> {
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

    pub (in crate) fn with_len(mut self, len: usize) -> Self {
        self.len = len;
        self
    }

    pub (in crate) fn height(&self) -> usize {
        self.height
    }
    
    pub (in crate) fn len(&self) -> usize {
        self.len
    }
    
    pub (in crate) fn text(&self, col_idx: usize) -> &str {
        self.inner
            .get(col_idx)
            .map_or("", |t| t)
    }

    pub (in crate) fn lines(&self, col_idx: usize) -> Lines<'_> {
        self.text(col_idx).lines()
    }
    
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
// Vertically aligns text from a `Lines` iterator.
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
