////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator row sorting module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::ColumnDef;
use crate::ColumnDefs;
use crate::Diagnostic;
use crate::Row;
use crate::FormatRow;
use crate::Sort;
use crate::SplitRow;
use crate::TextRow;
use crate::util::QuantileEstimator;

// Standard library imports.
use std::collections::HashSet;
use std::ops::RangeBounds as _;
use std::rc::Rc;
use std::vec::IntoIter;


////////////////////////////////////////////////////////////////////////////////
// Aggregate
////////////////////////////////////////////////////////////////////////////////
/// Table cell column aggregator.
#[allow(missing_copy_implementations)]
#[allow(missing_debug_implementations)]
pub (in crate) struct Aggregate<'a, R> {
	/// The total number of rows in the table.
	row_count: usize,
	/// The materialized rows of the table.
	rows: IntoIter<FormatRow<'a, R>>,
	/// The column output specifications.
	column_defs: &'a [ColumnDef<'a>],
	/// Function to use for calculating column widths. A `None` value means
	/// column widths should not be calculated.
	str_width_fn: Option<fn(&str) -> usize>,
	/// The column widths.
	col_widths: Vec<usize>,
	/// Function to apply post-width processing to formatted cell text.
	late_format_fn: Option<Rc<dyn Fn(&str, usize, usize) -> String>>,
	/// The table header row.
	header_row: Option<TextRow<'a>>,
	/// The table footer row.
	footer_row: Option<TextRow<'a>>,
}

