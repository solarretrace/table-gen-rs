////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! A table renderer that renders tables in the pandoc-markdown 'grid' style.
////////////////////////////////////////////////////////////////////////////////

// Workspace library imports.
use table_gen_core::ColumnDesc;
use table_gen_core::Features;
use table_gen_core::HorzAlign;
use table_gen_core::Renderer;


////////////////////////////////////////////////////////////////////////////////
// MarkdownGridRenderer
////////////////////////////////////////////////////////////////////////////////
/// A table renderer that renders tables in the pandoc-markdown 'grid' style.
#[derive(Debug, Clone)]
pub struct MarkdownGridRenderer {
    /// The column widths. Used to render separators with the correct size.
    column_widths: Vec<usize>,
    /// Indicates that headers were provided for rendering.
    headers_provided: bool,
    /// The amount of space to allocate between columns.
    column_padding: u8,
    /// The amount of extra space to allocate within columns.
    extra_width: u8,
}

impl Default for MarkdownGridRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownGridRenderer {
    /// Constructs a new `MarkdownGridRenderer`.
    pub const fn new() -> Self {
        Self {
            column_widths: Vec::new(),
            headers_provided: false,
            column_padding: 0,
            extra_width: 0,
        }
    }

    /// Sets the column padding and returns the `MarkdownGridRenderer`.
    pub const fn with_column_padding(mut self, column_padding: u8) -> Self {
        self.column_padding = column_padding;
        self
    }

    /// Sets the extra column width and returns the `MarkdownGridRenderer`.
    pub const fn with_extra_width(mut self, extra_width: u8) -> Self {
        self.extra_width = extra_width;
        self
    }

    /// Renders a row seperating line.
    fn write_row_sep<W>(&self, out: &mut W, line: &str)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        self.write_column_sep(out, HorzAlign::Left, "+", line)?;
        for (col, col_width) in self.column_widths.iter().copied().enumerate() {
            for _ in 0..col_width { write!(out, "{}", line)?; }
            for _ in 0..self.extra_width { write!(out, "{}", line)?; }
            if col + 1 == self.column_widths.len() { break; }
            self.write_column_sep(out, HorzAlign::Center, "+", line)?;
        }
        self.write_column_sep(out, HorzAlign::Right, "+", line)?;
        Ok(())
    }

    /// Renders a column separator.
    fn write_column_sep<W>(
        &self,
        out: &mut W,
        bias: HorzAlign,
        center: &str,
        outer: &str)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        let pad = std::cmp::max(self.column_padding / 2, 2) / 2;

        if bias != HorzAlign::Left {
            for _ in 0..pad { write!(out, "{}", outer)?; }
        }
        write!(out, "{}", center)?;
        if bias != HorzAlign::Right {
            for _ in 0..pad { write!(out, "{}", outer)?; }
        }
        Ok(())
    }
}

impl Renderer for MarkdownGridRenderer {
    fn features(&self) -> Features {
        Features::MULTILINE
    }

    fn init(
        &mut self,
        column_descs: &[ColumnDesc<'_>],
        _row_count: usize,
        column_widths: &[usize])
    {
        self.column_widths = column_widths.iter().copied().collect();
        self.headers_provided = column_descs.iter()
            .any(|column_desc| !column_desc.header.is_empty());
    }

    fn write_header_start<W>(&mut self, out: &mut W)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        if self.column_widths.is_empty() { return Ok(()) }
        self.write_row_sep(out, "-")?;
        writeln!(out)
    }

    fn write_data_cell_line<W>(
        &mut self,
        out: &mut W,
        _row: usize,
        _col: usize,
        _line: usize,
        text: &str,
        width: usize,
        horz_align: HorzAlign)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        let mut pad = width.saturating_sub(text.len());
        pad += self.extra_width as usize;
        let (l_pad, r_pad) = match horz_align {
            HorzAlign::Left   => (0,     pad),
            HorzAlign::Center => (pad/2, pad.div_ceil(2)),
            HorzAlign::Right  => (pad,   0),
        };
        
