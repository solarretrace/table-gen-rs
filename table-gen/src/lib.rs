////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator library.
////////////////////////////////////////////////////////////////////////////////

// Internal modules.
mod aggregate;
mod collate;
mod context;
mod diagnostic;
mod driver;
mod features;
mod format;
mod render;
mod row;
mod sort;
mod split;

// Public modules.
pub mod util;

// Internal exports.
pub (in crate) use aggregate::Aggregate;
pub (in crate) use collate::Collate;
pub (in crate) use collate::CollateRow;
pub (in crate) use format::Format;
pub (in crate) use format::FormatRow;
pub (in crate) use sort::Sort;
pub (in crate) use split::Split;
pub (in crate) use split::SplitRow;
pub (in crate) use split::TextRow;

// Public exports.
pub use collate::ColumnDef;
pub use collate::HorzAlign;
pub use collate::VertAlign;
pub use context::CellContext;
pub use context::RenderContext;
pub use diagnostic::Diagnostic;
pub use driver::Table;
pub use driver::TableBuilder;
pub use features::Features;
pub use format::DisplayFmt;
pub use format::Sign;
pub use render::Renderer;
pub use row::Cell;
pub use row::Row;
pub use sort::ColumnOrd;
