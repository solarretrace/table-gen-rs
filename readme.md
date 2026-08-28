
# `table-gen` -- Table generator library 

A Rust library for generating formatted tables from iterable data sources.

## Usage

The basic usage is as follows:

```rust
use table_gen_core::Table;
use table_gen_terminal::MinimalRenderer;

// Choose a Renderer implementation:
let mut renderer = MinimalRenderer::new();

// Define our data source via IntoIterator<item=R> where R: Row
let data = vec![/* rows */];

// Define where to write the table to
let mut out = std::io::stdout();

// Prepare the table.
let mut table = Table::new_builder(data, &mut renderer)
    .finish();

let res = table.render(&mut out);
```

You will usually want to extend this pattern by providing additional configuration for the table output. These can be specified before calling `finish` on the builder:

```rust
// We can provide metadata for each column. The `ColumnDef`s are provided
// in order of the output index:
let column_defs = vec![
    ColumnDef::new()
        .with_header("Header")              // Header text for the column
        .with_footer("Footer")              // Footer text for the column
        .with_display_fmt(DisplayFmt::new() // Formatting for cell values
            .with_precision(3)              // Precision for numerical columns
            .with_sign(Sign::Plus))         // Sign for numerical columns
        .with_min_width(10)                 // Minimum column width
        .with_max_width(15)                 // Maximum column width
        .with_horz_align(HorzAlign::Right)  // Horizontal alignment in cell
        .with_vert_align(VertAlign::Center) // Vertical alignment in cell
        .with_dynamic_width_quantile(0.9)   // Ignore widest outlier cells
        .with_dynamic_width_weight(2.0),    // Try to avoid narrowing the column
    // ... other rows will use default formatting (`ColumnDef::new()`).
];

// We can provide a multicolumn sort specification:
let sort_columns = vec![
    ColumnOrd::new(2)           // Sort on index 2 of the output
        .with_reversed_order(), // Reverse the sort ordering
    ColumnOrd::new(1)           // Then sort on index 1 of the output
        .with_formatted_order() // Order by column text, not value
        .with_none_lt_order(),  // Compare `None` values as less than others.
];

// Prepare the table. The renderer is provided up front to provide its own
// output requirements to the table driver.
let mut table = Table::new_builder(data, &mut renderer)

    // We can specify the default column metadata:
    .with_default_column_def(ColumnDef::new()) 
    .with_column_defs(&column_defs)

    // We can render only a subset of columns, choose the order, and even
    // render columns multiple times:
    .with_column_selection(&[0, 2, 4, 2])
    
    // We can sort the rows by choosing a list of columns to order by:
    .with_sort_columns(&sort_columns)
    
    // Row subselect via range. This will render only these rows after sorting.
    .with_row_selection(5..15)
    
    // We can set width constraints for the table, but individual columns will
    // never exceed their minimum or maximum width.
    .with_min_table_width(10)
    .with_max_table_width(100)
    
    // Diagnostic messages can be received via closure:
    .with_diagnositic_sink_fn(|msg| println!("{}", msg))
    
    // Finishing the builder will calculate column widths and materialize
    // the ordering.
    .finish();
```

## Architecture

The table-gen library generates a table through the following linear data flow sequence. 

    ┌───────────────────────────┐
    │ "Data Source"             ╞═> The data that will be iterated over to
    ├───────────────────────────┤   populate the table's data rows.
    │ impl IntoIterator<Item=R> │
    │      where R: Row         │
    ╰┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈╯
      │
      v
    ╭┈┈┈┈┈┈┈┈┈┈┈╮
    │ Collate   ╞═> Responsible for specifying the column selection, headers, 
    ╰┈┈┈┈┈┈┈┈┈┈┈╯   footers, cell alignment, and other output formatting detail.
      │             The `Collate` structure maps output columns to their final
      v             order, simplifying the rest of the flows.
    ╭┈┈┈┈┈┈┈┈┈┈┈╮
    │ Format    ╞═> Responsible for rendering the table cell text. This is to
    ╰┈┈┈┈┈┈┈┈┈┈┈╯   ensure the cells have a printable representation and a
      │             suitable format for text sorting.
      v
    ╭┈┈┈┈┈┈┈┈┈┈┈╮
    │ Sort      ╞═> Materializes columns relevant to sorting. The sort order is
    ╰┈┈┈┈┈┈┈┈┈┈┈╯   provided by `Collate`, and only those column values are
      │             cached.
      v
    ╭┈┈┈┈┈┈┈┈┈┈┈╮
    │ Aggregate ╞═> Runs column-based aggregation on the rows of the table. This
    ╰┈┈┈┈┈┈┈┈┈┈┈╯   will compute the required column widths and apply post-width
      │             formatting such as line wrapping.
      v
    ┌──────────┐
    │ "Driver" ╞═> Drives the table renderer by calling the renderer hooks in
    ├──────────┤   the appropriate order and with the cell line data. Forwards
    │ Table    │   table formatting detail when relevant.
    ╰┈┈┈┈┈┈┈┈┈┈╯
      │
      v
    ┌───────────────┐
    │ "Render"      ╞═> Responsible for writing the table to output.
    ├───────────────┤
    │ impl Renderer │
    ╰┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈╯

