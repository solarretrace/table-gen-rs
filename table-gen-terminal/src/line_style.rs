////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Box drawing unicode styles.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::Style;

// Standard library imports.
use std::fmt::Display;


////////////////////////////////////////////////////////////////////////////////
// LineStyle
////////////////////////////////////////////////////////////////////////////////
/// The line styling for a box drawn character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineStyle {
	/// The rendered shape of the line.
	pub shape: LineShape,
	/// The ansi terminal styling to apply to the line.
	pub style: Style,
}

impl LineStyle {
	/// Constructs a new `LineStyle` with the given `LineShape` and no effects
	/// enabled.
	pub fn new(shape: LineShape) -> Self {
		Self {
			shape, 
			style: Style::new(),
		}
	}

	/// Sets the given style and returns the `LineStyle`.
	pub fn with_style(mut self, style: Style) -> Self {
		self.style = style;
		self
	}

	/// Returns `true` if the style is empty.
	#[must_use]
	pub fn is_empty(self) -> bool {
		self.shape.is_empty()
	}

	/// Returns a `Display` implementing value to use for drawing horizontal
	/// lines.
	#[must_use]
	pub fn horz(self) -> impl Display + Copy {
		LineStyleRender {
			char_fn: move || self.shape.horz(),
			style: self.style,
		}
	}

	/// Returns a `Display` implementing value to use for drawing vertical
	/// lines.
	#[must_use]
	pub fn vert(self) -> impl Display + Copy {
		LineStyleRender {
			char_fn: move || self.shape.vert(),
			style: self.style,
		}
	}

	/// Returns a `Display` implementing value to use for drawing horizontal and
	/// vertical line crossings.
	#[must_use]
	pub fn cross<S>(self, vert: S) -> impl Display + Copy 
		where S: Into<LineShape>
	{
		let vert = vert.into();
		LineStyleRender {
			char_fn: move || self.shape.cross(vert),
			style: self.style,
		}
	}

	/// Returns a `Display` implementing value to use for drawing the left part
	/// of horizontal line segments.
	#[must_use]
	pub fn left(self) -> impl Display + Copy {
		LineStyleRender {
			char_fn: move || self.shape.left(),
			style: self.style,
		}
	}

	/// Returns a `Display` implementing value to use for drawing the right part
	/// of horizontal line segments.
	#[must_use]
	pub fn right(self) -> impl Display + Copy {
		LineStyleRender {
			char_fn: move || self.shape.right(),
			style: self.style,
		}
	}

	/// Returns a `Display` implementing value to use for drawing the top part
	/// of vertical line segments.
	#[must_use]
	pub fn top(self) -> impl Display + Copy {
		LineStyleRender {
			char_fn: move || self.shape.top(),
			style: self.style,
		}
	}

	/// Returns a `Display` implementing value to use for drawing the bottom
	/// part of vertical line segments.
	#[must_use]
	pub fn bottom(self) -> impl Display + Copy {
		LineStyleRender {
			char_fn: move || self.shape.bottom(),
			style: self.style,
		}
	}

	/// Returns a `Display` implementing value to use for drawing top-left
	/// corner line intersections.
	#[must_use]
	pub fn corner_top_left<S>(self, vert: S, round: bool)
		-> impl Display + Copy
		where S: Into<LineShape>
	{
		let vert = vert.into();
		LineStyleRender {
			char_fn: move || self.shape.corner_top_left(vert, round),
			style: self.style,
		}
	}

	/// Returns a `Display` implementing value to use for drawing top-right
	/// corner line intersections.
	#[must_use]
	pub fn corner_top_right<S>(self, vert: S, round: bool)
		-> impl Display + Copy
		where S: Into<LineShape>
	{
		let vert = vert.into();
		LineStyleRender {
			char_fn: move || self.shape.corner_top_right(vert, round),
			style: self.style,
		}
	}

	/// Returns a `Display` implementing value to use for drawing bottom-left
	/// corner line intersections.
	#[must_use]
	pub fn corner_bottom_left<S>(self, vert: S, round: bool)
		-> impl Display + Copy
		where S: Into<LineShape>
	{
		let vert = vert.into();
		LineStyleRender {
			char_fn: move || self.shape.corner_bottom_left(vert, round),
			style: self.style,
		}
	}

	/// Returns a `Display` implementing value to use for drawing bottom-right
	/// corner line intersections.
	#[must_use]
	pub fn corner_bottom_right<S>(self, vert: S, round: bool)
		-> impl Display + Copy
		where S: Into<LineShape>
	{
		let vert = vert.into();
		LineStyleRender {
			char_fn: move || self.shape.corner_bottom_right(vert, round),
			style: self.style,
		}
	}

