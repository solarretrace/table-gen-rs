////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! A table renderer that renders tables with minimal decoration.
////////////////////////////////////////////////////////////////////////////////

// Workspace library imports.
use table_gen_core::Features;
use table_gen_core::Renderer;


////////////////////////////////////////////////////////////////////////////////
// MinimalRenderer
////////////////////////////////////////////////////////////////////////////////
/// A table renderer that renders tables with minimal decoration.
#[derive(Debug, Clone, Copy, Default)]
pub struct MinimalRenderer {
    col_final: usize,
}

impl MinimalRenderer {
    pub fn new() -> Self {
        Self {
            col_final: 0,
        }
    }
}

impl Renderer for MinimalRenderer {
    fn features(&self) -> Features {
        Features::LINES_SUPPORTED
    }

    fn init(&mut self, _row_count: usize, col_widths: &[usize]) {
        self.col_final = col_widths.len().saturating_sub(1);
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
        if col == self.col_final { return Ok(()); }
        write!(out, " ")
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
    fn table_empty() {
        let data: Vec<[usize; 0]> = vec![];

        let mut table = Table::new_builder(data, MinimalRenderer::new())
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!("", out);
    }


    #[test]
    fn array_table_single_row() {
        let data: Vec<[usize; 4]> = vec![
            [1, 10, 100, 1000],
        ];

        let mut table = Table::new_builder(data, MinimalRenderer::new())
            .with_column_selection(&[0, 2, 3])
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!("1 100 1000\n", out);
    }


    #[test]
    fn tuple_table_two_rows_short_column_descs() {
        let data: Vec<(i32, char)> = vec![
            (-17, 'b'),
            (170000, '&'),
        ];

        let specs = vec![
            ColumnDesc::new()
                .with_header("H0")
                .with_footer("F0"),
        ];
        let mut table = Table::new_builder(data, MinimalRenderer::new())
            .with_column_descs(&specs)
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!(out, "\
H0      
-17    b
170000 &
F0      
");
    }
    
    
    #[test]
    fn align_odd() {
        let data: Vec<[&str; 2]> = vec![
            ["1\n2\n3", "a\nb"],
        ];
        
        let col_descs = vec![
            ColumnDesc::new()
                .with_header("N"),
            ColumnDesc::new()
                .with_header("<-1->")
                .with_horz_align(HorzAlign::Left)
                .with_vert_align(VertAlign::Top),
            ColumnDesc::new()
                .with_header("<-2->")
                .with_horz_align(HorzAlign::Center)
                .with_vert_align(VertAlign::Center),
            ColumnDesc::new()
                .with_header("<-3->")
                .with_horz_align(HorzAlign::Right)
                .with_vert_align(VertAlign::Bottom),
        ];

        let mut table = Table::new_builder(data, MinimalRenderer::new())
            .with_column_selection(&[0, 1, 1, 1])
            .with_column_descs(&col_descs)
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        println!("{}", out);

        assert_eq!(out, "\
N <-1-> <-2-> <-3->
1 a       a        
2 b       b       a
3                 b
");
    }
    
    #[test]
    fn align_even() {
        let data: Vec<[&str; 2]> = vec![
            ["1\n2\n3\n4", "a\nb"],
        ];
        
        let col_descs = vec![
            ColumnDesc::new()
                .with_header("N"),
            ColumnDesc::new()
                .with_header("<-1->")
                .with_horz_align(HorzAlign::Right)
                .with_vert_align(VertAlign::Top),
            ColumnDesc::new()
                .with_header("<-2->")
                .with_horz_align(HorzAlign::Center)
                .with_vert_align(VertAlign::Center),
            ColumnDesc::new()
                .with_header("<-3->")
                .with_horz_align(HorzAlign::Left)
                .with_vert_align(VertAlign::Bottom),
        ];

        let mut table = Table::new_builder(data, MinimalRenderer::new())
            .with_column_selection(&[0, 1, 1, 1])
            .with_column_descs(&col_descs)
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        println!("{}", out);

        assert_eq!(out, "\
N <-1-> <-2-> <-3->
1     a            
2     b   a        
3         b   a    
4             b    
");
    }
    
    #[test]
    fn tuple_table_subselect() {
        let data: Vec<(f32, char)> = vec![
            ( 0.0, 'a'),
            (-1.9, 'b'),
            ( 2.8, 'c'),
            (-3.7, 'd'),
            ( 4.6, 'e'),
            (-5.5, 'f'),
            ( 6.4, 'g'),
            (-7.3, 'h'),
            ( 8.2, 'i'),
            (-9.1, 'j'),
        ];

        let col_descs = vec![
            ColumnDesc::new()
                .with_header("<-0->")
                .with_display_fmt(DisplayFmt::new()
                    .with_precision(2)
                    .with_sign(Sign::Plus))
                .with_horz_align(HorzAlign::Left)
                .with_vert_align(VertAlign::Top),
            ColumnDesc::new()
                .with_header("<-1->")
                .with_horz_align(HorzAlign::Center)
                .with_vert_align(VertAlign::Center),
        ];

        let mut table = Table::new_builder(data, MinimalRenderer::new())
            .with_row_selection(3..7)
            .with_column_descs(&col_descs)
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!(out, "\
<-0-> <-1->
-3.70   d  
+4.60   e  
-5.50   f  
+6.40   g  
");
    }
}
