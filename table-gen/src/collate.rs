////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator column collation module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::Cell;
use crate::ColumnDef;
use crate::ColumnOrd;
use crate::Features;
use crate::Row;

// Standard library imports.
use std::ops::RangeBounds;
use std::ops::Bound;


////////////////////////////////////////////////////////////////////////////////
// Collate
////////////////////////////////////////////////////////////////////////////////
/// Table collater. Responsible for specifiying column ordering, headers,
/// footers, and defaults.
#[allow(missing_copy_implementations)]
#[allow(missing_debug_implementations)]
pub (in crate) struct Collate<'a, S> {
	/// The table data source.
	inner: S,
	/// The table output columns, in output order.
	col_select: &'a [usize],
	/// The table output rows.
	row_select: (Bound<usize>, Bound<usize>),
	/// The column output specifications.
	column_defs: &'a [ColumnDef<'a>],
	/// The sort parameters for columns, in order of sort priority.
	col_order: &'a [ColumnOrd],
	/// The supported rendering features.
	features: Features,
}

impl<R, S, I> From<I> for Collate<'_, S>
	where
		R: Row,
		S: Iterator<Item=R>,
		I: IntoIterator<Item=R, IntoIter=S>,
{
	fn from(inner: I) -> Self {
		Collate::new(inner.into_iter())
	}
}

impl<'a, R, S> Collate<'a, S>
	where
		R: Row,
		S: Iterator<Item=R>,
{
	/// Constructs a new `Collate` for the given data source.
	#[must_use]
	pub (in crate) fn new(inner: S) -> Self {
		Self {
			inner,
			col_select: &[],
			row_select: (Bound::Included(0), Bound::Unbounded),
			column_defs: &[],
			col_order: &[],
			features: Features::default(),
		}
	}

	/// Sets the supported features and returns the `Collate`.
	#[must_use]
	pub (in crate) fn with_features(mut self, features: Features)
		-> Self
	{
		self.features = features;
		self
	}

	/// Sets the column selection and returns the `Collate`.
	#[must_use]
	pub (in crate) fn with_column_selection(mut self, col_select: &'a [usize])
		-> Self
	{
		self.col_select = col_select;
		self
	}

	/// Sets the row selection and returns the `Collate`.
	#[must_use]
	pub (in crate) fn with_row_selection<B>(mut self, row_select: B) -> Self 
		where B: RangeBounds<usize>
	{
		self.row_select = (
			row_select.start_bound().cloned(),
			row_select.end_bound().cloned());
		self
	}

	/// Sets the column descriptors and returns the `Collate`.
	#[must_use]
	pub (in crate) fn with_column_defs(
		mut self,
		column_defs: &'a [ColumnDef<'a>])
		-> Self
	{
		self.column_defs = column_defs;
		self
	}

	/// Sets the column order and returns the `Collate`.
	#[must_use]
	pub (in crate) fn with_sort_columns(mut self, col_order: &'a [ColumnOrd])
		-> Self
	{
		self.col_order = col_order;
		self
	}

	/// Returns a reference to the supported features for the renderer.
	#[must_use]
	pub (in crate) fn features(&self) -> &Features {
		&self.features
	}

	/// Returns a mutable reference to the supported features for the renderer.
	#[must_use]
	pub (in crate) fn features_mut(&mut self) -> &mut Features {
		&mut self.features
	}

	/// The row selection bounds.
	#[must_use]
	pub (in crate) fn row_selection(&self) -> &(Bound<usize>, Bound<usize>) {
		&self.row_select
	}

	/// The column output specifications.
	#[must_use]
	pub (in crate) fn column_defs(&self) -> &'a [ColumnDef<'a>] {
		self.column_defs
	}

	/// The sort parameters for columns, in order of sort priority.
	#[must_use]
	pub (in crate) fn column_order(&self) -> &'a [ColumnOrd] {
		self.col_order
	}
}


impl<'a, R, S> Iterator for Collate<'a, S>
	where
		S: Iterator<Item=R>,
		R: Row,
{
	type Item = CollateRow<'a, R>;
	fn next(&mut self) -> Option<Self::Item> {
		self.inner
			.next()
			.map(|r| CollateRow { inner: r, col_select: self.col_select, })
	}
}


////////////////////////////////////////////////////////////////////////////////
// CollateRow
////////////////////////////////////////////////////////////////////////////////
/// A single table row with collated column selection and ordering.
#[derive(Debug, Clone)]
pub (in crate) struct CollateRow<'a, R> {
	/// The row to collate.
	inner: R,
	/// The column selection and order.
	col_select: &'a [usize]
}

impl<R> Row for CollateRow<'_, R>
	where R: Row
{
	fn len(&self) -> usize {
		if self.col_select.is_empty() {
			self.inner.len()
		} else {
			self.col_select.len()
		}
	}

	fn cell(&self, col_idx: usize) -> Option<&dyn Cell> {
		if self.col_select.is_empty() {
			self.inner.cell(col_idx)
		} else {
			self.inner.cell(self.col_select[col_idx])
		}
	}
}

