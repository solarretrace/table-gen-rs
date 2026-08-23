////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Table generator row sorting module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::ColumnDesc;
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
	col_descs: &'a [ColumnDesc<'a>],
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
		default_col_desc: &ColumnDesc<'_>,
		max_table_width: Option<usize>)
		-> Self
		where
			T: Into<Split<'a, R, S>>,
			S: Iterator<Item=R>,
			R: Row
	{
		let inner = inner.into();
		let str_width_fn = inner.features().str_width_fn;
		let row_select = *inner.row_selection();
		let col_descs = inner.column_descs();

		// Build the header row.
		let mut header_used = false;
		let header_cells: Vec<&str> = col_descs.iter()
			.map(|col_desc| col_desc.header)
			.inspect(|c| header_used |= !c.is_empty())
			.collect();
		let mut header_row = header_used
			.then_some(header_cells)
			.map(TextRow::new);
		// Build the footer row.
		let mut footer_used = false;
		let footer_cells: Vec<&str> = col_descs.iter()
			.map(|col_desc| col_desc.footer)
			.inspect(|c| footer_used |= !c.is_empty())
			.collect();
		let mut footer_row = footer_used
			.then_some(footer_cells)
			.map(TextRow::new);

		// Compute initial column widths from header/footer rows if available.
		let mut col_widths: Vec<usize> = match str_width_fn {
			Some(str_width) => match (header_row.as_ref(), footer_row.as_ref())
			{
				(Some(h), Some(f)) => (0..col_descs.len())
					.map(|idx| std::cmp::max(
						h.lines(idx)
							.map(str_width)
							.max()
							.unwrap_or(col_descs[idx].min_width),
						f.lines(idx)
							.map(str_width)
							.max()
							.unwrap_or(col_descs[idx].min_width)))
					.collect(),

				(Some(r), None)    |
				(None,    Some(r)) => (0..col_descs.len())
					.map(|idx| r
						.lines(idx)
						.map(str_width)
						.max()
						.unwrap_or(col_descs[idx].min_width))
					.collect(),

				_ => (0..col_descs.len())
					.map(|idx| col_descs[idx].min_width)
					.collect(),
			},
			None => Vec::new(),
		};

		let mut col_width_quantile_estimates = col_widths.iter().enumerate()
			.map(|(idx, w)| {
				let col_desc = col_descs.get(idx).unwrap_or(default_col_desc);
				let mut qe = QuantileEstimator::<10>::new(
					col_desc.dynamic_width_quantile);
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
			// TODO: Do aggregation for all rows.

			// Do aggregation for output rows.
			if row_select.contains(&n) {
				// Expand table width if needed.
				max_row_len = std::cmp::max(max_row_len, row.len());

				// Expand the column widths if needed.
				if let Some(str_width) = str_width_fn {
					for idx in 0..row.len() {
						// Get the ColumnDesc for this index.
						let col_desc = col_descs
							.get(idx)
							.unwrap_or(default_col_desc);

						// Expand widths arrays if past the end of the 
						// header/footer.
						if idx >= col_widths.len() {
							col_widths.push(0);
						}
						if idx >= col_width_quantile_estimates.len() {
							col_width_quantile_estimates
								.push(QuantileEstimator::<10>::new(col_desc
									.dynamic_width_quantile));
						}

						if col_desc.is_fixed_width() {
							// The col width is fixed, so set it.
							col_widths[idx] = col_desc.max_width;
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
							col_desc.min_width,
							cell_width);
						// If the cell widens the current width, do so, but
						// do not exceed the maximum allowed.
						col_widths[idx] = std::cmp::min(
							std::cmp::max(cell_width, col_widths[idx]),
							col_desc.max_width);
					}
				}

				rows.push(row);
			}
		}
		
		header_row = header_row.map(|r| r.with_len(max_row_len));
		footer_row = footer_row.map(|r| r.with_len(max_row_len));

		// Compute final column width distribution from constraints and
		// estimates.
		let quant_widths: Vec<usize> = col_width_quantile_estimates
			.into_iter()
			.map(|e| e.estimate().round() as usize )
			.collect();
		Self::distribute_column_widths(
			&mut col_widths,
			col_descs,
			default_col_desc,
			&quant_widths,
			max_table_width);

		Self {
			rows,
			col_descs,
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
	pub (in crate) fn column_descs(&self) -> &'a [ColumnDesc<'a>] {
		self.col_descs
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
		col_descs: &[ColumnDesc<'_>],
		default_col_desc: &ColumnDesc<'_>,
		quant_widths: &[usize],
		max_table_width: Option<usize>)
	{
		let Some(max_width) = max_table_width else {
			// Reduce column widths to quantile widths if possible.
			for (idx, w) in col_widths.iter_mut().enumerate() {
				let col_desc = col_descs.get(idx).unwrap_or(default_col_desc);
				*w = col_desc.clamp_to_valid_width(quant_widths[idx]);
			}
			return;
		};

		// Get the total amount of space to reduce by.
		let total: usize = col_widths.iter().sum();
		let mut overflow = total.saturating_sub(max_width);

		// Table has not overflowed the max, no need to do anything.
		if overflow == 0 { return; }

		// First, we reduce the column widths to their quantile widths. This
		// will prioritize shedding low-density width. Second, we reduce the
		// column widths to their minimum widths. We start with some setup.

		// Compute the absolute column weights.
		let weight_total: f64 = (0..col_widths.len())
			.map(|idx| {
				let col_desc = col_descs.get(idx).unwrap_or(default_col_desc);
				if col_desc.is_fixed_width() {
					0.0
				} else {
					col_desc.dynamic_width_weight
				}
			})
			.sum();
		let abs_weights: Vec<f64> = (0..col_widths.len())
			.map(|idx| {
				let col_desc = col_descs.get(idx).unwrap_or(default_col_desc);
				if col_desc.is_fixed_width() {
					0.0
				} else {
					col_desc.dynamic_width_weight / weight_total
				}
			})
			.collect();

		// Compute how much total quant space we must consume.
		let mut total_quant: usize = col_widths.iter().enumerate()
			.map(|(idx, w)| w.saturating_sub(quant_widths[idx]))
			.sum();
		if overflow >= total_quant {
			// Do full quant reduction, since we must consume it all.
			for (idx, col_width) in col_widths.iter_mut().enumerate() {
				*col_width = quant_widths[idx]
			}
			overflow -= total_quant;
		} else {
			total_quant = overflow;
			// Use a water-fill algorithm to distribute total_quant of reduction
			// according to the abs weights.
			let mut clamped = HashSet::new();
			while total_quant > 0 {
				for (idx, col_width) in col_widths.iter_mut().enumerate() {
					if clamped.contains(&idx) { continue; }
					let fair_amt: f64 = abs_weights[idx] * total_quant as f64;
					let fair_amt = fair_amt.round() as usize;
					let max = col_width.saturating_sub(quant_widths[idx]);
					let diff = std::cmp::min(max, fair_amt);
					*col_width -= diff;
					total_quant -= diff;
					overflow -= diff;
					if diff == 0 { let _ = clamped.insert(idx); }
				}
			}
		}

		// Compute how much total min space we must consume.
		let mut total_min: usize = col_widths.iter().enumerate()
			.map(|(idx, w)| {
				let col_desc = col_descs.get(idx).unwrap_or(default_col_desc);
				w.saturating_sub(col_desc.min_width)
			})
			.sum();
		if overflow >= total_min {
			// Do full min reduction, since we must consume it all.
			for (idx, col_width) in col_widths.iter_mut().enumerate() {
				let col_desc = col_descs.get(idx).unwrap_or(default_col_desc);
				*col_width = col_desc.min_width
			}
			overflow -= total_min;
		} else {
			total_min = overflow;
			// Use a water-fill algorithm to distribute total_min of reduction
			// according to the abs weights.
			let mut clamped = HashSet::new();
			while total_min > 0 {
				for (idx, col_width) in col_widths.iter_mut().enumerate() {
					if clamped.contains(&idx) { continue; }
					let col_desc = col_descs.get(idx)
						.unwrap_or(default_col_desc);
					let fair_amt: f64 = abs_weights[idx] * total_min as f64;
					let fair_amt = fair_amt.round() as usize;
					let max = col_width.saturating_sub(col_desc.min_width);
					let diff = std::cmp::min(max, fair_amt);
					*col_width -= diff;
					total_min -= diff;
					overflow -= diff;
					if diff == 0 { let _ = clamped.insert(idx); }
				}
			}
		}

		if overflow > 0 {
			println!("table width contraint not satisfied");
		}
	}
}