In total, there are five traits relevant to generating a table:

1. `Cell`: describes the requirements for data to populate a table cell. The data must implement `Display` and provide an analogue of `PartialCmp` that works on `&dyn Cell`. These are required to render the cell contents and support column sorting. Blanket implemented for all types that are `PartialOrd + Display + 'static`.

2. `Row`: implemented by data structures that want to provide rows of data to the table generator. Blanket implemented for arrays and tuples (for homogeneous and heterogeneous tables, respectively) and can also be implemented for custom structs & enums. The row length is a runtime value to support generating tables whose width is not known at compile time (such as data from a file.) The row length should not vary within a single render.

3. `IntoIterator<Item=R> where R: Row`: A table can be generated from any type that provides a sequence of `Row`s to render.

4. `Renderer`: provides an implementation of a table output format.

5. `std::io::Write`: provides a location for the table to render to.

For most use-cases, it should be very simple to generate a table with access to nothing more than a `Renderer`. A `Row` impl may be required for a custom data types, but the implementation will usually be obvious & trivial.


## Examples

The following are demonstrations of the default output of available renderers:

### Minimal renderer

`table_gen_terminal::MinimalRenderer`:

```
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
```

### Box-drawing grid renderer

`table_gen_terminal::BoxGridRenderer`:

```
┌─────────┬──────────────────┬──────────┬──────────────────────────────────────┐
│ COLUMN… ┆ COLUMN B         ┆ COLUMN C ┆ COLUMN D                             │
╞═════════╪══════════════════╪══════════╪══════════════════════════════════════╡
│ Sed ut  ┆ unde omnis iste  ┆ sit      ┆ doloremque laudantium, totam rem     │
│ perspi… ┆ natus error      ┆ volupta… ┆ aperiam,                             │
│         ┆                  ┆ accusan… ┆                                      │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ eaque   ┆ ab illo          ┆ quasi    ┆ vitae dicta sunt explicabo. Nemo     │
│ ipsa    ┆ inventore        ┆ archite… ┆ enim ipsam                           │
│ quae    ┆ veritatis et     ┆ beatae   ┆                                      │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ volupt… ┆ sit aspernatur   ┆ fugit,   ┆ quia consequuntur magni dolores eos  │
│ quia    ┆ aut odit aut     ┆ sed      ┆ qui ratione                          │
│ volupt… ┆                  ┆          ┆                                      │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ volupt… ┆ Neque porro      ┆ qui      ┆ quia dolor sit amet, consectetur,    │
│ sequi   ┆ quisquam est,    ┆ dolorem  ┆ adipisci                             │
│ nesciu… ┆                  ┆ ipsum    ┆                                      │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ velit,  ┆ quia non numquam ┆ tempora  ┆ labore et dolore magnam aliquam      │
│ sed     ┆ eius modi        ┆ incidunt ┆ quaerat voluptatem. Ut               │
│         ┆                  ┆ ut       ┆                                      │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ enim ad ┆ veniam,          ┆ ullam    ┆ laboriosam, nisi ut aliquid ex ea    │
│ minima  ┆ quis nostrum     ┆ corporis ┆                                      │
│         ┆ exercitationem   ┆ suscipit ┆                                      │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ commodi ┆ Quis autem vel   ┆ reprehe… ┆ ea voluptate velit esse quam nihil   │
│ conseq… ┆ eum iure         ┆ qui in   ┆ molestiae                            │
├─────────┼──────────────────┼──────────┼──────────────────────────────────────┤
│ conseq… ┆ illum qui        ┆ quo      ┆ pariatur?                            │
│ vel     ┆ dolorem eum      ┆ voluptas ┆                                      │
│         ┆ fugiat           ┆ nulla    ┆                                      │
╞═════════╪══════════════════╪══════════╪══════════════════════════════════════╡
│ COLUMN… ┆ COLUMN B         ┆ COLUMN C ┆ COLUMN D                             │
└─────────┴──────────────────┴──────────┴──────────────────────────────────────┘
```

