////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Box drawing unicode styles.
////////////////////////////////////////////////////////////////////////////////

/// The line style for a box drawn character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineStyle {
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

impl LineStyle {
    /// Returns `true` if the `LineStyle` is `Empty`.
    pub fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Returns a fallback `LineStyle` to use when a requested component is not
    /// available in the given style.
    pub fn fallback(self) -> Self {
        pub use LineStyle::*;
        match self {
            LightDash2 => Light,
            LightDash3 => Light,
            LightDash4 => Light,
            HeavyDash2 => Heavy,
            HeavyDash3 => Heavy,
            HeavyDash4 => Heavy,
            other      => other,
        }
    }

    /// Returns the `char` to use for drawing horizontal lines.
    pub fn horz(self) -> char {
        pub use LineStyle::*;
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
    pub fn vert(self) -> char {
        pub use LineStyle::*;
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
    pub fn cross(self, vert: Self) -> char {
        pub use LineStyle::*;
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
    pub fn left(self) -> char {
        pub use LineStyle::*;
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
    pub fn right(self) -> char {
        pub use LineStyle::*;
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
    pub fn top(self) -> char {
        pub use LineStyle::*;
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
    pub fn bottom(self) -> char {
        pub use LineStyle::*;
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
    pub fn corner_top_left(self, vert: Self, round: bool) -> char {
        pub use LineStyle::*;
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
    pub fn corner_top_right(self, vert: Self, round: bool) -> char {
        pub use LineStyle::*;
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
    pub fn corner_bottom_left(self, vert: Self, round: bool) -> char {
        pub use LineStyle::*;
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
    pub fn corner_bottom_right(self, vert: Self, round: bool) -> char {
        pub use LineStyle::*;
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
    pub fn vert_with_left(self, left: Self) -> char {
        pub use LineStyle::*;
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
    pub fn vert_with_right(self, right: Self) -> char {
        pub use LineStyle::*;
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
    pub fn horz_with_top(self, top: Self) -> char {
        pub use LineStyle::*;
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
    pub fn horz_with_bottom(self, bottom: Self) -> char {
        pub use LineStyle::*;
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
