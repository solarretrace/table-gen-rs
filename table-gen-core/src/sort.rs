////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator row sorting module.
////////////////////////////////////////////////////////////////////////////////


// Internal library imports.
use crate::Row;
use crate::Format;
use crate::FormattedRow;
use crate::ColSpec;
use crate::Collate;

// External library imports.
use bitflags::bitflags;

// Standard library imports.
use std::vec::IntoIter;
use std::cmp::Ordering;


////////////////////////////////////////////////////////////////////////////////
// Sort
////////////////////////////////////////////////////////////////////////////////
/// Table cell column sorter.
pub struct Sort<'a, R, S> {
    /// The table data source.
    inner: Format<'a, S>,
    sorted: Option<IntoIter<FormattedRow<'a, R>>>,
}

impl<'a, R, S, I> From<I> for Sort<'a, R, S>
    where
        R: Row,
        S: Iterator<Item=R>,
        I: IntoIterator<Item=R, IntoIter=S>,
{
    fn from(inner: I) -> Self {
        Sort::new(inner.into())
    }
}

impl<'a, R, S> From<Collate<'a, S>> for Sort<'a, R, S>
    where
        R: Row,
        S: Iterator<Item=R>,
{
    fn from(inner: Collate<'a, S>) -> Self {
        Sort::new(inner.into())
    }
}

impl<'a, R, S> From<Format<'a, S>> for Sort<'a, R, S>
    where
        R: Row,
        S: Iterator<Item=R>,
{
    fn from(inner: Format<'a, S>) -> Self {
        Sort::new(inner)
    }
}

impl<'a, R, S> Sort<'a, R, S>
    where
        R: Row,
        S: Iterator<Item=R>,
{
    /// Constructs a new `Sort` for the given data source.
    pub fn new(inner: Format<'a, S>) -> Self {
        Self {
            inner,
            sorted: None,
        }
    }

    /// The table output columns, in output order.
    pub fn cols(&self) -> &'a [usize] { self.inner.cols() }

    /// The column output specifications.
    pub fn col_specs(&self) -> &'a [ColSpec<'a>] { self.inner.col_specs() }

    /// The sort parameters for columns, in order of sort priority.
    pub fn col_ords(&self) -> &'a [ColOrd] { self.inner.col_ords() }

    /// Compares two `FormattedRow`s according to the ordering given by
    /// `[ColOrd]`.
    fn compare_rows(
        row_a: &FormattedRow<'_, R>,
        row_b: &FormattedRow<'_, R>,
        col_ord: &'_ [ColOrd])
        -> Ordering
    {
        let mut res = Ordering::Equal;
        for ord in col_ord {
            let text = ord.flags.contains(ColOrdFlags::FORMATTED);
            let rev = ord.flags.contains(ColOrdFlags::REVERSE);
            let nl = if ord.flags.contains(ColOrdFlags::NONE_LESS) {
                Ordering::Less
            } else {
                Ordering::Greater
            };
            res = if text {
                let a = row_a.text(ord.idx);
                let b = row_b.text(ord.idx);
                a.cmp(b)
            } else {
                let a = row_a.cell(ord.idx);
                let b = row_b.cell(ord.idx);
                match (a, b) {
                    (None,    Some(_)) => nl,
                    (Some(_), None)    => nl.reverse(),
                    (Some(a), Some(b)) => a.dyn_partial_cmp(b)
                        .unwrap_or(Ordering::Equal),
                    _ => Ordering::Equal,
                }
            };
            if rev { res = res.reverse(); }
            if res.is_ne() { break; }
        }
        res
    }
}

impl<'a, R, S> Iterator for Sort<'a, R, S>
    where
        S: Iterator<Item=R>,
        R: Row,
{
    type Item = FormattedRow<'a, R>;
    fn next(&mut self) -> Option<Self::Item> {
        let col_ords = self.col_ords();
        let iter = self.sorted.get_or_insert_with(|| {
            let mut rows: Vec<_> = (&mut self.inner).collect();
            rows.sort_by(|a, b| Self::compare_rows(a, b, col_ords));
            rows.into_iter()
        });
        iter.next()
    }
}



////////////////////////////////////////////////////////////////////////////////
// ColOrd
////////////////////////////////////////////////////////////////////////////////
/// A column ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColOrd {
    /// The column index
    pub idx: usize,
    /// The column ordering flags.
    pub flags: ColOrdFlags,

}


bitflags! {
    /// Columns ordering flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ColOrdFlags: u8 {
        /// Indicates that the column should be sorted in reverse order.
        const REVERSE    = 0b_0000_0001;
        /// Indicates that the ordering should be done on the formatted text
        /// rather than the cell values.
        const FORMATTED  = 0b_0000_0010;
        /// Indicates that empty cells in the column should sort before other
        /// values.
        const NONE_LESS = 0b_0000_0100;
    }
}