### Box-drawing tile renderer

`table_gen_terminal::BoxTileRenderer`:

```
╔════════╗╔══════════════════╗╔═════════╗╔═════════════════════════════════════╗
║ COLUM… ║║ COLUMN B         ║║ COLUMN… ║║ COLUMN D                            ║
╚════════╝╚══════════════════╝╚═════════╝╚═════════════════════════════════════╝
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ Sed ut ││ unde omnis iste  ││ sit     ││ doloremque laudantium, totam rem    │
│ persp… ││ natus error      ││ volupt… ││ aperiam,                            │
│        ││                  ││ accusa… ││                                     │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ eaque  ││ ab illo          ││ quasi   ││ vitae dicta sunt explicabo. Nemo    │
│ ipsa   ││ inventore        ││ archit… ││ enim ipsam                          │
│ quae   ││ veritatis et     ││ beatae  ││                                     │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ volup… ││ sit aspernatur   ││ fugit,  ││ quia consequuntur magni dolores eos │
│ quia   ││ aut odit aut     ││ sed     ││ qui ratione                         │
│ volup… ││                  ││         ││                                     │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ volup… ││ Neque porro      ││ qui     ││ quia dolor sit amet, consectetur,   │
│ sequi  ││ quisquam est,    ││ dolorem ││ adipisci                            │
│ nesci… ││                  ││ ipsum   ││                                     │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ velit, ││ quia non numquam ││ tempora ││ labore et dolore magnam aliquam     │
│ sed    ││ eius modi        ││ incidu… ││ quaerat voluptatem. Ut              │
│        ││                  ││ ut      ││                                     │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ enim   ││ veniam,          ││ ullam   ││ laboriosam, nisi ut aliquid ex ea   │
│ ad     ││ quis nostrum     ││ corpor… ││                                     │
│ minima ││ exercitationem   ││ suscip… ││                                     │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ commo… ││ Quis autem vel   ││ repreh… ││ ea voluptate velit esse quam nihil  │
│ conse… ││ eum iure         ││ qui in  ││ molestiae                           │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
┌────────┐┌──────────────────┐┌─────────┐┌─────────────────────────────────────┐
│ conse… ││ illum qui        ││ quo     ││ pariatur?                           │
│ vel    ││ dolorem eum      ││ volupt… ││                                     │
│        ││ fugiat           ││ nulla   ││                                     │
└────────┘└──────────────────┘└─────────┘└─────────────────────────────────────┘
╔════════╗╔══════════════════╗╔═════════╗╔═════════════════════════════════════╗
║ COLUM… ║║ COLUMN B         ║║ COLUMN… ║║ COLUMN D                            ║
╚════════╝╚══════════════════╝╚═════════╝╚═════════════════════════════════════╝
```


### Pandoc 'Simple' Markdown renderer

`table_gen_markdown::MarkdownSimpleRenderer`:

```
COLUMN A  COLUMN B            COLUMN C   COLUMN D                               
--------- ------------------- ---------- ---------------------------------------
Sed ut p… unde omnis iste na… sit volup… doloremque laudantium, totam rem aperi…
eaque ip… ab illo inventore … quasi arc… vitae dicta sunt explicabo. Nemo enim …
voluptat… sit aspernatur aut… fugit, sed quia consequuntur magni dolores eos qu…
voluptat… Neque porro quisqu… qui dolor… quia dolor sit amet, consectetur, adip…
velit, s… quia non numquam e… tempora i… labore et dolore magnam aliquam quaera…
enim ad … veniam, quis nostr… ullam cor… laboriosam, nisi ut aliquid ex ea      
commodi … Quis autem vel eum… reprehend… ea voluptate velit esse quam nihil mol…
consequa… illum qui dolorem … quo volup… pariatur?                              
COLUMN A  COLUMN B            COLUMN C   COLUMN D                               
```


### Pandoc 'Pipe' Markdown renderer

`table_gen_markdown::MarkdownPipeRenderer`:

