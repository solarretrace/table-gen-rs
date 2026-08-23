////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Renderer utilities module.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::HorzAlign;

// External library imports.
use unicode_segmentation::UnicodeSegmentation as _;

// Re-exports.
pub use unicode_display_width::width;


////////////////////////////////////////////////////////////////////////////////
// unicode_display_width
////////////////////////////////////////////////////////////////////////////////
/// Calculates the width of a string according to the Unicode 15.1.0 standard.
///
/// See the [unicode-display-width crate](https://docs.rs/unicode-display-width/0.3.0/unicode_display_width/)
/// for details.
///
/// # Panics
///
/// This function will panic if the text or cell width fails to fit into a
/// `usize` value.
#[must_use]
pub fn unicode_display_width(text: &str) -> usize {
	width(text).try_into().expect("unpack string width u64 into usize")
}

////////////////////////////////////////////////////////////////////////////////
// TruncateState
////////////////////////////////////////////////////////////////////////////////
/// Indicates the manner of truncation performed adn the width of the
/// truncated text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TruncateState {
	/// The left side was truncated to the provided width.
	Left(usize),
	/// The right side was truncatedto the provided width.
	Right(usize),
	/// The left and right sides were truncated to the provided width.
	Both(usize),
	/// No truncation occurred, to text of the provided width.
	Neither(usize),
}

impl TruncateState {
	/// Returns the width of the truncated text.
	#[must_use]
	pub fn width(self) -> usize {
		use TruncateState::*;
		match self {
			Left(width)    |
			Right(width)   |
			Both(width)    |
			Neither(width) => width,
		}
	}

	/// Returns `true` if the left side was truncated.
	#[must_use]
	pub fn left_truncated(self) -> bool {
		use TruncateState::*;
		matches!(self, Left(_) | Both(_))
	}

	/// Returns `true` if the right side was truncated.
	#[must_use]
	pub fn right_truncated(self) -> bool {
		use TruncateState::*;
		matches!(self, Right(_) | Both(_))
	}
}


