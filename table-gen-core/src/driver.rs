////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator driver module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::Row;
use crate::Renderer;
use crate::Collate;
use crate::Aggregate;
use crate::TextRow;
use crate::SplitRow;
use crate::ColumnDesc;
use crate::ColumnOrd;


////////////////////////////////////////////////////////////////////////////////
// TableBuilder
////////////////////////////////////////////////////////////////////////////////
/// A builder-style constructor for a `Table`.
pub struct TableBuilder<'a, S, T> {
    /// The table data source.
    inner: Collate<'a, S>,
    /// The table renderer.
    renderer: T,
    /// The default ColumnDesc.
    default_col_desc: ColumnDesc<'a>,
}

impl<'a, R, S, T> TableBuilder<'a, S, T>
    where
        R: Row,
        S: Iterator<Item=R>,
        T: Renderer,
{
    /// Constructs a new `TableBuilder`.
    pub fn new<I>(source: I, renderer: T) -> TableBuilder<'a, S, T>
        where I: IntoIterator<Item=R, IntoIter=S>
    {
        Self {
            inner: Collate::new(source.into_iter()),
            renderer,
            default_col_desc: ColumnDesc::new(),
        }
    }

    /// Prepares the table builder with the given columns selected for output in
    /// the given order.
    pub fn with_column_selection(mut self, col_select: &'a [usize]) -> Self {
        self.inner = self.inner.with_column_selection(col_select);
        self
    }

    /// Prepares the table builder with the given column output specifications.
    pub fn with_column_descs(mut self, col_descs: &'a [ColumnDesc<'a>]) -> Self
    {
        self.inner = self.inner.with_column_descs(col_descs);
        self
    }

    /// Prepares the table builder with the given default column output
    /// specification.
    pub fn with_default_col_desc(mut self, default_col_desc: ColumnDesc<'a>)
        -> Self
    {
        self.default_col_desc = default_col_desc;
        self
    }

    /// Prepares the table builder with the given output column orderings.
    pub fn with_column_order(mut self, col_order: &'a [ColumnOrd]) -> Self {
        self.inner = self.inner.with_column_order(col_order);
        self
    }

    /// Finishes collation of the data source and returns a `Table` for
    /// rendering.
    pub fn finish(self) -> Table<'a, R, T> {
        Table::new(self.inner, self.default_col_desc, self.renderer)
    }
}

impl<'a, R, S, T> From<TableBuilder<'a, S, T>> for Table<'a, R, T>
    where
        R: Row,
        S: Iterator<Item=R>,
        T: Renderer,
{
    fn from(builder: TableBuilder<'a, S, T>) -> Self {
        builder.finish()
    }
}


////////////////////////////////////////////////////////////////////////////////
// Table
////////////////////////////////////////////////////////////////////////////////
/// A driver for a table renderer operating on a data source.
pub struct Table<'a, R, T> {
    /// The table data source.
    inner: Aggregate<'a, R>,
    /// The table renderer.
    renderer: T,
    /// The default ColumnDesc.
    default_col_desc: ColumnDesc<'a>,
}