```
| COLUMN… | COLUMN B         | COLUMN C | COLUMN D                             |
|---------|:-----------------|:---------|:-------------------------------------|
| Sed ut… | unde omnis iste… | sit vol… | doloremque laudantium, totam rem ap… |
| eaque … | ab illo invento… | quasi a… | vitae dicta sunt explicabo. Nemo en… |
| volupt… | sit aspernatur … | fugit, … | quia consequuntur magni dolores eos… |
| volupt… | Neque porro qui… | qui dol… | quia dolor sit amet, consectetur, a… |
| velit,… | quia non numqua… | tempora… | labore et dolore magnam aliquam qua… |
| enim a… | veniam, quis no… | ullam c… | laboriosam, nisi ut aliquid ex ea    |
| commod… | Quis autem vel … | reprehe… | ea voluptate velit esse quam nihil … |
| conseq… | illum qui dolor… | quo vol… | pariatur?                            |
| COLUMN… | COLUMN B         | COLUMN C | COLUMN D                             |
```

### Pandoc 'Multiline' Markdown renderer

`table_gen_markdown::MarkdownMultilineRenderer`:

```
--------------------------------------------------------------------------------
COLUMN A  COLUMN B            COLUMN C   COLUMN D
--------- ------------------- ---------- ---------------------------------------
Sed ut    unde omnis iste     sit        doloremque laudantium, totam rem
perspici… natus error         voluptatem aperiam,
                              accusanti…

eaque     ab illo inventore   quasi      vitae dicta sunt explicabo. Nemo enim
ipsa quae veritatis et        architecto ipsam
                              beatae

voluptat… sit aspernatur aut  fugit, sed quia consequuntur magni dolores eos
quia      odit aut                       qui ratione
voluptas

voluptat… Neque porro         qui        quia dolor sit amet, consectetur,
sequi     quisquam est,       dolorem    adipisci
nesciunt                      ipsum

velit,    quia non numquam    tempora    labore et dolore magnam aliquam quaerat
sed       eius modi           incidunt   voluptatem. Ut
                              ut

enim ad   veniam,             ullam      laboriosam, nisi ut aliquid ex ea
minima    quis nostrum        corporis
          exercitationem      suscipit

commodi   Quis autem vel      reprehend… ea voluptate velit esse quam nihil
consequa… eum iure            qui in     molestiae

consequa… illum qui dolorem   quo        pariatur?
vel       eum fugiat          voluptas
                              nulla
--------------------------------------------------------------------------------

COLUMN A  COLUMN B            COLUMN C   COLUMN D
```

### Pandoc 'Grid' Markdown renderer

`table_gen_markdown::MarkdownGridRenderer`:

```
+---------+------------------+----------+--------------------------------------+
| COLUMN… | COLUMN B         | COLUMN C | COLUMN D                             |
+=========+==================+==========+======================================+
| Sed ut  | unde omnis iste  | sit      | doloremque laudantium, totam rem     |
| perspi… | natus error      | volupta… | aperiam,                             |
|         |                  | accusan… |                                      |
+---------+------------------+----------+--------------------------------------+
| eaque   | ab illo          | quasi    | vitae dicta sunt explicabo. Nemo     |
| ipsa    | inventore        | archite… | enim ipsam                           |
| quae    | veritatis et     | beatae   |                                      |
+---------+------------------+----------+--------------------------------------+
| volupt… | sit aspernatur   | fugit,   | quia consequuntur magni dolores eos  |
| quia    | aut odit aut     | sed      | qui ratione                          |
| volupt… |                  |          |                                      |
+---------+------------------+----------+--------------------------------------+
| volupt… | Neque porro      | qui      | quia dolor sit amet, consectetur,    |
| sequi   | quisquam est,    | dolorem  | adipisci                             |
| nesciu… |                  | ipsum    |                                      |
+---------+------------------+----------+--------------------------------------+
| velit,  | quia non numquam | tempora  | labore et dolore magnam aliquam      |
| sed     | eius modi        | incidunt | quaerat voluptatem. Ut               |
|         |                  | ut       |                                      |
+---------+------------------+----------+--------------------------------------+
| enim ad | veniam,          | ullam    | laboriosam, nisi ut aliquid ex ea    |
| minima  | quis nostrum     | corporis |                                      |
|         | exercitationem   | suscipit |                                      |
+---------+------------------+----------+--------------------------------------+
| commodi | Quis autem vel   | reprehe… | ea voluptate velit esse quam nihil   |
| conseq… | eum iure         | qui in   | molestiae                            |
+---------+------------------+----------+--------------------------------------+
| conseq… | illum qui        | quo      | pariatur?                            |
| vel     | dolorem eum      | voluptas |                                      |
|         | fugiat           | nulla    |                                      |
+---------+------------------+----------+--------------------------------------+
| COLUMN… | COLUMN B         | COLUMN C | COLUMN D                             |
```