	/// Returns a `Display` implementing value to use for drawing vertical lines
	/// with left segment intersections.
	#[must_use]
	pub fn vert_with_left<S>(self, left: S) -> impl Display + Copy
		where S: Into<LineShape>
	{
		let left = left.into();
		LineStyleRender {
			char_fn: move || self.shape.vert_with_left(left),
			style: self.style,
		}
	}

	/// Returns a `Display` implementing value to use for drawing vertical
	/// lines with right segment intersections.
	#[must_use]
	pub fn vert_with_right<S>(self, right: S) -> impl Display + Copy
		where S: Into<LineShape>
	{
		let right = right.into();
		LineStyleRender {
			char_fn: move || self.shape.vert_with_right(right),
			style: self.style,
		}
	}

	/// Returns a `Display` implementing value to use for drawing horizontal
	/// lines with top segment intersections.
	#[must_use]
	pub fn horz_with_top<S>(self, top: S) -> impl Display + Copy
		where S: Into<LineShape>
	{
		let top = top.into();
		LineStyleRender {
			char_fn: move || self.shape.horz_with_top(top),
			style: self.style,
		}
	}

	/// Returns a `Display` implementing value to use for drawing horizontal
	/// lines with bottom segment intersections.
	#[must_use]
	pub fn horz_with_bottom<S>(self, bottom: S) -> impl Display + Copy
		where S: Into<LineShape>
	{
		let bottom = bottom.into();
		LineStyleRender {
			char_fn: move || self.shape.horz_with_bottom(bottom),
			style: self.style,
		}
	}
}

impl From<LineShape> for LineStyle {
	fn from(shape: LineShape) -> Self {
		Self::new(shape)
	}
}


////////////////////////////////////////////////////////////////////////////////
// LineStyle
////////////////////////////////////////////////////////////////////////////////
/// Support struct for displaying `LineStyle` values.
#[derive(Clone, Copy)]
struct LineStyleRender<F> where F: Copy + Fn() -> char {
	char_fn: F,
	style: Style
}

impl<F> Display for LineStyleRender<F>
	where F: Copy + Fn() -> char
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{:#}", self.style, (self.char_fn)(), self.style)
    }
}


////////////////////////////////////////////////////////////////////////////////
// LineShape
////////////////////////////////////////////////////////////////////////////////
/// The line shape for a box drawn character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineShape {
	/// Draw empty space.
	Empty,
	/// Draw a light line.
	Light,
	/// Draw a light double-dashed line.
	LightDash2,
	/// Draw a light triple-dashed line.
	LightDash3,
	/// Draw a light quadruple-dashed line.
	LightDash4,
	/// Draw a heavy line.
	Heavy,
	/// Draw a heavy double-dashed line.
	HeavyDash2,
	/// Draw a heavy triple-dashed line.
	HeavyDash3,
	/// Draw a heavy quadruple-dashed line.
	HeavyDash4,
	/// Draw a double line.
	Double,
}

impl LineShape {
	/// Returns `true` if the `LineShape` is `Empty`.
	#[must_use]
	pub fn is_empty(self) -> bool {
		matches!(self, Self::Empty)
	}

	/// Returns a fallback `LineShape` to use when a requested component is not
	/// available in the given shape.
	#[must_use]
	pub fn fallback(self) -> Self {
		use LineShape::*;
		match self {
			LightDash2 |
			LightDash3 |
			LightDash4 => Light,
			HeavyDash2 |
			HeavyDash3 |
			HeavyDash4 => Heavy,
			other      => other,
		}
	}

	/// Returns the `char` to use for drawing horizontal lines.
	#[must_use]
	pub fn horz(self) -> char {
		use LineShape::*;
		match self {
			Empty      => ' ',
			Light      => '─',
			LightDash2 => '╌',
			LightDash3 => '┄',
			LightDash4 => '┈',
			Heavy      => '━',
			HeavyDash2 => '╍',
			HeavyDash3 => '┅',
			HeavyDash4 => '┉',
			Double     => '═',
		}
	}

	/// Returns the `char` to use for drawing vertical lines.
	#[must_use]
	pub fn vert(self) -> char {
		use LineShape::*;
		match self {
			Empty      => ' ', 
			Light      => '│',
			LightDash2 => '╎',
			LightDash3 => '┆',
			LightDash4 => '┊',
			Heavy      => '┃',
			HeavyDash2 => '╏',
			HeavyDash3 => '┇',
			HeavyDash4 => '┋',
			Double     => '║',
		}
	}

