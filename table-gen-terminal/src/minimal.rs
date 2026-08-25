////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! A table renderer that renders tables with minimal decoration.
////////////////////////////////////////////////////////////////////////////////

// Workspace library imports.
use table_gen::Features;
use table_gen::RenderContext;
use table_gen::Renderer;


////////////////////////////////////////////////////////////////////////////////
// MinimalRenderer
////////////////////////////////////////////////////////////////////////////////
/// A table renderer that renders tables with minimal decoration.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, Default)]
pub struct MinimalRenderer;

impl MinimalRenderer {
	/// Constructs a new `MinimalRenderer`.
	#[must_use]
	pub fn new() -> Self {
		Self
	}
}

impl Renderer for MinimalRenderer {
	fn features(&self) -> Features {
		Features::default()
	}

	fn write_data_cell_line_end<W>(
		&mut self,
		out: &mut W,
		ctx: &RenderContext<'_>)
		-> std::io::Result<()>
		where W: std::io::Write
	{
		if ctx.is_last_column() { return Ok(()); }
		write!(out, " ")
	}
}


////////////////////////////////////////////////////////////////////////////////
// Test module
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test {
	use super::*;
	use table_gen::ColumnDef;
	use table_gen::ColumnOrd;
	use table_gen::DisplayFmt;
	use table_gen::HorzAlign;
	use table_gen::Sign;
	use table_gen::Table;
	use table_gen::VertAlign;

	#[test]
	fn empty_table() {
		let data: Vec<[usize; 0]> = vec![];

		let mut table = Table::new_builder(data, MinimalRenderer::new())
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "");
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

		assert_eq!(out, "1 100 1000\n");
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
		let mut table = Table::new_builder(data, MinimalRenderer::new())
			.with_sort_columns(&order)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
30000
2000 
100  
10   
1    
");
	}

	#[test]
	fn tuple_table_two_rows_short_column_defs() {
		let data: Vec<(i32, char)> = vec![
			(-17, 'b'),
			(170000, '&'),
		];

		let specs = vec![
			ColumnDef::new()
				.with_header("H0")
				.with_footer("F0"),
		];
		let mut table = Table::new_builder(data, MinimalRenderer::new())
			.with_column_defs(&specs)
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
		
		let column_defs = vec![
			ColumnDef::new()
				.with_header("N"),
			ColumnDef::new()
				.with_header("<-1->")
				.with_horz_align(HorzAlign::Left)
				.with_vert_align(VertAlign::Top),
			ColumnDef::new()
				.with_header("<-2->")
				.with_horz_align(HorzAlign::Center)
				.with_vert_align(VertAlign::Center),
			ColumnDef::new()
				.with_header("<-3->")
				.with_horz_align(HorzAlign::Right)
				.with_vert_align(VertAlign::Bottom),
		];

		let mut table = Table::new_builder(data, MinimalRenderer::new())
			.with_column_selection(&[0, 1, 1, 1])
			.with_column_defs(&column_defs)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

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
		
		let column_defs = vec![
			ColumnDef::new()
				.with_header("N"),
			ColumnDef::new()
				.with_header("<-1->")
				.with_horz_align(HorzAlign::Right)
				.with_vert_align(VertAlign::Top),
			ColumnDef::new()
				.with_header("<-2->")
				.with_horz_align(HorzAlign::Center)
				.with_vert_align(VertAlign::Center),
			ColumnDef::new()
				.with_header("<-3->")
				.with_horz_align(HorzAlign::Left)
				.with_vert_align(VertAlign::Bottom),
		];

		let mut table = Table::new_builder(data, MinimalRenderer::new())
			.with_column_selection(&[0, 1, 1, 1])
			.with_column_defs(&column_defs)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
N <-1-> <-2-> <-3->
1     a            
2     b   a        
3         b   a    
4             b    
");
	}
	
