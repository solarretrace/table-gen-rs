////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Text wrapping parameters.
////////////////////////////////////////////////////////////////////////////////


// External library imports.
use textwrap::Options;
pub use textwrap::LineEnding;
pub use textwrap::WrapAlgorithm;
pub use textwrap::WordSplitter;
pub use textwrap::WordSeparator;

////////////////////////////////////////////////////////////////////////////////
// WrapOptions
////////////////////////////////////////////////////////////////////////////////
/// Options for specifying the text wrapping behavior.
#[derive(Debug, Clone)]
pub struct WrapOptions {
	/// Indentation string for the first line of text.
	initial_indent: Box<str>,
	/// Indentation string for the non-first lines of text.
	subsequent_indent: Box<str>,
	/// Flag to allow breaking in the middle of words.
	break_words: bool,
	/// The algorithm to use for wrapping.
	wrap_algorithm: WrapAlgorithm,
	/// The algorithm to use for line breaking.
	word_separator: WordSeparator,
	/// The method to use for word splitting.
	word_splitter: WordSplitter,
}

impl WrapOptions {
	/// Constructs a new `WrapOptions`.
	pub fn new() -> Self {
		Self {
			initial_indent: "".to_owned().into_boxed_str(),
			subsequent_indent: "".to_owned().into_boxed_str(),
			break_words: true,
			wrap_algorithm: WrapAlgorithm::new_optimal_fit(),
			word_separator: WordSeparator::UnicodeBreakProperties,
			word_splitter: WordSplitter::HyphenSplitter,
		}
	}

	/// Sets the indentation string for the first line of text and returns the
	/// `WrapOptions`.
	#[must_use]
	pub fn with_initial_indent(mut self, initial_indent: Box<str>) -> Self {
		self.initial_indent = initial_indent;
		self
	}

	/// Sets the indentation string for the non-first lines of text and returns
	/// the `WrapOptions`.
	#[must_use]
	pub fn with_subsequent_indent(mut self, subsequent_indent: Box<str>) -> Self
	{
		self.subsequent_indent = subsequent_indent;
		self
	}

	/// Sets the flag to allow breaking in the middle of words and returns the
	/// `WrapOptions`.
	#[must_use]
	pub fn with_break_words(mut self, break_words: bool) -> Self {
		self.break_words = break_words;
		self
	}

	/// Sets the the algorithm to use for wrapping and returns the
	/// `WrapOptions`.
	#[must_use]
	pub fn with_wrap_algorithm(mut self, wrap_algorithm: WrapAlgorithm) -> Self
	{
		self.wrap_algorithm = wrap_algorithm;
		self
	}

	/// Sets the the algorithm to use for line breaking and returns the
	/// `WrapOptions`.
	#[must_use]
	pub fn with_word_separator(mut self, word_separator: WordSeparator) -> Self
	{
		self.word_separator = word_separator;
		self
	}

	/// Sets the the method to use for word splitting and returns the
	/// `WrapOptions`.
	#[must_use]
	pub fn with_word_splitter(mut self, word_splitter: WordSplitter) -> Self {
		self.word_splitter = word_splitter;
		self
	}

	pub (in crate) fn as_options(&self, width: usize) -> Options<'_> {
		Options::new(width)
			.line_ending(LineEnding::LF)
			.initial_indent(&self.initial_indent)
			.subsequent_indent(&self.subsequent_indent)
			.break_words(self.break_words)
			.wrap_algorithm(self.wrap_algorithm.clone())
			.word_separator(self.word_separator.clone())
			.word_splitter(self.word_splitter.clone())
	}
}