impl<'a, R, T> Table<'a, R, T>
    where
        R: Row,
        T: Renderer,
{
    /// Constructs a new `TableBuilder` suitable for building this type of
    /// table.
    pub fn new_builder<I, S>(source: I, renderer: T) -> TableBuilder<'a, S, T>
        where
            I: IntoIterator<Item=R, IntoIter=S>,
            S: Iterator<Item=R>
    {
        TableBuilder::new(source, renderer)
    }

    /// Constructs a new `Table` from a collated data source and renderer.
    pub fn new<S>(
        source: Collate<'a, S>,
        default_col_desc: ColumnDesc<'a>,
        renderer: T) -> Self
        where S: Iterator<Item=R>
    {
        let inner = Aggregate::new(source, &default_col_desc);
        Self {
            inner,
            renderer,
            default_col_desc,
        }
    }

    /// Renders the table output.
    pub fn render<W>(&mut self, out: &mut W)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        let col_descs = self.inner.column_descs();
        let col_widths = self.inner.col_widths();
        let rows = self.inner.rows();

        self.renderer.init(rows.len(), &col_widths);

        self.renderer.write_table_start(out)?;
        // Write the header.
        if let Some(row) = self.inner.header_row().as_ref() {
            self.renderer.write_header_start(out)?;
            Self::render_header_row(
                &mut self.renderer,
                col_descs,
                &self.default_col_desc,
                col_widths,
                out,
                row,
                0)?;
            self.renderer.write_header_end(out)?;
        }

        // Write the data.
        self.renderer.write_data_start(out)?;
        for (row_idx, row) in rows.iter().enumerate() {
            Self::render_data_row(
                &mut self.renderer,
                col_descs,
                &self.default_col_desc,
                col_widths,
                out,
                row,
                row_idx)?;
        }
        self.renderer.write_data_end(out)?;

        // Write the footer.
        if let Some(row) = self.inner.footer_row().as_ref() {
            self.renderer.write_footer_start(out)?;
            Self::render_footer_row(
                &mut self.renderer,
                col_descs,
                &self.default_col_desc,
                col_widths,
                out,
                row,
                0)?;
            self.renderer.write_footer_end(out)?;
        }
        self.renderer.write_table_end(out)?;

        Ok(())
    }

    /// Renders a row of the header.
    fn render_header_row<W>(
        renderer: &mut T,
        col_descs: &[ColumnDesc<'_>],
        default_col_desc: &ColumnDesc<'_>,
        col_widths: &[usize],
        out: &mut W,
        row: &TextRow<'_>,
        row_idx: usize)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        renderer.write_header_row_start(out, row_idx)?;
        
        for line_idx in 0..row.height() {
            renderer.write_header_line_start(out, row_idx, line_idx)?;
            for col_idx in 0..row.len() {
                if line_idx == 0 {
                    renderer.write_header_cell_start(out, row_idx, col_idx)?;
                }
                
                let col_desc = col_descs
                    .get(col_idx)
                    .unwrap_or(&default_col_desc);
                let text = row.line_vert_aligned(
                    col_idx,
                    line_idx,
                    col_desc.vert_align);
                renderer.write_header_cell_line_start(
                    out,
                    row_idx,
                    col_idx,
                    line_idx)?;
                renderer.write_header_cell_line(
                    out,
                    row_idx,
                    col_idx,
                    line_idx,
                    text,
                    col_widths[col_idx],
                    col_desc.horz_align)?;
                renderer.write_header_cell_line_end(
                    out,
                    row_idx,
                    col_idx,
                    line_idx)?;
                if line_idx == row.height() - 1 {
                    renderer.write_header_cell_end(out, row_idx, col_idx)?;
                }
            }
            renderer.write_header_line_end(out, row_idx, line_idx)?;
        }

        renderer.write_header_row_end(out, row_idx)?;
        Ok(())
    }

    /// Renders row of table data.
    fn render_data_row<W>(
        renderer: &mut T,
        col_descs: &[ColumnDesc<'_>],
        default_col_desc: &ColumnDesc<'_>,
        col_widths: &[usize],
        out: &mut W,
        row: &SplitRow<'_, R>,
        row_idx: usize)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        renderer.write_data_row_start(out, row_idx)?;
        for line_idx in 0..row.height() {
            renderer.write_data_line_start(out, row_idx, line_idx)?;
            for col_idx in 0..row.len() {
                if line_idx == 0 {
                    renderer.write_data_cell_start(out, row_idx, col_idx)?;
                }

                let col_desc = col_descs
                    .get(col_idx)
                    .unwrap_or(&default_col_desc);
                let text = row.line_vert_aligned(
                    col_idx,
                    line_idx,
                    col_desc.vert_align);
                renderer.write_data_cell_line_start(
                    out,
                    row_idx,
                    col_idx,
                    line_idx)?;
                renderer.write_data_cell_line(
                    out,
                    row_idx,
                    col_idx,
                    line_idx,
                    text,
                    col_widths[col_idx],
                    col_desc.horz_align)?;
                renderer.write_data_cell_line_end(
                    out,
                    row_idx,
                    col_idx,
                    line_idx)?;
                if line_idx == row.height() - 1 {
                    renderer.write_data_cell_end(out, row_idx, col_idx)?;
                }
            }
            renderer.write_data_line_end(out, row_idx, line_idx)?;
        }

        renderer.write_data_row_end(out, row_idx)?;
        Ok(())
    }


    /// Renders a row of the footer.
    fn render_footer_row<W>(
        renderer: &mut T,
        col_descs: &[ColumnDesc<'_>],
        default_col_desc: &ColumnDesc<'_>,
        col_widths: &[usize],
        out: &mut W,
        row: &TextRow<'_>,
        row_idx: usize)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        renderer.write_footer_row_start(out, row_idx)?;
        for line_idx in 0..row.height() {
            renderer.write_footer_line_start(out, row_idx, line_idx)?;
            for col_idx in 0..row.len() {
                if line_idx == 0 {
                    renderer.write_footer_cell_start(out, row_idx, col_idx)?;
                }
                
                let col_desc = col_descs
                    .get(col_idx)
                    .unwrap_or(&default_col_desc);
                let text = row.line_vert_aligned(
                    col_idx,
                    line_idx,
                    col_desc.vert_align);
                renderer.write_footer_cell_line_start(
                    out,
                    row_idx,
                    col_idx,
                    line_idx)?;
                renderer.write_footer_cell_line(
                    out,
                    row_idx,
                    col_idx,
                    line_idx,
                    text,
                    col_widths[col_idx],
                    col_desc.horz_align)?;
                renderer.write_footer_cell_line_end(
                    out,
                    row_idx,
                    col_idx,
                    line_idx)?;
                if line_idx == row.height() - 1 {
                    renderer.write_footer_cell_end(out, row_idx, col_idx)?;
                }
            }
            renderer.write_footer_line_end(out, row_idx, line_idx)?;
        }

        renderer.write_footer_row_end(out, row_idx)?;
        Ok(())
    }
}
