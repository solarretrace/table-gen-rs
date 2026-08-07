////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! A table renderer that renders tables with minimal decoration.
////////////////////////////////////////////////////////////////////////////////

// Workspace library imports.
use table_gen_core::ColumnDesc;
use table_gen_core::Features;
use table_gen_core::HorzAlign;
use table_gen_core::Renderer;


////////////////////////////////////////////////////////////////////////////////
// MarkdownPipeRenderer
////////////////////////////////////////////////////////////////////////////////
/// A table renderer that renders tables in the pandoc-markdown 'grid' style.
#[derive(Debug, Clone)]
pub struct MarkdownPipeRenderer {
    /// The column widths. Used to render separators with the correct size.
    column_widths: Vec<usize>,
    /// The column horizontal aligments. Used to render alignment symbols.
    column_horz_aligns: Vec<HorzAlign>,
    /// Indicates that headers were provided for rendering.
    headers_provided: bool,
    /// The amount of space to allocate between columns.
    column_padding: u8,
    /// The amount of extra space to allocate within columns.
    extra_width: u8,
    /// Whether alignment markers should be omitted.
    align_markers: bool,
    /// Whether pipes should used in the header divider separator.
    header_pipes: bool,
}

impl Default for MarkdownPipeRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownPipeRenderer {
    /// Constructs a new `MarkdownPipeRenderer`.
    pub const fn new() -> Self {
        Self {
            column_widths: Vec::new(),
            column_horz_aligns: Vec::new(),
            headers_provided: false,
            column_padding: 0,
            extra_width: 0,
            align_markers: true,
            header_pipes: true,
        }
    }

    /// Sets the column padding and returns the `MarkdownPipeRenderer`.
    pub const fn with_column_padding(mut self, column_padding: u8) -> Self {
        self.column_padding = column_padding;
        self
    }

    /// Sets the extra column width and returns the `MarkdownPipeRenderer`.
    pub const fn with_extra_width(mut self, extra_width: u8) -> Self {
        self.extra_width = extra_width;
        self
    }

    /// Sets the alignment markers usage flag and returns the
    /// `MarkdownPipeRenderer`.
    pub const fn with_alignment_markers(mut self, align_markers: bool) -> Self {
        self.align_markers = align_markers;
        self
    }

    /// Sets the flag to use pipe symbols in the header divider separator and
    /// returns the `MarkdownPipeRenderer`.
    pub const fn with_header_div_pipes(mut self, header_pipes: bool)
        -> Self
    {
        self.header_pipes = header_pipes;
        self
    }

    /// Renders an empty row.
    fn write_empty_row<W>(&self, out: &mut W)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        self.write_column_sep(out, HorzAlign::Left, "|", " ", " ", " ")?;
        for (col, col_width) in self.column_widths.iter().copied().enumerate() {
            for _ in 0..col_width { write!(out, " ")?; }
            for _ in 0..self.extra_width { write!(out, " ")?; }
            if col + 1 == self.column_widths.len() { break; }
            self.write_column_sep(out, HorzAlign::Center, "|", " ", " ", " ")?;
        }
        self.write_column_sep(out, HorzAlign::Right, "|", " ", " ", " ")?;
        Ok(())
    }

    /// Renders a column separator.
    fn write_column_sep<W>(
        &self,
        out: &mut W,
        bias: HorzAlign,
        center: &str,
        outer: &str,
        inner_left: &str,
        inner_right: &str)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        debug_assert!(inner_left.len() < 2);
        debug_assert_eq!(inner_left.len(), inner_right.len());
        let inner_pad = inner_left.len() as u8;
        let mut pad = std::cmp::max(self.column_padding / 2, inner_pad * 2) / 2;
        pad -= inner_pad;

        if bias != HorzAlign::Left {
            for _ in 0..pad { write!(out, "{}", outer)?; }
            write!(out, "{}", inner_left)?;
        }
        write!(out, "{}", center)?;
        if bias != HorzAlign::Right {
            write!(out, "{}", inner_right)?;
            for _ in 0..pad { write!(out, "{}", outer)?; }
        }
        Ok(())
    }
}

impl Renderer for MarkdownPipeRenderer {
    fn features(&self) -> Features {
        Features::empty()
    }