////////////////////////////////////////////////////////////////////////////////
// unicode_grapheme_aware_truncation
////////////////////////////////////////////////////////////////////////////////
/// Truncates strings that overflow their cell widths by cutting them
/// between unicode grapheme cluster boundaries such that the text fit within
/// the given cell width.
///
/// Returns a string and the `TruncateState` indicating the string width and
/// which sides were truncated.
///
/// # Panics
///
/// This function will panic if the text or cell width fails to fit into the
/// smaller of a `u64` or `usize` value.
#[must_use]
pub fn unicode_grapheme_aware_truncation(
	text: &str,
	text_width: usize,
	cell_width: usize,
	align: HorzAlign)
	-> (&str, TruncateState)
{
	if text_width <= cell_width {
		return (text, TruncateState::Neither(text_width));
	}
	if cell_width == 0 { return ("", TruncateState::Neither(0)); }
	
	// Repack the types to make `width` arithmetic cleaner.
	let text_width: u64 = text_width.try_into()
		.expect("pack cell width into u64");
	let cell_width: u64 = cell_width.try_into()
		.expect("pack cell width into u64");

	match align {
		HorzAlign::Left => {
			// Walk from the left, adding graphemes until the next would
			// overflow the cell.
			let mut curr_width: u64 = 0;
			let mut end: usize = 0;
			for (idx, g) in text.grapheme_indices(true) {
				let w = width(g);
				if curr_width + w > cell_width { break; }
				curr_width += w;
				end = idx;
			}

			// Compute the state.
			let curr_width: usize = curr_width.try_into()
				.expect("pack u64 into usize");
			let state = if end == text.len() {
				TruncateState::Neither(curr_width)
			} else {
				TruncateState::Right(curr_width)
			};

			(&text[..end], state)
		},

		HorzAlign::Right => {
			// Walk from the right, adding graphemes until the next would
			// overflow the cell.
			let mut curr_width: u64 = 0;
			let mut start: usize = text.len();
			for (idx, g) in text.grapheme_indices(true).rev() {
				let w = width(g);
				if curr_width + w > cell_width { break; }
				curr_width += w;
				start = idx;
			}

			// Compute the state.
			let curr_width: usize = curr_width.try_into()
				.expect("pack u64 into usize");
			let state = if start == 0 {
				TruncateState::Neither(curr_width)
			} else {
				TruncateState::Left(curr_width)
			};
			
			(&text[start..], state)
		},

		HorzAlign::Center => {
			let overflow = cell_width - text_width;
			let left_budget = overflow / 2;
			let right_budget = overflow - left_budget;

			// Walk from the left, cutting graphemes until the next would
			// overflow left_budget.
			let mut start = 0;
			let mut left_cut = 0;
			let mut iter = text.grapheme_indices(true).peekable();
			while let Some(&(idx, g)) = iter.peek() {
				let w = width(g);
				if left_cut + w > left_budget { break; }
				left_cut += w;
				start = idx + g.len();
				let _ = iter.next();
			}

			// Walk from the right, cutting graphemes until the next would
			// overflow right_budget.
			let mut end = text.len();
			let mut right_cut = 0;
			let mut iter = text[start..]
				.grapheme_indices(true).rev().peekable();
			while let Some(&(idx, g)) = iter.peek() {
				let w = width(g);
				if right_cut + w > right_budget { break; }
				right_cut += w;
				end = start + idx;
				let _ = iter.next();
			}

			// If we're still too wide, the next graphemes on either side
			// are bigger than the remaining budget.
			let mut curr_width = text_width - left_cut - right_cut;
			let mut trim_right_next = false;
			while curr_width > cell_width && end > start {
				if trim_right_next {
					if let Some((idx, g)) = text[start..end]
						.grapheme_indices(true).next_back()
					{
						end = start + idx;
						curr_width -= width(g);
					}
				} else if let Some((_, g)) = text[start..end]
					.grapheme_indices(true).next()
				{
					let g_len = g.len();
					curr_width -= width(g);
					start += g_len;
				}
				trim_right_next = !trim_right_next;
			}

			// Compute the state.
			let curr_width: usize = curr_width.try_into()
				.expect("pack u64 into usize");
			let state = match (start == 0, end == text.len()) {
				(false, false) => TruncateState::Both(curr_width),
				(true,  false) => TruncateState::Right(curr_width),
				(false, true)  => TruncateState::Left(curr_width),
				(true,  true)  => TruncateState::Neither(curr_width),
			};

			(&text[start..end], state)
		},
	}
}


////////////////////////////////////////////////////////////////////////////////
// QuantileEstimator
////////////////////////////////////////////////////////////////////////////////
/// Streaming P² quantile estimator.
#[derive(Debug, Clone)]
pub struct QuantileEstimator<const EXACT: usize> {
	/// The quantile to estimate.
	quantile: f64,
	/// A buffer for the initial observations.
	init: Vec<f64>,
	/// Actual quantile marker positions.
	n: [f64; 5],
	/// Desired quantile marker positions.
	np: [f64; 5],
	/// Differential of n per observation.
	dn: [f64; 5],
	/// Quantile marker heights (the estimated values.)
	q: [f64; 5],
}

impl<const EXACT: usize> QuantileEstimator<EXACT> {

	/// Constructs a new `QuantileEstimator` for the given quantile.
	///
	/// # Panics
	///
	/// Panics if the quantile is not within the range [`0.0`, `1.0`].
	pub fn new(quantile: f64) -> Self {
		assert!(quantile >= 0.0 && quantile <= 1.0,
			"quantile must be in (0, 1)");
		Self {
			quantile,
			init: Vec::with_capacity(EXACT),
			n: [0.0; 5],
			np: [0.0; 5],
			dn: [
				0.0,
				quantile / 2.0,
				quantile,
				(1.0 + quantile) / 2.0,
				1.0
			],
			q: [0.0; 5],
		}
	}

