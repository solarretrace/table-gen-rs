
# `table-gen` Table generator library 

## Usage

The basic usage is as follows:

    use table_gen_core::Table;
    use table_gen_render::MinimalRenderer;
    
    // Choose a Renderer implementation:
    let mut renderer = MinimalRenderer::new();
    
    // Define our data source via IntoIterator<item=R> where R: Row
    let data = vec![/* rows */]
    
    // Define where to write the table to
    let mut out = std::io::stdout();

    // Prepare the table.
    let mut table = Table::new_builder(data, &mut renderer)
        .finish();
    
    let res = table.render(&mut out);

You will usually want to extend this pattern by providing additional configuration for the table output. These can be specified before calling `finish` on the builder:

    // We can provide metadata for each column. The `ColumnDesc`s are provided
    // in order of the output index:
    let column_descs = vec![
        ColumnDesc::new()
            .with_header("Header")               // Header text for the column
            .with_footer("Footer")               // Footer text for the column
            .with_display_fmt(DisplayFmt::new()  // Formatting for cell values
                .with_precision(3)               // Precision for numerical cols
                .with_sign(Sign::Plus))          // Sign for numerical cols
            .with_min_width(10)                  // Minimum column width
            .with_max_width(15)                  // Maximum column width
            .with_horz_align(HorzAlign::Right)   // Horizontal alignment in cell
            .with_vert_align(VertAlign::Center), // Vertical alignment in cell
        // ... other rows will use default formatting (`ColumnDesc::new()`).
    ];
        
    // Prepare the table. The renderer is provided up front to provide its own
    // output requirements to the table driver.
    let mut table = Table::new_builder(data, &mut renderer)
        // We can specify the default column metadata:
        .with_default_column_desc(ColumnDesc::new()) 
        .with_column_descs(column_descs)
        // We can render only a subset of columns, choose the order, and even
        // render columns multiple times:
        .with_column_selection(&[0, 2, 4, 2])
        // We can sort the rows by choosing a list of columns to order by:
        .with_column_order(&[2, 1])
        // Finishing the builder will calculate column widths and materialize
        // the ordering.
        .finish();

## Architecture

The table-gen library generates a table through the following linear data flow sequence. 

    +---------------------------+
    | "Data Source"             |=> The data that will be iterated over to
    +---------------------------+   populate the table's data rows.
    | impl IntoIterator<Item=R> |
    |      where R: Row         |
    +---------------------------+
      |
      v
    +-----------+
    | Collate   |=> Responsible for specifying the column selection, headers, 
    +-----------+   footers, cell alignment, and other output formatting detail.
      |             The `Collate` structure maps output columns to their final
      v             order, simplifying the rest of the flows.
    +-----------+
    | Format    |=> Responsible for rendering the table cell text. This is to
    +-----------+   ensure the cells have a printable representation and a
      |             suitable format for text sorting.
      v
    +-----------+
    | Sort      |=> Materializes columns relevant to sorting. The sort order is
    +-----------+   provided by `Collate`, and only those column values are
      |             cached.
      v
    +-----------+
    | Split     |=> Responsible for splitting the cell text into lines.
    +-----------+
      |
      v
    +-----------+
    | Aggregate |=> Runs column-based aggregation on the rows of the table. This
    +-----------+   will materialize all non-fixed width columns as it computes
      |             the required widths for each column.
      v
    +----------+
    | "Driver" |=> Drives the table renderer by calling the renderer hook in the
    +----------+   appropriate order and with the cell line data. Forwards table
    | Table    |   formatting detail when relevant.
    +----------+
      |
      v
    +---------------+
    | "Render"      |=> Responsible for writing the table to output.
    +---------------+
    | impl Renderer |
    +---------------+

In total, there are five traits relevant to generating a table:

1. `Cell`: describes the requirements for data to populate a table cell. The data must implement `Display` and provide an analogue of `PartialCmp` that works on `&dyn Cell`. These are required to render the cell contents and support column sorting. Blanket implemented for all types that are `PartialOrd + Display + 'static`.

2. `Row`: implemented by data structures that want to provide rows of data to the table generator. Blanket implemented for arrays and tuples (for homogeneous and heterogeneous tables, respectively) and can also be implemented for custom structs & enums. The row length is a runtime value to support generating tables whose width is not known at compile time (such as data from a file.) The row length should not vary within a single render.

3. `IntoIterator<Item=R> where R: Row`: A table can be generated from any type that provides a sequence of `Row`s to render.

4. `Renderer`: provides an implementation of a table output format.

5. `std::io::Write`: provides a location for the table to render to.

For most use-cases, it should be very simple to generate a table with access to nothing more than a `Renderer`. A `Row` impl may be required for a custom data types, but the implementation will usually be obvious & trivial.