impl<'a, R> Aggregate<'a, R>
	where R: Row,
{
	/// Constructs a new `Aggregate` for the given data source.
	#[must_use]
	pub (in crate) fn new<S, T>(
		inner: T,
		column_default_def: ColumnDef<'a>,
		min_table_width: usize,
		max_table_width: usize,
		diagnostic_sink_fn: &mut (dyn FnMut(Diagnostic) + 'static))
		-> Self
		where
			T: Into<Sort<'a, R, S>>,
			S: Iterator<Item=R>,
			R: Row
	{
		let mut inner = inner.into();
		let extra_column_width = inner.features().extra_column_width;
		let late_format_fn = inner.features()
			.late_format_fn
			.as_ref()
			.map(Rc::clone);
		*inner.column_defs_mut().column_default_mut() = column_default_def;
		*inner.column_defs_mut().extra_column_width_mut() = extra_column_width;
		let str_width_fn = inner.features().str_width_fn;
		let width_contribution_fn = inner.features_mut()
			.width_contribution_fn
			.take()
			.unwrap_or_else(|| Box::new(|_| 0));
		let row_select = *inner.row_selection();

		// Build the header row.
		let mut header_used = false;
		let header_cells: Vec<&str> = inner.column_defs().columns().iter()
			.map(|column_def| column_def.header)
			.inspect(|c| header_used |= !c.is_empty())
			.collect();
		let mut header_row = header_used
			.then_some(header_cells)
			.map(TextRow::new);
		// Build the footer row.
		let mut footer_used = false;
		let footer_cells: Vec<&str> = inner.column_defs().columns().iter()
			.map(|column_def| column_def.footer)
			.inspect(|c| footer_used |= !c.is_empty())
			.collect();
		let mut footer_row = footer_used
			.then_some(footer_cells)
			.map(TextRow::new);

		let column_defs = inner.column_defs().clone();

		// Compute initial column widths from header/footer rows if available.
		let mut col_widths: Vec<usize> = match str_width_fn {
			Some(str_width) => match (header_row.as_ref(), footer_row.as_ref())
			{
				(Some(h), Some(f)) => (0..column_defs.len())
					.map(|idx| std::cmp::max(
						h.lines(idx)
							.map(str_width)
							.max()
							.unwrap_or(column_defs.min_width(idx)),
						f.lines(idx)
							.map(str_width)
							.max()
							.unwrap_or(column_defs.min_width(idx))))
					.collect(),

				(Some(r), None)    |
				(None,    Some(r)) => (0..column_defs.len())
					.map(|idx| r
						.lines(idx)
						.map(str_width)
						.max()
						.unwrap_or(column_defs.min_width(idx)))
					.collect(),

				_ => (0..column_defs.len())
					.map(|idx| column_defs.min_width(idx))
					.collect(),
			},
			None => Vec::new(),
		};

		// Prepare quantile estimators observing the current widths.
		let mut col_width_quantile_estimates = col_widths.iter().enumerate()
			.map(|(idx, w)| {
				let mut qe = QuantileEstimator::<10>::new(
					column_defs.dynamic_width_quantile(idx));
				qe.observe(f64::try_from(u32::try_from(*w)
					.unwrap_or(u32::MAX))
						.unwrap());
				qe
			})
			.collect::<Vec<_>>();

		let mut max_row_len = 0;
		let mut rows = Vec::new();
		// Do column aggregations.
		for (n, row) in inner.into_iter().enumerate() {
			// TODO: Do custom aggregation for all rows.

			// Do aggregation for output rows.
			if row_select.contains(&n) {
				// Expand table width if needed.
				max_row_len = std::cmp::max(max_row_len, row.len());

				// Expand the column widths if needed.
				if let Some(str_width) = str_width_fn {
					for idx in 0..row.len() {
						// Expand widths arrays if past the end of the 
						// header/footer.
						if idx >= col_widths.len() {
							col_widths.push(0);
						}
						if idx >= col_width_quantile_estimates.len() {
							col_width_quantile_estimates
								.push(QuantileEstimator::<10>::new(column_defs
									.dynamic_width_quantile(idx)));
						}

						if column_defs.is_fixed_width(idx) {
							// The col width is fixed, so set it.
							col_widths[idx] = column_defs.max_width(idx);
							continue;
						} 

						// The col width is dynamic. Get the width of the
						// cell.
						let cell_width = row.lines(idx)
							.map(str_width)
							.max()
							.unwrap_or(0);
						// Have the quantile estimator process the value.
						col_width_quantile_estimates[idx]
							.observe(f64::try_from(u32::try_from(cell_width)
								.unwrap_or(u32::MAX))
									.unwrap());

						// The cell width is at least as wide as the
						// min_width.
						let cell_width = std::cmp::max(
							column_defs.min_width(idx),
							cell_width);
						// If the cell widens the current width, do so, but
						// do not exceed the maximum allowed.
						col_widths[idx] = std::cmp::min(
							std::cmp::max(cell_width, col_widths[idx]),
							column_defs.max_width(idx));
					}
				}

				rows.push(row);
			}
		}
		
		// Ensure header and footer rows have enough columns to span the table.
		header_row = header_row.map(|r| r.with_len(max_row_len));
		footer_row = footer_row.map(|r| r.with_len(max_row_len));

		// Add extra column width.
		for w in col_widths.iter_mut() { *w += extra_column_width; }

		// Adjust table constraints for the renderer padding, which cannot be
		// allocated away.
		let render_contrib = (width_contribution_fn)(max_row_len);
		// println!("render_contrib = {:?}", render_contrib);
		let min_table_width = min_table_width.saturating_sub(render_contrib);
		let max_table_width = max_table_width.saturating_sub(render_contrib);


		// Compute final column width distribution from constraints and
		// estimates.
		// println!("natural {:?}", col_widths);
		let quant_widths: Vec<usize> = col_width_quantile_estimates
			.into_iter()
			.map(|e| e.estimate().round() as usize )
			.collect();
		distribute_column_widths(
			&mut col_widths,
			&column_defs,
			&quant_widths,
			min_table_width,
			max_table_width,
			diagnostic_sink_fn);
		// println!("distributed {:?}", col_widths);

		Self {
			row_count: rows.len(),
			rows: rows.into_iter(),
			column_defs: column_defs.into_parts().1,
			str_width_fn,
			col_widths,
			late_format_fn,
			header_row,
			footer_row,
		}
	}

	/// Returns the row count.
	#[must_use]
	pub (in crate) fn row_count(&self) -> usize {
		self.row_count
	}

	/// Returns an iterator over the rows of the table.
	#[must_use]
	pub (in crate) fn drain_rows(&mut self) -> RowsDrainIter<'a, R> {
		RowsDrainIter::new(
			std::mem::take(&mut self.rows),
			self.col_widths.clone(),
			self.late_format_fn.as_ref().map(Rc::clone))
	}

	/// Returns a slice of the column definitions.
	#[must_use]
	pub (in crate) fn column_defs(&self) -> &'a [ColumnDef<'a>] {
		self.column_defs
	}

	/// Returns a slice of the column widths.
	#[must_use]
	pub (in crate) fn col_widths(&self) -> &[usize] {
		&self.col_widths
	}

	/// Returns a reference to the header row.
	#[must_use]
	pub (in crate) fn header_row(&self) -> Option<&TextRow<'_>> {
		self.header_row.as_ref()
	}

	/// Returns a reference to the footer row.
	#[must_use]
	pub (in crate) fn footer_row(&self) -> Option<&TextRow<'_>> {
		self.footer_row.as_ref()
	}

	/// The Function to use for calculating column widths. A `None` value means
	/// column widths should not be calculated.
	#[must_use]
	pub (in crate) fn str_width_fn(&self) -> Option<fn(&str) -> usize> {
		self.str_width_fn
	}
}


