
# Documentation & API tasks:

+ Write API documentation

	let data: Vec<(i64, f64, bool, &str)> = vec![
		(15,  0.0,     false, "A single line column"),
		(-15, 18.0001, true,  "A two-\nline column"),
		(0,   18e4,    true,  "A\nmulti-\nline\ncolumn"),
	];

	let col_descs = vec![
		ColumnDesc::new()
			.with_header("i64\nvalues (wide)")
			.with_footer("COLUMN 0")
			.with_min_width(18),
		ColumnDesc::new()
			.with_header("f64\nvalues")
			.with_footer("COLUMN 1")
			.with_display_fmt(DisplayFmt::new()
				.with_precision(3)
				.with_sign(Sign::Plus))
			.with_horz_align(HorzAlign::Center)
			.with_vert_align(VertAlign::Top),
		ColumnDesc::new()
			.with_header("bool\nvalues")
			.with_footer("COLUMN 2")
			.with_horz_align(HorzAlign::Right)
			.with_vert_align(VertAlign::Center),
		ColumnDesc::new()
			.with_header("left-aligned\nstrings")
			.with_footer("COLUMN 3")
			.with_horz_align(HorzAlign::Left)
			.with_vert_align(VertAlign::Top),
		ColumnDesc::new()
			.with_header("bool\nagain")
			.with_footer("COLUMN 4")
			.with_horz_align(HorzAlign::Right)
			.with_vert_align(VertAlign::Bottom),
		ColumnDesc::new()
			.with_header("right-aligned\nstrings")
			.with_footer("COLUMN 5")
			.with_horz_align(HorzAlign::Right)
			.with_vert_align(VertAlign::Bottom)
			.with_max_width(10),
	];
	let order = [ColumnOrd::new(1).with_reversed_order(), ColumnOrd::new(2)];
		
	let mut table = Table::new_builder(data, MinimalRenderer::new())
		.with_column_descs(&col_descs)
		.with_column_selection(&[0, 1, 2, 3, 2, 3])
		.with_sort_columns(&order)
		.finish();


# Implementation tasks:
.
+ Implement Features
	- Support truncating long values
	- Support custom line breaker/wrapping	
	- Support unicode string widths
	- Support extra width in aggregator
	- Support custom formatting
+ Support full table width constraints
+ Implement colors for terminal renderers
+ Deduplicate column references
+ Support independent header/footer alignment
+ Implement parallelism
+ Implement custom aggregation
+ Support subcolumns
+ Support multiple header/footer rows
+ Implement more robust formatting options

# Deferred tasks:

+ Deduplicate sort indices
	- Requires copying the order
	- little payoff
+ Implement Transpose operation
	- Can no longer expose ColumnDesc array to renderers.
	- Need a way to request column descriptors dynamically.
