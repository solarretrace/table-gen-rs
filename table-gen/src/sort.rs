////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator row sorting module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::Collate;
use crate::ColumnDef;
use crate::Features;
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
#[allow(missing_copy_implementations)]
#[allow(missing_debug_implementations)]
pub (in crate) struct Sort<'a, R, S> {
	/// The table data source.
	inner: Format<'a, S>,
	/// An iterator over the sorted rows of the table. Populated on first row
	/// access.
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
	#[must_use]
	pub (in crate) fn new(inner: Format<'a, S>) -> Self {
		Self {
			inner,
			sorted: None,
		}
	}

	/// Returns a reference to the supported features for the renderer.
	#[must_use]
	pub (in crate) fn features(&self) -> &Features {
		self.inner.features()
	}

	/// Returns a mutable reference to the supported features for the renderer.
	#[must_use]
	pub (in crate) fn features_mut(&mut self) -> &mut Features {
		self.inner.features_mut()
	}
	
	/// The row selection bounds.
	#[must_use]
	pub (in crate) fn row_selection(&self) -> &(Bound<usize>, Bound<usize>) {
		self.inner.row_selection()
	}

	/// The column output specifications.
	#[must_use]
	pub (in crate) fn column_defs(&self) -> &'a [ColumnDef<'a>] {
		self.inner.column_defs()
	}

	/// The sort parameters for columns, in order of sort priority.
	#[must_use]
	pub (in crate) fn column_order(&self) -> &'a [ColumnOrd] {
		self.inner.column_order()
	}

	/// Compares two `FormatRow`s according to the ordering given by
	/// `[ColumnOrd]`.
	#[must_use]
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

impl ColumnOrd {
	/// Constructs a new `ColumnOrd` ordering on the given column index.
	#[must_use]
	pub fn new(idx: usize) -> Self {
		Self {
			idx,
			flags: ColumnOrdFlags::default(),
		}
	}

	/// Toggles the sort order and returns the `ColumnOrd`.
	#[must_use]
	pub fn with_reversed_order(mut self) -> Self {
		self.flags.toggle(ColumnOrdFlags::REVERSE);
		self
	}

	/// Sets a flag indicating to order by the formatted column text and returns
	/// the `ColumnOrd`.
	#[must_use]
	pub fn with_formatted_order(mut self) -> Self {
		self.flags.set(ColumnOrdFlags::FORMATTED, true);
		self
	}

	/// Sets a flag indicating to order `None` values before all other values
	/// and returns the `ColumnOrd`.
	#[must_use]
	pub fn with_none_lt_order(mut self) -> Self {
		self.flags.set(ColumnOrdFlags::NONE_LESS, true);
		self
	}
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

impl Default for ColumnOrdFlags {
	fn default() -> Self {
		Self::empty()
	}
}
