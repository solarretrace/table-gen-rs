////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator markdown renderer implementations.
////////////////////////////////////////////////////////////////////////////////

// Internal modules.
mod grid;
mod multiline;
mod pipe;
mod simple;
mod wrap;
mod ws_trim;

// Exports.
pub use grid::*;
pub use multiline::*;
pub use pipe::*;
pub use simple::*;
pub use wrap::*;
pub (in crate) use ws_trim::TrailingWsTrimWriter;
