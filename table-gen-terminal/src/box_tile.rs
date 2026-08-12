////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! A table renderer that renders tables using box-drawing unicode style with
//! distinct 'tiles' for each cell.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::LineStyle;

// Workspace library imports.
use table_gen::CellContext;
use table_gen::Features;
use table_gen::HorzAlign;
use table_gen::RenderContext;
use table_gen::Renderer;


////////////////////////////////////////////////////////////////////////////////
// BoxTileStyle
////////////////////////////////////////////////////////////////////////////////
/// The style specification for the box renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoxTileStyle {
    /// The style to use for the left table border.
    pub header: LineStyle,
    /// The style to use for the right table border.
    pub footer: LineStyle,
    /// The style to use for the top table border.
    pub data: LineStyle,
    /// Whether to use rounded corner variants.
    pub round_corners: bool,
}

impl Default for BoxTileStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl BoxTileStyle {
    /// Constructs a new `BoxTileStyle` with the default styling.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            header: LineStyle::Double,
            footer: LineStyle::Double,
            data: LineStyle::Light,
            round_corners: false,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// BoxTileRenderer
////////////////////////////////////////////////////////////////////////////////
/// A table renderer that renders tables using box-drawing unicode style.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct BoxTileRenderer {
    /// The amount of space to allocate between columns.
    column_padding: u8,
    /// The amount of extra space to allocate within columns.
    extra_width: u8,
    /// The `BoxTileStyle` to render with.
    style: BoxTileStyle,
}

impl Default for BoxTileRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl BoxTileRenderer {
    /// Constructs a new `BoxTileRenderer`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            column_padding: 0,
            extra_width: 0,
            style: BoxTileStyle::new(),
        }
    }

    /// Sets the column padding and returns the `MarkdownGridRenderer`.
    #[must_use]
    pub const fn with_column_padding(mut self, column_padding: u8) -> Self {
        self.column_padding = column_padding;
        self
    }

    /// Sets the extra column width and returns the `BoxTileRenderer`.
    #[must_use]
    pub const fn with_extra_width(mut self, extra_width: u8) -> Self {
        self.extra_width = extra_width;
        self
    }

    /// Sets the style and returns the `BoxTileRenderer`.
    #[must_use]
    pub const fn with_style(mut self, style: BoxTileStyle) -> Self {
        self.style = style;
        self
    }

    /// Writes a row divider line.
    fn write_div<W>(
        &self, 
        out: &mut W,
        ctx: &RenderContext<'_>,
        left: char,
        horz: char,
        right: char)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        let pad = std::cmp::max(self.column_padding / 2, 2);

        write!(out, "{}", left)?;
        for (col, col_width) in ctx.col_widths.iter().copied().enumerate() {
            for _ in 0..col_width { write!(out, "{}", horz)?; }
            for _ in 0..pad { write!(out, "{}", horz)?; }
            for _ in 0..self.extra_width { write!(out, "{}", horz)?; }
            write!(out, "{}", right)?;
            if col + 1 == ctx.column_count() { break; }
            write!(out, "{}", left)?;
        }
        Ok(())
    }

    /// Writes the left border of a line.
    fn write_border_left<W>(&self, out: &mut W, style: LineStyle)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        let pad = std::cmp::max(self.column_padding / 2, 2) / 2;
        
        write!(out, "{}", style.vert())?;
        for _ in 0..pad { write!(out, " ")?; }
        Ok(())
    }

    /// Writes the right border of a line.
    fn write_border_right<W>(&self, out: &mut W, style: LineStyle)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        let pad = std::cmp::max(self.column_padding / 2, 2) / 2;

        for _ in 0..pad { write!(out, " ")?; }
        write!(out, "{}", style.vert())?;
        Ok(())
    }

}

impl Renderer for BoxTileRenderer {
    fn features(&self) -> Features {
        Features::MULTILINE
    }

