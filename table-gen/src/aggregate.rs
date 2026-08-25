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
use crate::Split;
use crate::SplitRow;
use crate::TextRow;
use crate::util::QuantileEstimator;

// Standard library imports.
use std::ops::RangeBounds as _;
use std::collections::HashSet;


////////////////////////////////////////////////////////////////////////////////
// Aggregate
////////////////////////////////////////////////////////////////////////////////
/// Table cell column aggregator.
#[derive(Debug, Clone)]
pub (in crate) struct Aggregate<'a, R> {
	/// The materialized rows of the table.
	rows: Vec<SplitRow<'a, R>>,
	/// The column output specifications.
	column_defs: &'a [ColumnDef<'a>],
    /// Function to use for calculating column widths. A `None` value means
    /// column widths should not be calculated.
	str_width_fn: Option<fn(&str) -> usize>,
	/// The column widths.
	col_widths: Vec<usize>,
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
			T: Into<Split<'a, R, S>>,
			S: Iterator<Item=R>,
			R: Row
	{
		let mut inner = inner.into();
		let extra_column_width = inner.features().extra_column_width;
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
		println!("render_contrib = {:?}", render_contrib);
		// if r_width > min_table_width {}
		// if r_width > max_table_width {}
		let min_table_width = min_table_width.saturating_sub(render_contrib);
		let max_table_width = max_table_width.saturating_sub(render_contrib);


		// Compute final column width distribution from constraints and
		// estimates.
		println!("natural {:?}", col_widths);
		let quant_widths: Vec<usize> = col_width_quantile_estimates
			.into_iter()
			.map(|e| e.estimate().round() as usize )
			.collect();
		Self::distribute_column_widths(
			&mut col_widths,
			&column_defs,
			&quant_widths,
			min_table_width,
			max_table_width,
			diagnostic_sink_fn);
		println!("distributed {:?}", col_widths);

		Self {
			rows,
			column_defs: column_defs.into_parts().1,
			str_width_fn,
			col_widths,
			header_row,
			footer_row,
		}
	}

	/// The rows of the table.
	#[must_use]
	pub (in crate) fn rows(&self) -> &[SplitRow<'a, R>] {
		&self.rows[..]
	}

	/// The column output descriptors.
	#[must_use]
	pub (in crate) fn column_defs(&self) -> &'a [ColumnDef<'a>] {
		self.column_defs
	}

	/// The Function to use for calculating column widths. A `None` value means
    /// column widths should not be calculated.
	#[must_use]
	pub (in crate) fn str_width_fn(&self) -> Option<fn(&str) -> usize> {
		self.str_width_fn
	}

	/// The column widths.
	#[must_use]
	pub (in crate) fn col_widths(&self) -> &[usize] {
		&self.col_widths
	}

	/// The header row.
	#[must_use]
	pub (in crate) fn header_row(&self) -> Option<&TextRow<'_>> {
		self.header_row.as_ref()
	}

	/// The footer row.
	#[must_use]
	pub (in crate) fn footer_row(&self) -> Option<&TextRow<'_>> {
		self.footer_row.as_ref()
	}

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
		let mut overflow = total.saturating_sub(max_table_width);

		// Table has not overflowed the max, no need to do anything.
		if overflow == 0 { return; }

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
				|idx| column_defs.min_width(idx),
				overflow);
		}
		if overflow == 0 { return; }
		println!("quant {:?}", col_widths);

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
				|idx| column_defs.min_width(idx),
				overflow);
		}
		if overflow == 0 { return; }
		println!("min {:?}", col_widths);

		(diagnostic_sink_fn)(Diagnostic::TableWidthConstraintUnsatisfied);
	}
}

/// Distrubutes column width reductions across each column by weight.
///
/// The `allocate` parameter determines the total amount of reduction to apply.
/// The `min_col_width_fn` should return the minimum width of the column indexed
/// by its argument.
fn distribute_by_weight<F>(
	widths: &mut [usize],
	weights: &[f64],
	min_col_width_fn: F,
	mut allocate: usize)
	-> usize
	where F: Fn(usize) -> usize
{
	let mut total_allocated = 0;
	let mut clamped_weight: f64 = 0.0;
	let mut clamped: HashSet<usize> = weights.iter().enumerate()
		.filter_map(|(idx, w)| (*w == 0.0).then(|| idx))
		.collect();
	while allocate > 0 {
		let weight_mult = (1.0 - clamped_weight).recip();
		let current_allocate = allocate;
		println!("allocate {:?}, clamped {:?}, widths {:?}",
			allocate, clamped, widths);
		for (idx, width) in widths.iter_mut().enumerate() {
			let fair_raw: f64 = weight_mult * weights[idx] * current_allocate as f64;
			if clamped.contains(&idx) {
				continue; 
			}
			let fair = fair_raw.round() as usize;
			println!("\t{idx} {fair} ({:.3} = {fair_raw:.3})",
				weight_mult * weights[idx]);
			
			let max = width.saturating_sub((min_col_width_fn)(idx));
			let diff = std::cmp::min(max, fair);
			*width -= diff;
			allocate -= diff;
			total_allocated += diff;
			if diff == 0 {
				clamped_weight += weights[idx];
				let _ = clamped.insert(idx);
			}
		}
		if clamped.len() == widths.len() { break; }
	}
	total_allocated
}