	/// Returns the `char` to use for drawing horizontal and vertical line
	/// crossings.
	#[allow(clippy::match_same_arms)]
	#[must_use]
	pub fn cross(self, vert: Self) -> char {
		use LineShape::*;
		match (self.fallback(), vert.fallback()) {
			(Empty,  _)      => vert.vert(),
			(_,      Empty)  => self.horz(),
			(Light,  Light)  => '┼',
			(Light,  Heavy)  => '╂',
			(Light,  Double) => '╫',
			(Heavy,  Light)  => '┿',
			(Heavy,  Heavy)  => '╋',
			(Heavy,  Double) => '╬', // No Heavy+Double.
			(Double, Light)  => '╪',
			(Double, Heavy)  => '╬', // No Heavy+Double.
			(Double, Double) => '╬',
			_ => unreachable!(),
		}
	}

	/// Returns the `char` to use for drawing the left part of horizontal line
	/// segments.
	#[allow(clippy::match_same_arms)]
	#[must_use]
	pub fn left(self) -> char {
		use LineShape::*;
		match self.fallback() {
			Empty  => ' ',
			Light  => '╴',
			Heavy  => '╸',
			Double => '╸', // No Double.
			_ => unreachable!(),
		}
	}

	/// Returns the `char` to use for drawing the right part of horizontal line
	/// segments.
	#[allow(clippy::match_same_arms)]
	#[must_use]
	pub fn right(self) -> char {
		use LineShape::*;
		match self.fallback() {
			Empty  => ' ',
			Light  => '╶',
			Heavy  => '╺',
			Double => '╺', // No Double.
			_ => unreachable!(),
		}
	}

	/// Returns the `char` to use for drawing the top part of vertical line
	/// segments.
	#[allow(clippy::match_same_arms)]
	#[must_use]
	pub fn top(self) -> char {
		use LineShape::*;
		match self.fallback() {
			Empty  => ' ',
			Light  => '╵',
			Heavy  => '╹',
			Double => '╹', // No Double.
			_ => unreachable!(),
		}
	}

	/// Returns the `char` to use for drawing the bottom part of vertical line
	/// segments.
	#[allow(clippy::match_same_arms)]
	#[must_use]
	pub fn bottom(self) -> char {
		use LineShape::*;
		match self.fallback() {
			Empty  => ' ',
			Light  => '╷',
			Heavy  => '╻',
			Double => '╻', // No Double.
			_ => unreachable!(),
		}
	}

	/// Returns the `char` to use for drawing top-left corner line
	/// intersections.
	#[allow(clippy::match_same_arms)]
	#[must_use]
	pub fn corner_top_left(self, vert: Self, round: bool) -> char {
		use LineShape::*;
		match (self.fallback(), vert.fallback()) {
			(Empty,  _)      => vert.bottom(),
			(_    ,  Empty)  => vert.right(),
			(Light,  Light)  => if round { '╭' } else { '┌' },
			(Light,  Heavy)  => '┎',
			(Light,  Double) => '╓',
			(Heavy,  Light)  => '┍',
			(Heavy,  Heavy)  => '┏',
			(Heavy,  Double) => '╔', // No Heavy+Double.
			(Double, Light)  => '╒',
			(Double, Heavy)  => '┏', // No Heavy+Double.
			(Double, Double) => '╔',
			_ => unreachable!(),
		}
	}

	/// Returns the `char` to use for drawing top-right corner line
	/// intersections.
	#[allow(clippy::match_same_arms)]
	#[must_use]
	pub fn corner_top_right(self, vert: Self, round: bool) -> char {
		use LineShape::*;
		match (self.fallback(), vert.fallback()) {
			(Empty,  _)      => vert.bottom(),
			(_    ,  Empty)  => vert.left(),
			(Light,  Light)  => if round { '╮' } else { '┐' },
			(Light,  Heavy)  => '┒',
			(Light,  Double) => '╖',
			(Heavy,  Light)  => '┑',
			(Heavy,  Heavy)  => '┓',
			(Heavy,  Double) => '╗', // No Heavy+Double.
			(Double, Light)  => '╕',
			(Double, Heavy)  => '┓', // No Heavy+Double.
			(Double, Double) => '╗',
			_ => unreachable!(),
		}
	}

	/// Returns the `char` to use for drawing bottom-left corner line
	/// intersections.
	#[allow(clippy::match_same_arms)]
	#[must_use]
	pub fn corner_bottom_left(self, vert: Self, round: bool) -> char {
		use LineShape::*;
		match (self.fallback(), vert.fallback()) {
			(Empty,  _)      => vert.top(),
			(_    ,  Empty)  => vert.right(),
			(Light,  Light)  => if round { '╰' } else { '└' },
			(Light,  Heavy)  => '┖',
			(Light,  Double) => '╙',
			(Heavy,  Light)  => '┕',
			(Heavy,  Heavy)  => '┗',
			(Heavy,  Double) => '╚', // No Heavy+Double.
			(Double, Light)  => '╘',
			(Double, Heavy)  => '┗', // No Heavy+Double.
			(Double, Double) => '╚',
			_ => unreachable!(),
		}
	}

