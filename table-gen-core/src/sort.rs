////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator row sorting module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::Collate;
use crate::ColumnDesc;
use crate::Format;
use crate::FormatRow;
use crate::Row;

// External library imports.
use bitflags::bitflags;

// Standard library imports.
use std::cmp::Ordering;
use std::ops::Bound;
use std::vec::IntoIter;


////////////////////////////////////////////////////////////////////////////////
// Sort
////////////////////////////////////////////////////////////////////////////////
/// Table cell column sorter.
pub (in crate) struct Sort<'a, R, S> {
	/// The table data source.
	inner: Format<'a, S>,
	sorted: Option<IntoIter<FormatRow<'a, R>>>,
}

impl<R, S, I> From<I> for Sort<'_, R, S>
	where
		R: Row,
		S: Iterator<Item=R>,
		I: IntoIterator<Item=R, IntoIter=S>,
{
	fn from(inner: I) -> Self {
		Sort::new(inner.into())
	}
}

impl<'a, R, S> From<Collate<'a, S>> for Sort<'a, R, S>
	where
		R: Row,
		S: Iterator<Item=R>,
{
	fn from(inner: Collate<'a, S>) -> Self {
		Sort::new(inner.into())
	}
}

impl<'a, R, S> From<Format<'a, S>> for Sort<'a, R, S>
	where
		R: Row,
		S: Iterator<Item=R>,
{
	fn from(inner: Format<'a, S>) -> Self {
		Sort::new(inner)
	}
}

impl<'a, R, S> Sort<'a, R, S>
	where
		R: Row,
		S: Iterator<Item=R>,
{
	/// Constructs a new `Sort` for the given data source.
	pub (in crate) fn new(inner: Format<'a, S>) -> Self {
		Self {
			inner,
			sorted: None,
		}
	}

	/// The row selection bounds.
	pub (in crate) fn row_selection(&self) -> &(Bound<usize>, Bound<usize>) {
		self.inner.row_selection()
	}

	/// The column output specifications.
	pub (in crate) fn column_descs(&self) -> &'a [ColumnDesc<'a>] {
		self.inner.column_descs()
	}

	/// The sort parameters for columns, in order of sort priority.
	pub (in crate) fn column_order(&self) -> &'a [ColumnOrd] {
		self.inner.column_order()
	}

	/// Compares two `FormatRow`s according to the ordering given by
	/// `[ColumnOrd]`.
	fn compare_rows(
		row_a: &FormatRow<'_, R>,
		row_b: &FormatRow<'_, R>,
		col_ord: &'_ [ColumnOrd])
		-> Ordering
	{
		let mut res = Ordering::Equal;
		for ord in col_ord {
			let text = ord.flags.contains(ColumnOrdFlags::FORMATTED);
			let rev = ord.flags.contains(ColumnOrdFlags::REVERSE);
			let nl = if ord.flags.contains(ColumnOrdFlags::NONE_LESS) {
				Ordering::Less
			} else {
				Ordering::Greater
			};
			res = if text {
				let a = row_a.text(ord.idx);
				let b = row_b.text(ord.idx);
				a.cmp(b)
			} else {
				let a = row_a.cell(ord.idx);
				let b = row_b.cell(ord.idx);
				match (a, b) {
					(None,    Some(_)) => nl,
					(Some(_), None)    => nl.reverse(),
					(Some(a), Some(b)) => a.dyn_partial_cmp(b)
						.unwrap_or(Ordering::Equal),
					_ => Ordering::Equal,
				}
			};
			if rev { res = res.reverse(); }
			if res.is_ne() { break; }
		}
		res
	}
}

impl<'a, R, S> Iterator for Sort<'a, R, S>
	where
		S: Iterator<Item=R>,
		R: Row,
{
	type Item = FormatRow<'a, R>;
	fn next(&mut self) -> Option<Self::Item> {
		let col_order = self.column_order();
		let iter = self.sorted.get_or_insert_with(|| {
			let mut rows: Vec<_> = (&mut self.inner).collect();
			rows.sort_by(|a, b| Self::compare_rows(a, b, col_order));
			rows.into_iter()
		});
		iter.next()
	}
}



////////////////////////////////////////////////////////////////////////////////
// ColumnOrd
////////////////////////////////////////////////////////////////////////////////
/// A column ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnOrd {
	/// The column index
	pub idx: usize,
	/// The column ordering flags.
	pub flags: ColumnOrdFlags,

}


bitflags! {
	/// Columns ordering flags.
	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct ColumnOrdFlags: u8 {
		/// Indicates that the column should be sorted in reverse order.
		const REVERSE    = 0b_0000_0001;
		/// Indicates that the ordering should be done on the formatted text
		/// rather than the cell values.
		const FORMATTED  = 0b_0000_0010;
		/// Indicates that empty cells in the column should sort before other
		/// values.
		const NONE_LESS = 0b_0000_0100;
	}
}
