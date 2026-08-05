////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator cell formatting module.
////////////////////////////////////////////////////////////////////////////////


// Internal library imports.
use crate::Row;
use crate::Cell;
use crate::ColumnDesc;
use crate::ColOrd;
use crate::Collate;
use crate::CollatedRow;

// Standard library imports.
use std::fmt::Display;
use std::cell::OnceCell;


////////////////////////////////////////////////////////////////////////////////
// Format
////////////////////////////////////////////////////////////////////////////////
/// Table cell formatter. Responsible for generating text for cells.
#[derive(Debug, Clone)]
pub struct Format<'a, S> {
    /// The table data source.
    inner: Collate<'a, S>,
}

impl<'a, R, S, I> From<I> for Format<'a, S>
    where
        R: Row,
        S: Iterator<Item=R>,
        I: IntoIterator<Item=R, IntoIter=S>,
{
    fn from(inner: I) -> Self {
        Format::new(inner.into())
    }
}

impl<'a, R, S> From<Collate<'a, S>> for Format<'a, S>
    where
        R: Row,
        S: Iterator<Item=R>,
{
    fn from(inner: Collate<'a, S>) -> Self {
        Format::new(inner)
    }
}

impl<'a, R, S> Format<'a, S>
    where
        S: Iterator<Item=R>,
        R: Row,
{
    /// Constructs a new `Format` for the given data source.
    pub fn new(inner: Collate<'a, S>) -> Self {
        Self {
            inner,
        }
    }

    /// The table output columns, in output order.
    pub fn column_selection(&self) -> &'a [usize] { self.inner.column_selection() }

    /// The column output specifications.
    pub fn column_descs(&self) -> &'a [ColumnDesc<'a>] { self.inner.column_descs() }

    /// The sort parameters for columns, in order of sort priority.
    pub fn column_order(&self) -> &'a [ColOrd] { self.inner.column_order() }
}

impl<'a, R, S> Iterator for Format<'a, S>
    where
        S: Iterator<Item=R>,
        R: Row,
{
    type Item = FormattedRow<'a, R>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|r| FormattedRow::new(r, self.inner.column_descs()))
    }
}


////////////////////////////////////////////////////////////////////////////////
// FormattedRow
////////////////////////////////////////////////////////////////////////////////
/// A single table row with collated column selection and ordering.
pub struct FormattedRow<'a, R> {
    /// The row to format.
    inner: CollatedRow<'a, R>,
    /// The column output specifications,
    col_descs: &'a [ColumnDesc<'a>],
    /// The cached cell texts.
    cache: Vec<OnceCell<Box<str>>>,
}

impl<'a, R> Row for FormattedRow<'a, R>
    where R: Row
{
    fn len(&self) -> usize {
        self.inner.len()
    }

    fn cell(&self, col_idx: usize) -> Option<&dyn Cell> {
        self.inner.cell(col_idx)
    }
}

impl<'a, R> FormattedRow<'a, R>
    where R: Row,
{
    pub fn new(inner: CollatedRow<'a, R>, col_descs: &'a [ColumnDesc]) -> Self {
        let cache = vec![OnceCell::new(); inner.len()];
        Self {
            inner,
            col_descs,
            cache,
        }
    }

    pub fn text(&self, col_idx: usize) -> &str {
        self.cache[col_idx].get_or_init(|| match self.inner.cell(col_idx) {
            Some(cell) => self.col_descs
                .get(col_idx)
                .map(|spec| spec.display_fmt)
                .unwrap_or(DisplayFmt::default())
                .apply(cell),
            None => String::new().into_boxed_str(),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////
// DisplayFmt
////////////////////////////////////////////////////////////////////////////////
/// Parameters for cell `Display` output formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DisplayFmt {
    /// Format precision specifier.
    precision: Option<usize>,
    /// Format sign specifier.
    sign: Option<Sign>,
}

impl Default for DisplayFmt {
    fn default() -> Self {
        Self::new()
    }
}

impl DisplayFmt {
    pub const fn new() -> Self {
        Self {
            precision: None,
            sign: None,
        }
    }
    
    pub fn apply<C>(&self, cell: C) -> Box<str>
        where C: Display
    {
        match (self.precision, self.sign) {
            (Some(p), Some(Sign::Plus))     => format!("{:+.p$}", cell, p=p),
            (Some(p), Some(Sign::Minus))    => format!("{:-.p$}", cell, p=p),
            (Some(p), Some(Sign::Zero))     => format!("{:0.p$}", cell, p=p),
            (Some(p), None)                 => format!("{:.p$}", cell, p=p),
            (None,    Some(Sign::Plus))     => format!("{:+}", cell),
            (None,    Some(Sign::Minus))    => format!("{:-}", cell),
            (None,    Some(Sign::Zero))     => format!("{:0}", cell),
            (None,    None)                 => format!("{}", cell),
        }
        .into_boxed_str()
    }
}

/// Format sign specifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sign {
    Plus,
    Minus,
    Zero,
}