	/// Returns the `char` to use for drawing bottom-right corner line
	/// intersections.
	#[allow(clippy::match_same_arms)]
	#[must_use]
	pub fn corner_bottom_right(self, vert: Self, round: bool) -> char {
		use LineShape::*;
		match (self.fallback(), vert.fallback()) {
			(Empty,  _)      => vert.top(),
			(_    ,  Empty)  => vert.left(),
			(Light,  Light)  => if round { '╯' } else { '┘' },
			(Light,  Heavy)  => '┚',
			(Light,  Double) => '╜',
			(Heavy,  Light)  => '┙',
			(Heavy,  Heavy)  => '┛',
			(Heavy,  Double) => '╝', // No Heavy+Double.
			(Double, Light)  => '╛',
			(Double, Heavy)  => '┛', // No Heavy+Double.
			(Double, Double) => '╝',
			_ => unreachable!(),
		}
	}

	/// Returns the `char` to use for drawing vertical lines with left segment
	/// intersections.
	#[allow(clippy::match_same_arms)]
	#[must_use]
	pub fn vert_with_left(self, left: Self) -> char {
		use LineShape::*;
		match (self.fallback(), left.fallback()) {
			(Empty,  _)      => left.left(),
			(_,      Empty)  => self.vert(),
			(Light,  Light)  => '┤',
			(Light,  Heavy)  => '┥',
			(Light,  Double) => '╡',
			(Heavy,  Light)  => '┨',
			(Heavy,  Heavy)  => '┫',
			(Heavy,  Double) => '┫', // No Heavy+Double.
			(Double, Light)  => '╢',
			(Double, Heavy)  => '╣', // No Heavy+Double.
			(Double, Double) => '╣',
			_ => unreachable!(),
		}
	}

	/// Returns the `char` to use for drawing vertical lines with right segment
	/// intersections.
	#[allow(clippy::match_same_arms)]
	#[must_use]
	pub fn vert_with_right(self, right: Self) -> char {
		use LineShape::*;
		match (self.fallback(), right.fallback()) {
			(Empty,  _)      => right.right(),
			(_,      Empty)  => self.vert(),
			(Light,  Light)  => '├',
			(Light,  Heavy)  => '┝',
			(Light,  Double) => '╞',
			(Heavy,  Light)  => '┠',
			(Heavy,  Heavy)  => '┣',
			(Heavy,  Double) => '┣', // No Heavy+Double.
			(Double, Light)  => '╟',
			(Double, Heavy)  => '╠', // No Heavy+Double.
			(Double, Double) => '╠',
			_ => unreachable!(),
		}
	}

	/// Returns the `char` to use for drawing horizontal lines with top segment
	/// intersections.
	#[allow(clippy::match_same_arms)]
	#[must_use]
	pub fn horz_with_top(self, top: Self) -> char {
		use LineShape::*;
		match (self.fallback(), top.fallback()) {
			(Empty,  _)      => top.top(),
			(_,      Empty)  => self.horz(),
			(Light,  Light)  => '┴',
			(Light,  Heavy)  => '┸',
			(Light,  Double) => '╨',
			(Heavy,  Light)  => '┷',
			(Heavy,  Heavy)  => '┻',
			(Heavy,  Double) => '┻', // No Heavy+Double.
			(Double, Light)  => '╧',
			(Double, Heavy)  => '╩', // No Heavy+Double.
			(Double, Double) => '╩',
			_ => unreachable!(),
		}
	}

	/// Returns the `char` to use for drawing horizontal lines with bottom
	/// segment intersections.
	#[allow(clippy::match_same_arms)]
	#[must_use]
	pub fn horz_with_bottom(self, bottom: Self) -> char {
		use LineShape::*;
		match (self.fallback(), bottom.fallback()) {
			(Empty,  _)      => bottom.bottom(),
			(_,      Empty)  => self.horz(),
			(Light,  Light)  => '┬',
			(Light,  Heavy)  => '┰',
			(Light,  Double) => '╥',
			(Heavy,  Light)  => '┯',
			(Heavy,  Heavy)  => '┳',
			(Heavy,  Double) => '┳', // No Heavy+Double.
			(Double, Light)  => '╤',
			(Double, Heavy)  => '╦', // No Heavy+Double.
			(Double, Double) => '╦',
			_ => unreachable!(),
		}
	}
}

impl From<LineStyle> for LineShape {
	fn from(ls: LineStyle) -> Self {
		ls.shape
	}
}
