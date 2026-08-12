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
use unicode_display_width::width;


////////////////////////////////////////////////////////////////////////////////
// TruncateState
////////////////////////////////////////////////////////////////////////////////
/// Indicates the manner of truncation performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TruncateState {
    /// The left side was truncated.
    Left,
    /// The right side was truncated.
    Right,
    /// The left and right sides were truncated.
    Both,
    /// No truncation occurred.
    Neither,
}

impl TruncateState {
    /// Returns `true` if the left side was truncated.
    #[must_use]
    pub fn left_truncated(self) -> bool {
        use TruncateState::*;
        matches!(self, Left | Both)
    }

    /// Returns `true` if the right side was truncated.
    #[must_use]
    pub fn right_truncated(self) -> bool {
        use TruncateState::*;
        matches!(self, Right | Both)
    }
}


////////////////////////////////////////////////////////////////////////////////
// unicode_grapheme_aware_truncation
////////////////////////////////////////////////////////////////////////////////
/// Truncates strings that overflow their cell widths by cutting them
/// between unicode grapheme cluster boundaries such that the text fit within
/// the given cell width.
///
/// Returns a string
#[must_use]
pub fn unicode_grapheme_aware_truncation(
    text: &str,
    text_width: usize,
    cell_width: usize,
    align: HorzAlign)
    -> (&str, TruncateState)
{
    if text_width <= cell_width { return (text, TruncateState::Neither); }
    if cell_width == 0 { return ("", TruncateState::Neither); }
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
            let state = if end == text.len() {
                TruncateState::Neither
            } else {
                TruncateState::Left
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
            let state = if start == 0 {
                TruncateState::Neither
            } else {
                TruncateState::Right
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
                } else {
                    if let Some((_, g)) = text[start..end]
                        .grapheme_indices(true).next()
                    {
                        let g_len = g.len();
                        curr_width -= width(g);
                        start += g_len;
                    }
                }
                trim_right_next = !trim_right_next;
            }

            // Compute the state.
            let state = match (start == 0, end == text.len()) {
                (false, false) => TruncateState::Both,
                (true,  false) => TruncateState::Right,
                (false, true)  => TruncateState::Left,
                (true,  true)  => TruncateState::Neither,
            };

            (&text[start..end], state)
        },
    }
}