	#[test]
	fn tuple_table_subselect_float() {
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

		let column_defs = vec![
			ColumnDef::new()
				.with_header("<-0->")
				.with_display_fmt(DisplayFmt::new()
					.with_precision(2)
					.with_sign(Sign::Plus))
				.with_horz_align(HorzAlign::Left)
				.with_vert_align(VertAlign::Top),
			ColumnDef::new()
				.with_header("<-1->")
				.with_horz_align(HorzAlign::Center)
				.with_vert_align(VertAlign::Center),
		];

		let mut table = Table::new_builder(data, MinimalRenderer::new())
			.with_row_selection(3..7)
			.with_column_defs(&column_defs)
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

	#[test]
	fn demo_table() {
		let data: Vec<(i64, f64, bool, &str)> = vec![
			(15,  0.0,     false, "A single line column"),
			(-15, 18.0001, true,  "A two-\nline column"),
			(0,   18e4,    true,  "A\nmulti-\nline\ncolumn"),
		];

		let column_defs = vec![
			ColumnDef::new()
				.with_header("i64\nvalues (wide)")
				.with_footer("COLUMN 0")
				.with_min_width(18),
			ColumnDef::new()
				.with_header("f64\nvalues")
				.with_footer("COLUMN 1")
				.with_display_fmt(DisplayFmt::new()
					.with_precision(3)
					.with_sign(Sign::Plus))
				.with_horz_align(HorzAlign::Center)
				.with_vert_align(VertAlign::Top),
			ColumnDef::new()
				.with_header("bool\nvalues")
				.with_footer("COLUMN 2")
				.with_horz_align(HorzAlign::Right)
				.with_vert_align(VertAlign::Center),
			ColumnDef::new()
				.with_header("left-aligned\nstrings")
				.with_footer("COLUMN 3")
				.with_horz_align(HorzAlign::Left)
				.with_vert_align(VertAlign::Top),
			ColumnDef::new()
				.with_header("bool\nagain")
				.with_footer("COLUMN 4")
				.with_horz_align(HorzAlign::Right)
				.with_vert_align(VertAlign::Bottom),
			ColumnDef::new()
				.with_header("right-aligned\nstrings")
				.with_footer("COLUMN 5")
				.with_horz_align(HorzAlign::Right)
				.with_vert_align(VertAlign::Bottom)
				.with_max_width(10),
		];
		let order = [ColumnOrd::new(1).with_reversed_order(), ColumnOrd::new(2)];
			
		let mut table = Table::new_builder(data, MinimalRenderer::new())
			.with_column_defs(&column_defs)
			.with_column_selection(&[0, 1, 2, 3, 2, 3])
			.with_sort_columns(&order)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
i64                    f64         bool left-aligned             bool …t-aligned
values (wide)        values      values strings                 again    strings
0                  +180000.000          A                                      A
                                   true multi-                            multi-
                                        line                                line
                                        column                   true     column
-15                  +18.000       true A two-                            A two-
                                        line column              true …ne column
15                   +0.000       false A single line column    false …ne column
COLUMN 0            COLUMN 1   COLUMN 2 COLUMN 3             COLUMN 4   COLUMN 5
");
	}

	#[test]
	fn cicero() {
		let data: Vec<[&str; 4]> = vec![
			[
				"Sed ut perspiciatis",
				"unde omnis iste natus error",
				"sit voluptatem accusantium",
				"doloremque laudantium, totam rem aperiam,",
			],
			[
				"eaque ipsa quae",
				"ab illo inventore veritatis et",
				"quasi architecto beatae",
				"vitae dicta sunt explicabo. Nemo enim ipsam",
			],
			[
				"voluptatem quia voluptas",
				"sit aspernatur aut odit aut",
				"fugit, sed",
				"quia consequuntur magni dolores eos qui ratione",
			],
			[
				"voluptatem sequi nesciunt",
				"Neque porro quisquam est,",
				"qui dolorem ipsum",
				"quia dolor sit amet, consectetur, adipisci",
			],
			[
				"velit, sed",
				"quia non numquam eius modi",
				"tempora incidunt ut",
				"labore et dolore magnam aliquam quaerat voluptatem. Ut",
			],
			[
				"enim ad minima",
				"veniam, quis nostrum exercitationem",
				"ullam corporis suscipit",
				"laboriosam, nisi ut aliquid ex ea",
			],
			[
				"commodi consequatur?",
				"Quis autem vel eum iure",
				"reprehenderit qui in",
				"ea voluptate velit esse quam nihil molestiae",
			],
			[
				"consequatur, vel",
				"illum qui dolorem eum fugiat",
				"quo voluptas nulla",
				"pariatur?",
			],
		];
		
		let column_defs = vec![
			ColumnDef::new()
				.with_header("COLUMN A")
				.with_footer("COLUMN A"),
			ColumnDef::new()
				.with_header("COLUMN B")
				.with_footer("COLUMN B"),
			ColumnDef::new()
				.with_header("COLUMN C")
				.with_footer("COLUMN C"),
			ColumnDef::new()
				.with_header("COLUMN D")
				.with_footer("COLUMN D"),
		];

		let mut table = Table::new_builder(data, MinimalRenderer::new())
			.with_column_defs(&column_defs)
			.with_max_table_width(80)
			.finish();

		let mut out: Vec<u8> = Vec::new();
		assert!(table.render(&mut out).is_ok());
		let out = String::from_utf8(out).unwrap();
		//println!("{}", out);

		assert_eq!(out, "\
COLUMN A  COLUMN B            COLUMN C   COLUMN D                               
Sed ut p… unde omnis iste na… sit volup… doloremque laudantium, totam rem aperi…
eaque ip… ab illo inventore … quasi arc… vitae dicta sunt explicabo. Nemo enim …
voluptat… sit aspernatur aut… fugit, sed quia consequuntur magni dolores eos qu…
voluptat… Neque porro quisqu… qui dolor… quia dolor sit amet, consectetur, adip…
velit, s… quia non numquam e… tempora i… labore et dolore magnam aliquam quaera…
enim ad … veniam, quis nostr… ullam cor… laboriosam, nisi ut aliquid ex ea      
commodi … Quis autem vel eum… reprehend… ea voluptate velit esse quam nihil mol…
consequa… illum qui dolorem … quo volup… pariatur?                              
COLUMN A  COLUMN B            COLUMN C   COLUMN D                               
");
	}
}
