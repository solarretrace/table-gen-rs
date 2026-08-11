////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator row and cell traits module.
////////////////////////////////////////////////////////////////////////////////

// Standard library imports.
use std::any::Any;
use std::cmp::Ordering;
use std::fmt::Display;

// External library imports.
use seq_macro::seq;


////////////////////////////////////////////////////////////////////////////////
// Cell
////////////////////////////////////////////////////////////////////////////////
/// A partially orderable, displayable, object safe interface to a single table
/// cell.
pub trait Cell: Display {
	#[must_use]
	/// Convert a cell to `Any` to enable downcasting to the base type.
	fn as_any(&self) -> &dyn Any;

	/// Perform a partial compare to another cell. This will return `None` if
	/// we are comparing cells from different columns, but this should never be
	/// done anyway.
	#[must_use]
	fn dyn_partial_cmp(&self, other: &dyn Cell) -> Option<Ordering>;
}


// Blanket impl for all PartialOrd + Display + 'static.
impl<T: PartialOrd + Display + 'static> Cell for T {
	fn as_any(&self) -> &dyn Any { self }

	fn dyn_partial_cmp(&self, other: &dyn Cell) -> Option<Ordering> {
		other.as_any()
			.downcast_ref::<T>()
			.and_then(|o| self.partial_cmp(o))
	}
}


////////////////////////////////////////////////////////////////////////////////
// Row
////////////////////////////////////////////////////////////////////////////////
/// Provides methods required for processing a single table row.
pub trait Row {
	/// Returns the number of columns in the row.
	#[must_use]
	fn len(&self) -> usize;

	/// Returns the cell at the given column index. Returns `None` if the cell
	/// is not.
	#[must_use]
	fn cell(&self, col_idx: usize) -> Option<&dyn Cell>;

	/// Returns `true` if the row contains no columns.
	#[must_use]
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}


// Fully homogeneous table rows provided via slices.
impl<C> Row for &'_ [C]
	where C: Cell
{
	fn len(&self) -> usize { <[C]>::len(self) }
	fn cell(&self, col_idx: usize) -> Option<&dyn Cell> {
		debug_assert!(col_idx < self.len());
		Some(&self[col_idx])
	}
}

impl<C> Row for [C]
	where C: Cell
{
	fn len(&self) -> usize { <[C]>::len(self) }
	fn cell(&self, col_idx: usize) -> Option<&dyn Cell> {
		debug_assert!(col_idx < self.len());
		Some(&self[col_idx])
	}
}

impl<const N: usize, C> Row for [C; N]
	where C: Cell
{
	fn len(&self) -> usize { N }
	fn cell(&self, col_idx: usize) -> Option<&dyn Cell> {
		debug_assert!(col_idx < self.len());
		Some(&self[col_idx])
	}
}


/// Generates `Row` implementations for tuples of the given length.
macro_rules! tuple_row_impl {
	($idx:literal) => {
		seq!(N in 0..$idx {
			impl<#(T~N,)*> Row for (#(T~N,)*)
				where #(T~N: Cell,)*
			{
				fn len(&self) -> usize { $idx }
				fn cell(&self, col_idx: usize) -> Option<&dyn Cell> {
					match col_idx {
						#( N => Some(&self.N), )*
						_ => panic!("invalid column index"),
					}
				}
			}
		});
	};
}

// Heterogeneous row impls provided via tuples.
tuple_row_impl!(1);
tuple_row_impl!(2);
tuple_row_impl!(3);
tuple_row_impl!(4);
tuple_row_impl!(5);
tuple_row_impl!(6);
tuple_row_impl!(7);
tuple_row_impl!(8);
tuple_row_impl!(9);
tuple_row_impl!(10);
tuple_row_impl!(11);
tuple_row_impl!(12);
tuple_row_impl!(13);
tuple_row_impl!(14);
tuple_row_impl!(15);
tuple_row_impl!(16);
