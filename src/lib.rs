//! Provides `EnsuredBuffer` that impls [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html).
//!
//!

#![warn(missing_docs)]
use std::fmt;
use std::io::{self, BufRead, Read};

/// Default buffer size
///
/// Current value is 8 kiB, but may change in the future.
pub const DEFAULT_BUFFER_SIZE: usize = 8 * 1024;

/// Default ensured size.
///
/// Current value is 128 B, but may change in the future.
pub const DEFAULT_ENSURE_BYTES: usize = 128;

/// A [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html)er that ensures _ensured_ bytes in buffer.
///
/// `EnsuredBufReader` keeps _ensured_ bytes in buffer if it can read from underlying reader.
/// To fetch bytes into buffer, call `fill_buf()`.
pub struct EnsuredBufReader<R: Read> {
    inner: R,
    buf: Vec<u8>,
    pos: usize,
    cap: usize,
    ensure: usize,
}

impl<R: Read> EnsuredBufReader<R> {
    /// Creates a new `EnsuredBufReader` with a default _capacity_ (`DEFAULT_BUFFER_SIZE`) and a default _ensure_ (`DEFAULT_ENSURE_BYTES`).
    pub fn new(inner: R) -> EnsuredBufReader<R> {
        EnsuredBufReader::with_capacity_and_ensure(DEFAULT_BUFFER_SIZE, DEFAULT_ENSURE_BYTES, inner)
    }

    /// Creates a new `EnsuredBufReader` with a specified `capacity`.
    ///
    /// If specified `capacity` is smaller than `2 * DEFAULT_ENSURE_BYTES`, `capacity` will be set to `2 * DEFAULT_ENSURE_BYTES`.
    pub fn with_capacity(capacity: usize, inner: R) -> EnsuredBufReader<R> {
        if capacity < 2 * DEFAULT_ENSURE_BYTES {
            EnsuredBufReader::with_capacity_and_ensure(
                2 * DEFAULT_ENSURE_BYTES,
                DEFAULT_ENSURE_BYTES,
                inner,
            )
        } else {
            EnsuredBufReader::with_capacity_and_ensure(capacity, DEFAULT_ENSURE_BYTES, inner)
        }
    }

    /// Creates a new `EnsuredBufReader` with a specified `ensure`.
    ///
    /// If specified `ensure` is larger than `DEFAULT_ENSURE_BYTES / 2`, `capacity` will be set to `2 * ensure`.
    pub fn with_ensure(ensure: usize, inner: R) -> EnsuredBufReader<R> {
        if ensure > DEFAULT_BUFFER_SIZE / 2 {
            EnsuredBufReader::with_capacity_and_ensure(2 * ensure, ensure, inner)
        } else {
            EnsuredBufReader::with_capacity_and_ensure(DEFAULT_BUFFER_SIZE, ensure, inner)
        }
    }

    /// Creates a new `EnsuredBufReader` with a specified `capacity` and `ensure`. `capacity` must be larger than or equal to `ensure`.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is smaller than `ensure`.
    pub fn with_capacity_and_ensure(
        capacity: usize,
        ensure: usize,
        inner: R,
    ) -> EnsuredBufReader<R> {
        assert!(
            capacity >= ensure,
            "'capacity' ({}) must be larger than or equal to 'ensure' ({}).",
            capacity,
            ensure
        );
        EnsuredBufReader {
            inner,
            buf: vec![0; capacity],
            pos: 0,
            cap: 0,
            ensure,
        }
    }

    /// Returns a reference to current buffer.
    pub fn buffer(&self) -> &[u8] {
        &self.buf[self.pos..self.cap]
    }

    fn current_bytes(&self) -> usize {
        self.cap - self.pos
    }

    fn move_buf_to_head(&mut self) {
        let current_bytes = self.current_bytes();
        if self.pos > current_bytes {
            // New space does't overlap old space.
            let (ls, rs) = self.buf.split_at_mut(self.pos);
            let dist = &mut ls[0..current_bytes];
            let from = &rs[0..current_bytes];
            dist.copy_from_slice(from);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

impl<R: Read> Read for EnsuredBufReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!()
    }
}

impl<R: Read> BufRead for EnsuredBufReader<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.current_bytes() >= self.ensure {
            return Ok(self.buffer());
        }

        if self.buf.len() - self.pos < self.ensure {
            self.move_buf_to_head()
        }
        while self.current_bytes() < self.ensure {}

        Ok(self.buffer())
    }

    fn consume(&mut self, amt: usize) {
        assert!(
            amt <= self.current_bytes(),
            "the amt must be <= the number of bytes in the buffer returned by fill_buf."
        );
        self.pos += amt;
    }
}

impl<R> fmt::Debug for EnsuredBufReader<R>
where
    R: Read + fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("BufReader")
            .field("reader", &self.inner)
            .field(
                "buffer",
                &format_args!("{}/{}", self.cap - self.pos, self.buf.len()),
            )
            .finish()
    }
}