////////////////////////////////////////////////////////////////////////////////
// RowsDrainIter
////////////////////////////////////////////////////////////////////////////////
#[allow(missing_copy_implementations)]
#[allow(missing_debug_implementations)]
pub (in crate) struct RowsDrainIter<'a, R> {
	rows: IntoIter<FormatRow<'a, R>>,
	col_widths: Vec<usize>,
	late_format_fn: Option<Rc<dyn Fn(&str, usize, usize) -> String>>,
}

impl<'a, R> RowsDrainIter<'a, R>
	where R: Row,
{
	/// Constructs a new `RowsDrainIter` over the given rows.
	pub (in crate) fn new(
		rows: IntoIter<FormatRow<'a, R>>,
		col_widths: Vec<usize>,
		late_format_fn: Option<Rc<dyn Fn(&str, usize, usize) -> String>>)
		-> Self
	{
		RowsDrainIter {
			rows,
			col_widths,
			late_format_fn,
		}
	}
}

impl<'a, R> Iterator for RowsDrainIter<'a, R>
	where R: Row,
{
	type Item = SplitRow<'a, R>;
	fn next(&mut self) -> Option<Self::Item> {
		self.rows.next().map(|row| SplitRow::new(
			row,
			&self.col_widths,
			self.late_format_fn.as_deref()))
	}
}