        for _ in 0..l_pad { write!(out, " ")?; }
        write!(out, "{}", text)?;
        for _ in 0..r_pad { write!(out, " ")?; }
        Ok(())
    }

    fn write_data_start<W>(&mut self, out: &mut W) -> std::io::Result<()>
        where W: std::io::Write
    {
        if self.column_widths.is_empty() { return Ok(()) }
        let c = if self.headers_provided { "=" } else {"-" };
        self.write_row_sep(out, c)?;
        writeln!(out)
    }

    fn write_data_cell_line_start<W>(
        &mut self,
        out: &mut W,
        _row: usize,
        col: usize,
        _line: usize)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        if col == 0 {
            self.write_column_sep(out, HorzAlign::Left, "|", " ")?;
        }
        Ok(())
    }

    fn write_data_cell_line_end<W>(
        &mut self,
        out: &mut W,
        _row: usize,
        col: usize,
        _line: usize)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        if col + 1 == self.column_widths.len() {
            self.write_column_sep(out, HorzAlign::Right, "|", " ")
        } else {
            self.write_column_sep(out, HorzAlign::Center, "|", " ")
        }
    }

    fn write_data_row_start<W>(
        &mut self,
        out: &mut W,
        row: usize)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        if row != 0 {
            self.write_row_sep(out, "-")?;
            writeln!(out)?;
        }
        Ok(())
    }

    fn write_data_end<W>(&mut self, out: &mut W) -> std::io::Result<()>
        where W: std::io::Write
    {
        if !self.headers_provided {
            self.write_data_start(out)
        } else {
            self.write_header_start(out)
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;
    use table_gen_core::ColumnDesc;
    use table_gen_core::HorzAlign;
    use table_gen_core::Table;

    #[test]
    fn empty_table() {
        let data: Vec<(usize, )> = vec![];

        let mut table = Table::new_builder(data, MarkdownGridRenderer::new())
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!(out, "");
    }


    #[test]
    fn simple_table() {
        let data: Vec<(i64, )> = vec![
            (12,),
            (123,),
            (1,),
            (-8000,),
        ];

        let col_descs = vec![
            ColumnDesc::new()
                .with_header("Right")
                .with_horz_align(HorzAlign::Right),
            ColumnDesc::new()
                .with_header("Left")
                .with_horz_align(HorzAlign::Left),
            ColumnDesc::new()
                .with_header("Center")
                .with_horz_align(HorzAlign::Center),
        ];

        let mut table = Table::new_builder(data, MarkdownGridRenderer::new())
            .with_column_descs(&col_descs)
            .with_column_selection(&[0, 0, 0])
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!(out, "\
+-------+-------+--------+
| Right | Left  | Center |
+=======+=======+========+
|    12 | 12    |   12   |
+-------+-------+--------+
|   123 | 123   |  123   |
+-------+-------+--------+
|     1 | 1     |   1    |
+-------+-------+--------+
| -8000 | -8000 | -8000  |
+-------+-------+--------+
");
    }

    #[test]
    fn simple_table_no_headers() {
        let data: Vec<(i64, )> = vec![
            (12,),
            (123,),
            (1,),
            (-8000,),
        ];

        let col_descs = vec![
            ColumnDesc::new()
                .with_horz_align(HorzAlign::Right),
            ColumnDesc::new()
                .with_horz_align(HorzAlign::Left),
            ColumnDesc::new()
                .with_horz_align(HorzAlign::Center),
        ];

        let mut table = Table::new_builder(data, MarkdownGridRenderer::new())
            .with_column_descs(&col_descs)
            .with_column_selection(&[0, 0, 0])
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!(out, "\
+-------+-------+-------+
|    12 | 12    |  12   |
+-------+-------+-------+
|   123 | 123   |  123  |
+-------+-------+-------+
|     1 | 1     |   1   |
+-------+-------+-------+
| -8000 | -8000 | -8000 |
+-------+-------+-------+
");
    }

    #[test]
    fn multiline_table() {
        let data: Vec<(&str, &str, f64, &str)> = vec![
            ("First", "row",
                12.0, "Example of a row that\nspans multiple lines."),
            ("Second", "row",
                5.0, "Here's another one. Note\nthe blank line between\nrows"),
        ];

        let col_descs = vec![
            ColumnDesc::new()
                .with_header("Centered\nHeader")
                .with_horz_align(HorzAlign::Center),
            ColumnDesc::new()
                .with_header("Left\nAligned")
                .with_horz_align(HorzAlign::Left),
            ColumnDesc::new()
                .with_header("Right\nAligned")
                .with_horz_align(HorzAlign::Right),
            ColumnDesc::new()
                .with_header("Left\nAligned")
                .with_horz_align(HorzAlign::Left),
        ];

        let mut table = Table::new_builder(data, MarkdownGridRenderer::new())
            .with_column_descs(&col_descs)
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!(out, "\
+----------+---------+---------+--------------------------+
| Centered | Left    |   Right | Left                     |
|  Header  | Aligned | Aligned | Aligned                  |
+==========+=========+=========+==========================+
|  First   | row     |      12 | Example of a row that    |
|          |         |         | spans multiple lines.    |
+----------+---------+---------+--------------------------+
|  Second  | row     |       5 | Here's another one. Note |
|          |         |         | the blank line between   |
|          |         |         | rows                     |
+----------+---------+---------+--------------------------+
");
    }
}
