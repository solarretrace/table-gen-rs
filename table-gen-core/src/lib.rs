////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator core library.
////////////////////////////////////////////////////////////////////////////////

// Internal modules.
mod aggregate;
mod collate;
mod driver;
mod format;
mod render;
mod row;
mod sort;
mod split;

// internal exports.
pub (in crate) use aggregate::Aggregate;
pub (in crate) use collate::Collate;
pub (in crate) use collate::CollateRow;
pub (in crate) use format::Format;
pub (in crate) use format::FormatRow;
pub (in crate) use sort::ColumnOrd;
pub (in crate) use sort::Sort;
pub (in crate) use split::Split;
pub (in crate) use split::SplitRow;
pub (in crate) use split::TextRow;

// Public exports.
pub use collate::ColumnDesc;
pub use collate::HorzAlign;
pub use collate::VertAlign;
pub use driver::Table;
pub use driver::TableBuilder;
pub use format::DisplayFmt;
pub use format::Sign;
pub use render::Features;
pub use render::Renderer;
pub use row::Cell;
pub use row::Row;
