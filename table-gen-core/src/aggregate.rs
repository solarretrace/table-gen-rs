////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator row sorting module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::ColumnDesc;
use crate::Row;
use crate::Split;
use crate::SplitRow;
use crate::TextRow;



////////////////////////////////////////////////////////////////////////////////
// Aggregate
////////////////////////////////////////////////////////////////////////////////
/// Table cell column aggregator.
#[derive(Debug, Clone)]
pub (in crate) struct Aggregate<'a, R> {
    /// The materialized rows of the table.
    rows: Vec<SplitRow<'a, R>>,
    /// The column output specifications.
    col_descs: &'a [ColumnDesc<'a>],
    /// The column widths.
    col_widths: Vec<usize>,
    /// The table header row.
    header_row: Option<TextRow<'a>>,
    /// The table footer row.
    footer_row: Option<TextRow<'a>>,
}

impl<'a, R> Aggregate<'a, R>
    where R: Row,
{
    /// Constructs a new `Aggregate` for the given data source.
    pub (in crate) fn new<S, T>(inner: T, default_col_desc: &ColumnDesc<'_>) -> Self
        where
            T: Into<Split<'a, R, S>>,
            S: Iterator<Item=R>,
            R: Row
    {
        let str_width = |l: &str| { l.len() };
        let inner = inner.into();
        let col_descs = inner.column_descs();

        // Build the header row.
        let mut header_used = false;
        let header_cells: Vec<&str> = col_descs.iter()
            .map(|col_desc| col_desc.header)
            .map(|c| { header_used |= !c.is_empty(); c })
            .collect();
        let mut header_row = header_used
            .then_some(header_cells)
            .map(TextRow::new);
        // Build the footer row.
        let mut footer_used = false;
        let footer_cells: Vec<&str> = col_descs.iter()
            .map(|col_desc| col_desc.footer)
            .map(|c| { footer_used |= !c.is_empty(); c })
            .collect();
        let mut footer_row = footer_used
            .then_some(footer_cells)
            .map(TextRow::new);

        // Compute initial column widths from header/footer rows if available.
        let mut col_widths: Vec<usize> = match (
            header_row.as_ref(),
            footer_row.as_ref())
        {
            (Some(h), Some(f)) => (0..col_descs.len())
                .map(|idx| std::cmp::max(
                    h.lines(idx)
                        .map(|l| (str_width)(l))
                        .max()
                        .unwrap_or(col_descs[idx].min_width),
                    f.lines(idx)
                        .map(|l| (str_width)(l))
                        .max()
                        .unwrap_or(col_descs[idx].min_width)))
                .collect(),

            (Some(r), None)    |
            (None,    Some(r)) => (0..col_descs.len())
                .map(|idx| r
                    .lines(idx)
                    .map(|l| (str_width)(l))
                    .max()
                    .unwrap_or(col_descs[idx].min_width))
                .collect(),

            _ => (0..col_descs.len())
                .map(|idx| col_descs[idx].min_width)
                .collect(),
        };

        let mut max_row_len = 0;
        let mut rows = Vec::new();
        // Do column aggregations.
        for row in inner {
            
            max_row_len = std::cmp::max(max_row_len, row.len());
            for idx in 0..row.len() {
                // Expand widths array if past the end of the header/footer.
                if idx >= col_widths.len() { col_widths.push(0); }

                // Get the ColumnDesc for this index.
                let col_desc = col_descs.get(idx).unwrap_or(&default_col_desc);

                if col_desc.min_width == col_desc.max_width {
                    // The col width is fixed, so set it.
                    col_widths[idx] = col_desc.max_width;
                } else {
                    // The col width is dynamic. Get the width of the cell.
                    let cell_width = row.lines(idx)
                        .map(|l| (str_width)(l))
                        .max()
                        .unwrap_or(0);
                    // If the cell widens the current width, do so, but do not
                    // exceed the maximum allowed.
                    col_widths[idx] = std::cmp::min(
                        std::cmp::max(cell_width, col_widths[idx]),
                        col_desc.max_width);
                }
            }
            rows.push(row);
        }
        
        header_row = header_row.map(|r| r.with_len(max_row_len));
        footer_row = footer_row.map(|r| r.with_len(max_row_len));

        Self {
            rows,
            col_descs,
            col_widths,
            header_row,
            footer_row,
        }
    }

    /// The column widths.
    pub (in crate) fn rows(&self) -> &[SplitRow<'a, R>] { &self.rows[..] }

    /// The column output descriptors.
    pub (in crate) fn column_descs(&self) -> &'a [ColumnDesc<'a>] { &self.col_descs[..] }

    /// The column widths.
    pub (in crate) fn col_widths(&self) -> &[usize] { &self.col_widths[..] }

    /// The header row.
    pub (in crate) fn header_row(&self) -> Option<&TextRow<'_>> {
        self.header_row.as_ref()
    }

    /// The footer row.
    pub (in crate) fn footer_row(&self) -> Option<&TextRow<'_>> {
        self.footer_row.as_ref()
    }
}

