
# Documentation & API tasks:

+ Write API documentation

	pub struct SupportFlags: u16 {
		/// Indicates that the renderer supports header rendering.
		const HEADERS                  = 1;
		/// Indictaes that the renderer supports footer rendering.
		const FOOTERS                  = 1 << 1;
		/// Indicates that the renderer supports column widths.
		const COLUMN_WIDTH             = 1 << 2;
		/// Indicates that the renderer supports column widths constraints.
		const COLUMN_WIDTH_CONSTRAINTS = 1 << 3 | 1 << 2;
		/// Indicates that the renderer supports dynamic column widths.
		const COLUMN_WIDTH_DYNAMIC     = 1 << 4 | 1 << 2;
		/// Indicates that the renderer supports horizontal column alignment.
		const HORZ_ALIGN               = 1 << 5 | 1 << 2;
		/// Indicates that the renderer supports multiline rows & cells.
		const MULTILINE                = 1 << 6;
		/// Indicates that the renderer supports vertical alignment of multiline
		/// rows and cells.
		const VERT_ALIGN               = 1 << 7 | 1 << 6;
		/// Indicates that the renderer supports text wrapping.
		const TEXT_WRAP                = 1 << 8 | 1 << 6;
		/// Indicates that the renderer supports ANSI styling.
		const ANSI_STYLE               = 1 << 9;
	}

# Implementation tasks:

+ Expand ColumnDefs as feature requests mechanism.
+ Support text coloring
+ Support outer border elision in terminal renderers
+ Support independent header/footer alignment
+ Implement parallelism
+ Implement custom aggregation
+ Support subcolumns
	- Splice adapter
	- Support multiple header/footer rows
	- Support joined column rendering
+ Implement more robust formatting options

# Deferred tasks:

+ Deduplicate sort indices
	- Requires copying the order
	- little payoff
+ Implement Transpose operation
	- Can no longer expose ColumnDesc array to renderers.
	- Need a way to request column descriptors dynamically.
+ Deduplicate column references
	- Unnecessary. Duplicate columns can have different formatting once features are implemented.
+ Forced column narrowing
	- Unnecessary. You could just remove min widths from ColumnDefs.