////////////////////////////////////////////////////////////////////////////////
// Column width distribution
////////////////////////////////////////////////////////////////////////////////
/// Distributes column widths according to table max and balancing rules.
fn distribute_column_widths(
	col_widths: &mut [usize],
	column_defs: &ColumnDefs<'_>,
	quant_widths: &[usize],
	min_table_width: usize,
	max_table_width: usize,
	diagnostic_sink_fn: &mut (dyn FnMut(Diagnostic) + 'static))
{
	// Get the total amount of space to reduce by.
	let total: usize = col_widths.iter().sum();
	let overflow = total.saturating_sub(max_table_width);
	let underflow = min_table_width.saturating_sub(total);

	// Table doesn't require adjustment.
	if overflow == 0 && underflow == 0 { return; }

	// Compute the absolute column weights.
	let weight_total: f64 = (0..col_widths.len())
		.map(|idx| {
			if column_defs.is_fixed_width(idx) {
				0.0
			} else {
				column_defs.dynamic_width_weight(idx)
			}
		})
		.sum();
	let abs_weights: Vec<f64> = (0..col_widths.len())
		.map(|idx| {
			if column_defs.is_fixed_width(idx) {
				0.0
			} else {
				column_defs.dynamic_width_weight(idx) / weight_total
			}
		})
		.collect();

	// Narrow or widen as needed.
	if overflow > 0 {
		narrow_column_widths(
			col_widths,
			column_defs,
			quant_widths,
			&abs_weights[..],
			overflow,
			diagnostic_sink_fn);
	} else {
		widen_column_widths(
			col_widths,
			column_defs,
			&abs_weights[..],
			underflow,
			diagnostic_sink_fn);
	}
}

/// Distributes column widths according to table max and balancing rules.
fn narrow_column_widths(
	col_widths: &mut [usize],
	column_defs: &ColumnDefs<'_>,
	quant_widths: &[usize],
	abs_weights: &[f64],
	mut overflow: usize,
	diagnostic_sink_fn: &mut (dyn FnMut(Diagnostic) + 'static))
{
	// Starting column widths are the 'natural widths' that hold each value.

	// 1. We reduce the column widths to their quantile widths. This will
	// prioritize shedding low-density space.

	// Compute how much total quant space we must consume.
	let total_quant: usize = col_widths.iter().enumerate()
		.map(|(idx, w)| w.saturating_sub(quant_widths[idx]))
		.sum();
	if overflow >= total_quant {
		// Do full quant reduction, since we must consume it all.
		for (idx, col_width) in col_widths.iter_mut().enumerate() {
			*col_width = quant_widths[idx]
		}
		overflow -= total_quant;
	} else {
		// Distribute `overflow` amount of width reduction by weight.
		overflow -= distribute_by_weight(
			col_widths,
			&abs_weights[..],
			overflow,
			|idx, w| w.saturating_sub(column_defs.min_width(idx)),
			|w, diff| *w -= diff);
	}
	if overflow == 0 { return; }
	// println!("quant {:?}", col_widths);

	// 2. We reduce the column widths to their min widths.

	// Compute how much total min space we must consume.
	let total_min: usize = col_widths.iter().enumerate()
		.map(|(idx, w)| w.saturating_sub(column_defs.min_width(idx)))
		.sum();
	if overflow >= total_min {
		// Do full min reduction, since we must consume it all.
		for (idx, col_width) in col_widths.iter_mut().enumerate() {
			*col_width = column_defs.min_width(idx)
		}
		overflow -= total_min;
	} else {
		// Distribute `overflow` amount of width reduction by weight.
		overflow -= distribute_by_weight(
			col_widths,
			&abs_weights[..],
			overflow,
			|idx, w| w.saturating_sub(column_defs.min_width(idx)),
			|w, diff| *w -= diff);
	}
	if overflow != 0 {
		// println!("min {:?}", col_widths);
		(diagnostic_sink_fn)(Diagnostic::TableWidthConstraintUnsatisfied);
	}
}

/// Distributes column widths according to table max and balancing rules.
fn widen_column_widths(
	col_widths: &mut [usize],
	column_defs: &ColumnDefs<'_>,
	abs_weights: &[f64],
	mut underflow: usize,
	diagnostic_sink_fn: &mut (dyn FnMut(Diagnostic) + 'static))
{
	// Starting column widths are the 'natural widths' that hold each value.

	// 1. We expand the column widths according to their weights, not to
	// exceed their max widths.

	// Distribute `underflow` amount of width expansion by weight.
	underflow -= distribute_by_weight(
		col_widths,
		&abs_weights[..],
		underflow,
		|idx, w| column_defs.max_width(idx).saturating_sub(w),
		|w, diff| *w += diff);

	if underflow != 0 {
		// println!("min {:?}", col_widths);
		(diagnostic_sink_fn)(Diagnostic::TableWidthConstraintUnsatisfied);
	}
}

/// Distrubutes column width reductions across each column by weight.
///
/// The `allocate` parameter determines the total amount of change to apply.
/// The `col_max_fn` should return the maximum change that may be applied to a
/// column. It takes the column index and its current width as arguments. The
/// `alloc_op` parameter determines how to apply the distribution: it takes a 
/// mutable reference to the current column width and the amount it should
/// change by.
fn distribute_by_weight<F, O>(
	widths: &mut [usize],
	weights: &[f64],
	mut allocate: usize,
	col_max_fn: F,
	alloc_op: O)
	-> usize
	where
		F: Fn(usize, usize) -> usize,
		O: Fn(&mut usize, usize),
{
	let mut total_allocated = 0;
	let mut clamped_weight: f64 = 0.0;
	let mut clamped: HashSet<usize> = weights.iter().enumerate()
		.filter_map(|(idx, w)| (*w == 0.0).then(|| idx))
		.collect();
	while allocate > 0 {
		let weight_mult = (1.0 - clamped_weight).recip();
		let current_allocate = allocate;
		// println!("allocate {:?}, clamped {:?}, widths {:?}",
		// 	allocate, clamped, widths);
		let mut residual: f64 = 0.0;
		for (idx, width) in widths.iter_mut().enumerate() {
			let fair_raw: f64 = weight_mult * weights[idx]
				* current_allocate as f64 + residual;
			if clamped.contains(&idx) { continue; }
			let fair = fair_raw.round() as usize;
			// println!("\t{idx} {fair} ({:.3} = {fair_raw:.3})",
			// 	weight_mult * weights[idx]);
			
			let max = (col_max_fn)(idx, *width);
			let diff = std::cmp::min(std::cmp::min(max, fair), allocate);
			alloc_op(width, diff);
			allocate -= diff;
			total_allocated += diff;
			if diff == 0 {
				clamped_weight += weights[idx];
				residual += fair_raw.fract();
				let _ = clamped.insert(idx);
			}
		}
		if clamped.len() == widths.len() { break; }
	}
	total_allocated
}
