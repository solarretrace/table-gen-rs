////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Utility crate for whitespace trimming.
////////////////////////////////////////////////////////////////////////////////

// Standard library imports.
use std::io::Result;
use std::io::Write;

/// A `Write` buffer that trims trailing whitespace by holding in a buffer until
/// a newline (CRLF or LF) is written.
pub (in crate) struct TrailingWsTrimWriter<W> 
	where W: Write
{
	/// The inner writer.
	inner: Option<W>,
	/// The buffered whitespace.
	pending: Vec<u8>,
}

impl<W> TrailingWsTrimWriter<W> 
	where W: Write
{
	/// Constructs a new `TrailingWsTrimWriter`.
	pub (in crate) fn new(inner: W) -> Self {
		Self {
			inner: Some(inner),
			pending: Vec::new(),
		}
	}

	/// Consumes the `TrailingWsTrimWriter`, returning the inner writer.
	///
	/// This will flush any pending whitspace.
	#[allow(unused)]
	pub (in crate) fn into_inner(mut self) -> Result<W> {
		self.flush_pending()?;
		Ok(self.inner.take().unwrap())
	}

	/// Writes any pending whitspace and clears the buffer.
	#[allow(unused)]
	fn flush_pending(&mut self) -> Result<()> {
		if !self.pending.is_empty() {
			self.inner.as_mut().unwrap().write_all(&self.pending)?;
			self.pending.clear();
		}
		Ok(())
	}

	/// Clears any pending whitspace in the buffer.
	pub (in crate) fn clear_pending(&mut self) {
		self.pending.clear();
	}

	/// Returns `true` if the given byte is trimmable by the writer.
	#[inline]
	const fn is_trimmable(byte: u8) -> bool {
		matches!(byte, b' ' | b'\t' | b'\r' | 0x0B | 0x0C)
	}
}

impl<W> Write for TrailingWsTrimWriter<W> 
	where W: Write
{
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		let inner = self.inner.as_mut().unwrap();
		for &b in buf {
			match b {
				b'\n' => {
					// If we have an \r\n, we want to write the last \r.
					if self.pending.last() == Some(&b'\r') {
						inner.write_all(&[b'\r'])?;
					}
					// Discard buffered whitespace.
					self.pending.clear();
					inner.write_all(&[b'\n'])?;
				}
				b if Self::is_trimmable(b) => self.pending.push(b),
				b => {
					if !self.pending.is_empty() {
						inner.write_all(&self.pending)?;
						self.pending.clear();
					}
					inner.write_all(&[b])?;
				}
			}
		}
		Ok(buf.len())
	}

	fn flush(&mut self) -> Result<()> {
		// We don't flush the pending text because we haven't seen a newline
		// yet.
		self.inner.as_mut().unwrap().flush()
	}
}

impl<W> Drop for TrailingWsTrimWriter<W> 
	where W: Write
{
	fn drop(&mut self) {
		if let Some(inner) = self.inner.as_mut() {
			// Write pending text, since we won't be seeing a newline after all.
			if let Err(e) = inner.write_all(&self.pending) {
				// Nothing reasonable to do with an error here.
				drop(e)
			}
			if let Err(e) = inner.flush() {
				// Nothing reasonable to do with an error here either.
				drop(e)
			}
		}
	}
}