    fn write_header_start<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        if ctx.is_empty() { return Ok(()) }
        self.write_div(
            out,
            ctx,
            self.style.header.corner_top_left(
                self.style.header,
                self.style.round_corners),
            self.style.header.horz(),
            self.style.header.corner_top_right(
                self.style.header,
                self.style.round_corners))?;
        writeln!(out)
    }

    fn write_header_end<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        if ctx.is_empty() { return Ok(()) }
        self.write_div(
            out,
            ctx,
            self.style.header.corner_bottom_left(
                self.style.header,
                self.style.round_corners),
            self.style.header.horz(),
            self.style.header.corner_bottom_right(
                self.style.header,
                self.style.round_corners))?;
        writeln!(out)
    }

    fn write_data_cell_line<W>(
        &mut self,
        out: &mut W,
        _ctx: &RenderContext<'_>,
        cell: &CellContext<'_>)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        let mut pad = cell.padding();
        pad += self.extra_width as usize;
        let (l_pad, r_pad) = match cell.desc.horz_align {
            HorzAlign::Left   => (0,     pad),
            HorzAlign::Center => (pad/2, pad.div_ceil(2)),
            HorzAlign::Right  => (pad,   0),
        };
        
        for _ in 0..l_pad { write!(out, " ")?; }
        write!(out, "{}", cell.text)?;
        for _ in 0..r_pad { write!(out, " ")?; }
        Ok(())
    }
    
    fn write_header_cell_line_start<W>(
        &mut self,
        out: &mut W,
        _ctx: &RenderContext<'_>)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        self.write_border_left(out, self.style.header)
    }

    fn write_header_cell_line_end<W>(
        &mut self,
        out: &mut W,
        _ctx: &RenderContext<'_>)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        self.write_border_right(out, self.style.header)
    }

    fn write_data_cell_line_start<W>(
        &mut self,
        out: &mut W,
        _ctx: &RenderContext<'_>)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        self.write_border_left(out, self.style.data)
    }

    fn write_data_cell_line_end<W>(
        &mut self,
        out: &mut W,
        _ctx: &RenderContext<'_>)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        self.write_border_right(out, self.style.data)
    }

    fn write_footer_cell_line_start<W>(
        &mut self,
        out: &mut W,
        _ctx: &RenderContext<'_>)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        self.write_border_left(out, self.style.footer)
    }

    fn write_footer_cell_line_end<W>(
        &mut self,
        out: &mut W,
        _ctx: &RenderContext<'_>)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        self.write_border_right(out, self.style.footer)
    }


    fn write_data_row_start<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        if ctx.is_empty() { return Ok(()) }
        self.write_div(
            out,
            ctx,
            self.style.data.corner_top_left(
                self.style.data,
                self.style.round_corners),
            self.style.data.horz(),
            self.style.data.corner_top_right(
                self.style.data,
                self.style.round_corners))?;
        writeln!(out)
    }

    fn write_data_row_end<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        if ctx.is_empty() { return Ok(()) }
        self.write_div(
            out,
            ctx,
            self.style.data.corner_bottom_left(
                self.style.data,
                self.style.round_corners),
            self.style.data.horz(),
            self.style.data.corner_bottom_right(
                self.style.data,
                self.style.round_corners))?;
        writeln!(out)
    }

    fn write_footer_start<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        if ctx.is_empty() { return Ok(()) }
        self.write_div(
            out,
            ctx,
            self.style.footer.corner_top_left(
                self.style.footer,
                self.style.round_corners),
            self.style.footer.horz(),
            self.style.footer.corner_top_right(
                self.style.footer,
                self.style.round_corners))?;
        writeln!(out)
    }

    fn write_footer_end<W>(&mut self, out: &mut W, ctx: &RenderContext<'_>)
        -> std::io::Result<()>
        where W: std::io::Write
    {
        if ctx.is_empty() { return Ok(()) }
        self.write_div(
            out,
            ctx,
            self.style.footer.corner_bottom_left(
                self.style.footer,
                self.style.round_corners),
            self.style.footer.horz(),
            self.style.footer.corner_bottom_right(
                self.style.footer,
                self.style.round_corners))?;
        writeln!(out)
    }
}


