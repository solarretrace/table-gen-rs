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
// MarkdownSimpleRenderer
////////////////////////////////////////////////////////////////////////////////
/// A table renderer that renders tables in the pandoc-markdown 'simple' style.
#[derive(Debug, Clone)]
pub struct MarkdownSimpleRenderer {
    column_widths: Vec<usize>,
    last_column: usize,
    headers_provided: bool,
    column_padding: u8,
    extra_width: u8,
}

impl Default for MarkdownSimpleRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownSimpleRenderer {
    /// Constructs a new `MarkdownSimpleRenderer`.
    pub const fn new() -> Self {
        Self {
            column_widths: Vec::new(),
            last_column: 0,
            headers_provided: false,
            column_padding: 0,
            extra_width: 2,
        }
    }

    /// Sets the column padding and returns the `MarkdownSimpleRenderer`.
    pub const fn with_column_padding(mut self, column_padding: u8) -> Self {
        self.column_padding = column_padding;
        self
    }

    /// Sets the extra column width and returns the `MarkdownSimpleRenderer`.
    pub const fn with_extra_width(mut self, extra_width: u8) -> Self {
        self.extra_width = extra_width;
        self
    }
}

impl Renderer for MarkdownSimpleRenderer {
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
        self.last_column = column_widths.len().saturating_sub(1);
        self.headers_provided = column_descs.iter()
            .any(|column_desc| !column_desc.header.is_empty());
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

    fn write_header_cell_line_end<W>(
        &mut self,
        out: &mut W,
        _row: usize,
        col: usize,
        _line: usize)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        if col == self.last_column { return Ok(()); }
        for _ in 0..self.column_padding { write!(out, " ")?; }
        write!(out, " ")
    }

    fn write_data_start<W>(&mut self, out: &mut W) -> std::io::Result<()>
        where W: std::io::Write
    {
        if self.column_widths.is_empty() { return Ok(()) }
        for (col, col_width) in self.column_widths.iter().copied().enumerate() {
            for _ in 0..col_width { write!(out, "-")?; }
            for _ in 0..self.extra_width { write!(out, "-")?; }
            if col == self.last_column { break; }
            for _ in 0..self.column_padding { write!(out, " ")?; }
            write!(out, " ")?;
        }
        writeln!(out)
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
        if col == self.last_column { return Ok(()); }
        for _ in 0..self.column_padding { write!(out, " ")?; }
        write!(out, " ")
    }

    fn write_data_end<W>(&mut self, out: &mut W) -> std::io::Result<()>
        where W: std::io::Write
    {
        if !self.headers_provided {
            self.write_data_start(out)?;
        }
        Ok(())
    }
}



#[cfg(test)]
mod test {
    use super::*;
    use table_gen_core::ColumnDesc;
    use table_gen_core::DisplayFmt;
    use table_gen_core::HorzAlign;
    use table_gen_core::Sign;
    use table_gen_core::Table;
    use table_gen_core::VertAlign;

    #[test]
    fn empty_table() {
        let data: Vec<(usize, )> = vec![];

        let mut table = Table::new_builder(data, MarkdownSimpleRenderer::new())
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

        let mut table = Table::new_builder(data, MarkdownSimpleRenderer::new())
            .with_column_descs(&col_descs)
            .with_column_selection(&[0, 0, 0])
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!(out, "\
  Right Left     Center 
------- ------- --------
     12 12         12   
    123 123       123   
      1 1          1    
  -8000 -8000    -8000  
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

        let mut table = Table::new_builder(data, MarkdownSimpleRenderer::new())
            .with_column_descs(&col_descs)
            .with_column_selection(&[0, 0, 0])
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        println!("{}", out);

        assert_eq!(out, "\
------- ------- -------
     12 12        12   
    123 123       123  
      1 1          1   
  -8000 -8000    -8000 
------- ------- -------
");
    }

}