    fn init(
        &mut self,
        column_descs: &[ColumnDesc<'_>],
        _row_count: usize,
        column_widths: &[usize])
    {
        self.column_widths = column_widths.iter().copied().collect();
        self.column_horz_aligns = Vec::with_capacity(column_descs.len());
        self.headers_provided = false;
        for column_desc in column_descs.iter() {
            self.column_horz_aligns.push(column_desc.horz_align);
            self.headers_provided |= !column_desc.header.is_empty();
        }
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
        if !self.headers_provided {
            self.write_empty_row(out)?;
            writeln!(out)?;
        }

        // Define style markers.
        let dm = "-"; // divider marker
        let am = if self.align_markers { ":" } else { dm }; // align marker
        let pm = "|"; // pipe marker
        let im = if self.header_pipes { pm } else { "+" }; // internal div marker

        use HorzAlign::*;
        for (col, col_width) in self.column_widths.iter().copied().enumerate() {
            let cur_align = self.column_horz_aligns.get(col).copied();
            if col == 0 {
                let r = if matches!(cur_align, Some(Right|Center)) {
                    am
                } else {
                    dm
                };
                self.write_column_sep(out, Left, pm, dm, " ", r)?;
            }
            for _ in 0..col_width { write!(out, "{}", dm)?; }
            for _ in 0..self.extra_width { write!(out, "{}", dm)?; }

            let (l, r) = match (cur_align, self.column_horz_aligns.get(col + 1))
            {
                (Some(Right|Center), Some(Left|Center)) => (am, am),
                (Some(Right|Center), _)                 => (am, dm),
                (_,                  Some(Left|Center)) => (dm, am),
                _                                       => (dm, dm),
            };
            if col + 1 == self.column_widths.len() { 
                self.write_column_sep(out, Right, pm, dm, l, " ")?;
                break;
            }
            self.write_column_sep(out, Center, im, dm, l, r)?;
        }
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
        let pm = "|"; // pipe marker
        if col == 0 {
            self.write_column_sep(out, HorzAlign::Left, pm, " ", " ", " ")?;
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
        let pm = "|"; // pipe marker
        if col + 1 == self.column_widths.len() {
            self.write_column_sep(out, HorzAlign::Right, pm, " ", " ", " ")
        } else {
            self.write_column_sep(out, HorzAlign::Center, pm, " ", " ", " ")
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

        let mut table = Table::new_builder(data, MarkdownPipeRenderer::new())
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

        let mut table = Table::new_builder(data, MarkdownPipeRenderer::new())
            .with_column_descs(&col_descs)
            .with_column_selection(&[0, 0, 0])
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!(out, "\
| Right | Left  | Center |
|:-----:|:------|:------:|
|    12 | 12    |   12   |
|   123 | 123   |  123   |
|     1 | 1     |   1    |
| -8000 | -8000 | -8000  |
");
    }

    #[test]
    fn simple_table_alt() {
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

        let mut table = Table::new_builder(data, MarkdownPipeRenderer::new()
                .with_alignment_markers(false)
                .with_header_div_pipes(false))
            .with_column_descs(&col_descs)
            .with_column_selection(&[0, 0, 0])
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!(out, "\
| Right | Left  | Center |
|-------+-------+--------|
|    12 | 12    |   12   |
|   123 | 123   |  123   |
|     1 | 1     |   1    |
| -8000 | -8000 | -8000  |
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

        let mut table = Table::new_builder(data, MarkdownPipeRenderer::new())
            .with_column_descs(&col_descs)
            .with_column_selection(&[0, 0, 0])
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!(out, "\
|       |       |       |
|:-----:|:------|:-----:|
|    12 | 12    |  12   |
|   123 | 123   |  123  |
|     1 | 1     |   1   |
| -8000 | -8000 | -8000 |
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

        let mut table = Table::new_builder(data, MarkdownPipeRenderer::new())
            .with_column_descs(&col_descs)
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        println!("{}", out);

        assert_eq!(out, "\
| Centered | Left    |   Right | Left                                                 |
|  Header  | Aligned | Aligned | Aligned                                              |
|:--------:|:--------|--------:|:-----------------------------------------------------|
|  First   | row     |      12 | Example of a row that spans multiple lines.          |
|  Second  | row     |       5 | Here's another one. Note the blank line between rows |
");
    }
}