////////////////////////////////////////////////////////////////////////////////
// Test module
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test {
    use super::*;
    use table_gen::ColumnDesc;
    use table_gen::ColumnOrd;
    use table_gen::Table;

    #[test]
    fn empty_table() {
        let data: Vec<(usize, )> = vec![];

        let mut table = Table::new_builder(data, BoxTileRenderer::new())
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!(out, "");
    }

    #[test]
    fn array_table_single_column() {
        let data: Vec<[usize; 1]> = vec![
            [1],
            [2000],
            [10],
            [30000],
            [100],
        ];

        let order = [ColumnOrd::new(0).with_reversed_order()];
        let mut table = Table::new_builder(data, BoxTileRenderer::new())
            .with_sort_columns(&order)
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!(out, "\
┌───────┐
│ 30000 │
└───────┘
┌───────┐
│ 2000  │
└───────┘
┌───────┐
│ 100   │
└───────┘
┌───────┐
│ 10    │
└───────┘
┌───────┐
│ 1     │
└───────┘
");
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

        let mut table = Table::new_builder(data, BoxTileRenderer::new())
            .with_column_descs(&col_descs)
            .with_column_selection(&[0, 0, 0])
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!(out, "\
╔═══════╗╔═══════╗╔════════╗
║ Right ║║ Left  ║║ Center ║
╚═══════╝╚═══════╝╚════════╝
┌───────┐┌───────┐┌────────┐
│    12 ││ 12    ││   12   │
└───────┘└───────┘└────────┘
┌───────┐┌───────┐┌────────┐
│   123 ││ 123   ││  123   │
└───────┘└───────┘└────────┘
┌───────┐┌───────┐┌────────┐
│     1 ││ 1     ││   1    │
└───────┘└───────┘└────────┘
┌───────┐┌───────┐┌────────┐
│ -8000 ││ -8000 ││ -8000  │
└───────┘└───────┘└────────┘
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

        let mut table = Table::new_builder(data, BoxTileRenderer::new())
            .with_column_descs(&col_descs)
            .with_column_selection(&[0, 0, 0])
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!(out, "\
┌───────┐┌───────┐┌───────┐
│    12 ││ 12    ││  12   │
└───────┘└───────┘└───────┘
┌───────┐┌───────┐┌───────┐
│   123 ││ 123   ││  123  │
└───────┘└───────┘└───────┘
┌───────┐┌───────┐┌───────┐
│     1 ││ 1     ││   1   │
└───────┘└───────┘└───────┘
┌───────┐┌───────┐┌───────┐
│ -8000 ││ -8000 ││ -8000 │
└───────┘└───────┘└───────┘
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

        let mut table = Table::new_builder(data, BoxTileRenderer::new())
            .with_column_descs(&col_descs)
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!(out, "\
╔══════════╗╔═════════╗╔═════════╗╔══════════════════════════╗
║ Centered ║║ Left    ║║   Right ║║ Left                     ║
║  Header  ║║ Aligned ║║ Aligned ║║ Aligned                  ║
╚══════════╝╚═════════╝╚═════════╝╚══════════════════════════╝
┌──────────┐┌─────────┐┌─────────┐┌──────────────────────────┐
│  First   ││ row     ││      12 ││ Example of a row that    │
│          ││         ││         ││ spans multiple lines.    │
└──────────┘└─────────┘└─────────┘└──────────────────────────┘
┌──────────┐┌─────────┐┌─────────┐┌──────────────────────────┐
│  Second  ││ row     ││       5 ││ Here's another one. Note │
│          ││         ││         ││ the blank line between   │
│          ││         ││         ││ rows                     │
└──────────┘└─────────┘└─────────┘└──────────────────────────┘
");
    }


    #[test]
    fn multiline_table_alt() {
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

        let mut table = Table::new_builder(data, BoxTileRenderer::new()
                .with_style(BoxTileStyle {
                    header: LineStyle::Heavy,
                    footer: LineStyle::Light,
                    data: LineStyle::LightDash4,
                    round_corners: true,
                }))
            .with_column_descs(&col_descs)
            .finish();

        let mut out: Vec<u8> = Vec::new();
        assert!(table.render(&mut out).is_ok());
        let out = String::from_utf8(out).unwrap();
        //println!("{}", out);

        assert_eq!(out, "\
┏━━━━━━━━━━┓┏━━━━━━━━━┓┏━━━━━━━━━┓┏━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ Centered ┃┃ Left    ┃┃   Right ┃┃ Left                     ┃
┃  Header  ┃┃ Aligned ┃┃ Aligned ┃┃ Aligned                  ┃
┗━━━━━━━━━━┛┗━━━━━━━━━┛┗━━━━━━━━━┛┗━━━━━━━━━━━━━━━━━━━━━━━━━━┛
╭┈┈┈┈┈┈┈┈┈┈╮╭┈┈┈┈┈┈┈┈┈╮╭┈┈┈┈┈┈┈┈┈╮╭┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈╮
┊  First   ┊┊ row     ┊┊      12 ┊┊ Example of a row that    ┊
┊          ┊┊         ┊┊         ┊┊ spans multiple lines.    ┊
╰┈┈┈┈┈┈┈┈┈┈╯╰┈┈┈┈┈┈┈┈┈╯╰┈┈┈┈┈┈┈┈┈╯╰┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈╯
╭┈┈┈┈┈┈┈┈┈┈╮╭┈┈┈┈┈┈┈┈┈╮╭┈┈┈┈┈┈┈┈┈╮╭┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈╮
┊  Second  ┊┊ row     ┊┊       5 ┊┊ Here's another one. Note ┊
┊          ┊┊         ┊┊         ┊┊ the blank line between   ┊
┊          ┊┊         ┊┊         ┊┊ rows                     ┊
╰┈┈┈┈┈┈┈┈┈┈╯╰┈┈┈┈┈┈┈┈┈╯╰┈┈┈┈┈┈┈┈┈╯╰┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈╯
");
    }
}