	/// Adjusts the estimation to account for a single observed value.
	pub fn observe(&mut self, value: f64) {
		// Buffer values if fewer than 5 observations have been made.
		if self.init.len() < EXACT {
			self.init.push(value);
			// Early exit if we're only doing degenerate min/max tracking.
			if self.quantile == 0.0 && self.init[0] > value {
				self.init[0] = value;
				return;
			}
			if self.quantile == 1.0 && self.init[0] < value {
				self.init[0] = value;
				return;
			}
			// Initialize the estimator arrays if 5 observations have been made.
			if self.init.len() == EXACT {
				self.init.sort_by(|a, b| a.partial_cmp(b).unwrap());
				self.q = <[f64; 5]>::try_from(&self.init[..])
					.expect("copy Vec into array");
				self.n = [1.0, 2.0, 3.0, 4.0, 5.0];
				self.np = [
					1.0,
					1.0 + 2.0 * self.quantile,
					1.0 + 4.0 * self.quantile,
					3.0 + 2.0 * self.quantile,
					5.0,
				];
			}
			return;
		}

		// Find the index of the cell containing the value.
		let idx = if value < self.q[0] {
			// Value is less than current minimum.
			self.q[0] = value;
			0
		} else if value >= self.q[4] {
			// Value is greater than current max.
			self.q[4] = value;
			3
		} else {
			// Value lies in one of the middle cells.
			(0..4)
				.find(|&i| self.q[i] <= value && value < self.q[i + 1])
				.unwrap()
		};

		// Increment actual positions for markers above the insertion cell.
		for i in (idx + 1)..5 {
			self.n[i] += 1.0;
		}

		// Update desired positions.
		for i in 0..5 {
			self.np[i] += self.dn[i];
		}

		// Adjust heights of the 3 interior markers.
		for i in 1..4 {
			let d = self.np[i] - self.n[i];
			let n_lo = self.n[i - 1];
			let n_hi = self.n[i + 1];
			if (d >= 1.0 && n_hi - self.n[i] > 1.0)
				|| (d <= -1.0 && n_lo - self.n[i] < -1.0)
			{
				let d = if d >= 1.0 { 1.0 } else { -1.0 };
				let qp = parabolic(
					self.n[i - 1],
					self.n[i],
					self.n[i + 1],
					self.q[i - 1],
					self.q[i],
					self.q[i + 1],
					d);
				self.q[i] = if self.q[i - 1] < qp && qp < self.q[i + 1] {
					qp
				} else {
					linear(
						self.n[i],
						self.n[i] + d,
						self.q[i],
						self.q[if d > 0.0 { i + 1 } else { i - 1 }],
						self.n[if d > 0.0 { i + 1 } else { i - 1 }])
				};
				self.n[i] += d;
			}
		}
	}

	/// Returns the current estimate of the quantile.
	///
	/// If fewer than 5 observations have been made, the quantile will be
	/// estimated by positioning within the sorted array of observations that
	/// have been made.
	pub fn estimate(&self) -> f64 {
		// Degenerate min/max value is tracked in self.init[0]:
		if self.quantile == 0.0 || self.quantile == 1.0 {
			return self.init[0];
		}

		// Do sort and positioning if we've seen fewer than 5 values:
		if self.init.len() < EXACT {
			let mut values = <[f64; EXACT]>::try_from(
					&self.init[..])
				.expect("copy Vec into array");
			values.sort_by(|a, b| a.partial_cmp(b).unwrap());
			let idx = ((values.len() as f64 - 1.0) * self.quantile)
				.round() as usize;
			return values.get(idx).copied().unwrap_or(0.0);
		}

		// Otherwise, return the estimate:
		self.q[2]
	}
}

fn parabolic(
	n_lo: f64,
	n: f64,
	n_hi: f64,
	q_lo: f64,
	q: f64,
	q_hi: f64,
	d: f64)
	-> f64
{
	q + d / (n_hi - n_lo)
		* ((n - n_lo + d) * (q_hi - q) / (n_hi - n)
			+ (n_hi - n - d) * (q - q_lo) / (n - n_lo))
}

fn linear(
	n: f64,
	n_d: f64,
	q: f64,
	q_adj: f64,
	n_adj: f64)
	-> f64
{
	q + (n_d - n) * (q_adj - q) / (n_adj - n)
}
