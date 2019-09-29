//! Provides `EnsuredBuffer` that impls [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html).
//!
//!

#![warn(missing_docs)]
use std::fmt;
use std::io::{self, BufRead, Read};

/// Default buffer _capacity_
///
/// Current value is 8 kiB, but may change in the future.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use ensured_bufreader::{DEFAULT_BUFFER_SIZE, EnsuredBufReader};
///
/// fn main() -> std::io::Result<()> {
///     let f = File::open("README.md")?;
///     let r = EnsuredBufReader::new(f);
///
///     assert_eq!(r.get_capacity(), DEFAULT_BUFFER_SIZE);
///     Ok(())
/// }
/// ```
pub const DEFAULT_BUFFER_SIZE: usize = 8 * 1024;

/// Default _ensured_ size.
///
/// Current value is 128 B, but may change in the future.
/// # Examples
///
/// ```
/// use std::fs::File;
/// use ensured_bufreader::{DEFAULT_ENSURED_BYTES, EnsuredBufReader};
///
/// fn main() -> std::io::Result<()> {
///     let f = File::open("README.md")?;
///     let r = EnsuredBufReader::new(f);
///
///     assert_eq!(r.get_ensured_size(), DEFAULT_ENSURED_BYTES);
///     Ok(())
/// }
/// ```
pub const DEFAULT_ENSURED_BYTES: usize = 128;

/// A [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html)er that ensures _ensured_ bytes in buffer.
///
/// `EnsuredBufReader` keeps _ensured_ bytes in buffer if it can read from underlying reader.
/// To fetch bytes into buffer, call `fill_buf()`.
pub struct EnsuredBufReader<R: Read> {
    inner: R,
    buf: Vec<u8>,
    pos: usize,
    cap: usize,
    ensured_size: usize,
}

impl<R: Read> EnsuredBufReader<R> {
    /// Creates a new `EnsuredBufReader` with a default _capacity_ (`DEFAULT_BUFFER_SIZE`) and a default _ensured_ size (`DEFAULT_ENSURED_BYTES`).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use ensured_bufreader::EnsuredBufReader;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let f = File::open("README.md")?;
    ///     let r = EnsuredBufReader::new(f);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(inner: R) -> EnsuredBufReader<R> {
        EnsuredBufReader::with_capacity_and_ensured_size(
            DEFAULT_BUFFER_SIZE,
            DEFAULT_ENSURED_BYTES,
            inner,
        )
    }

    /// Creates a new `EnsuredBufReader` with a specified minimal `min_capacity`.
    ///
    /// If specified `min_capacity` is too small, more bigger _capacity_ will be set automatically.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use ensured_bufreader::EnsuredBufReader;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let f = File::open("README.md")?;
    ///     let r = EnsuredBufReader::with_capacity(1024, f);
    ///     Ok(())
    /// }
    /// ```
    pub fn with_capacity(min_capacity: usize, inner: R) -> EnsuredBufReader<R> {
        if min_capacity < 2 * DEFAULT_ENSURED_BYTES {
            EnsuredBufReader::with_capacity_and_ensured_size(
                2 * DEFAULT_ENSURED_BYTES,
                DEFAULT_ENSURED_BYTES,
                inner,
            )
        } else {
            EnsuredBufReader::with_capacity_and_ensured_size(
                min_capacity,
                DEFAULT_ENSURED_BYTES,
                inner,
            )
        }
    }

    /// Creates a new `EnsuredBufReader` with a specified `ensured_size`.
    ///
    /// `ensured_size` must be positive.
    ///
    /// If specified `ensured_size` is larger than `DEFAULT_ENSURED_BYTES / 2`, `capacity` will be set to `2 * ensured_size`.
    ///
    /// # Panics
    ///
    /// Panics if `ensured_size` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use ensured_bufreader::EnsuredBufReader;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let f = File::open("README.md")?;
    ///     let r = EnsuredBufReader::with_ensured_size(16, f);
    ///     Ok(())
    /// }
    /// ```
    pub fn with_ensured_size(ensured_size: usize, inner: R) -> EnsuredBufReader<R> {
        if ensured_size > DEFAULT_BUFFER_SIZE / 2 {
            EnsuredBufReader::with_capacity_and_ensured_size(2 * ensured_size, ensured_size, inner)
        } else {
            EnsuredBufReader::with_capacity_and_ensured_size(
                DEFAULT_BUFFER_SIZE,
                ensured_size,
                inner,
            )
        }
    }

    /// Creates a new `EnsuredBufReader` with a specified `capacity` and `ensure`.
    ///
    /// `capacity` must be larger than or equal to `ensure`.
    /// `ensure` must be positive.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is smaller than `ensure`.
    /// Panics if `ensure` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use ensured_bufreader::EnsuredBufReader;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let f = File::open("README.md")?;
    ///     let r = EnsuredBufReader::with_capacity_and_ensured_size(1024, 32, f);
    ///     Ok(())
    /// }
    /// ```
    pub fn with_capacity_and_ensured_size(
        capacity: usize,
        ensured_size: usize,
        inner: R,
    ) -> EnsuredBufReader<R> {
        assert_ne!(ensured_size, 0, "'ensure' must be positive.");
        assert!(
            capacity >= ensured_size,
            "'capacity' ({}) must be larger than or equal to 'ensured_size' ({}).",
            capacity,
            ensured_size
        );
        EnsuredBufReader {
            inner,
            buf: vec![0; capacity],
            pos: 0,
            cap: 0,
            ensured_size,
        }
    }

    /// Returns a reference to current buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use std::io::{self, BufRead};
    /// use ensured_bufreader::EnsuredBufReader;
    ///
    /// fn main() -> io::Result<()> {
    ///     let f = File::open("README.md")?;
    ///     let mut r = EnsuredBufReader::new(f);
    ///
    ///     // Read bytes from file and consume 8 bytes.
    ///     let read_bytes = r.fill_buf()?.to_owned();
    ///     r.consume(8);
    ///     
    ///     // Get buffer.
    ///     // Current buffer should be　8 bytes shorter than `read_bytes`.
    ///     let buf = r.buffer();
    ///     assert_eq!(buf, &read_bytes[8..]);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn buffer(&self) -> &[u8] {
        &self.buf[self.pos..self.cap]
    }

    /// Try to fill buffer and return reference to buffer.
    /// The buffer filled at least `ensured_size` bytes if `EnsuredBufReader` could read from underlying reader.
    ///
    /// If `ensured_size` is larger than half of _capacity_, buffer will be extended to `2 * ensured_size`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use std::io::{self, BufRead};
    /// use ensured_bufreader::EnsuredBufReader;
    ///
    /// fn main() -> io::Result<()> {
    ///     let f = File::open("README.md")?;
    ///     let mut r = EnsuredBufReader::with_capacity_and_ensured_size(1, 1, f);
    ///
    ///     // Fill buffer.
    ///     let read_bytes = r.fill_buf_with_ensured_size(512)?;
    ///     assert!(read_bytes.len() >= 512);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn fill_buf_with_ensured_size(&mut self, ensured_size: usize) -> io::Result<&[u8]> {
        if self.current_bytes() >= ensured_size {
            return Ok(self.buffer());
        }

        if self.buf.len() < 2 * ensured_size {
            self.buf.resize(2 * ensured_size, 0);
        }
        if self.buf.len() - self.pos < ensured_size {
            self.move_buf_to_head()
        }
        while self.current_bytes() < ensured_size {
            let n = self.inner.read(&mut self.buf[self.cap..])?;
            if n == 0 {
                // Reach EOF
                break;
            }
            self.cap += n;
        }

        Ok(self.buffer())
    }

    /// Get current _capacity_ size.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use ensured_bufreader::EnsuredBufReader;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let f = File::open("README.md")?;
    ///     let r = EnsuredBufReader::new(f);
    ///
    ///     assert_eq!(r.get_capacity(), 8192);
    ///     Ok(())
    /// }
    /// ```
    pub fn get_capacity(&self) -> usize {
        self.buf.len()
    }

    /// Get current _ensured_ size.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use ensured_bufreader::EnsuredBufReader;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let f = File::open("README.md")?;
    ///     let r = EnsuredBufReader::new(f);
    ///
    ///     assert_eq!(r.get_ensured_size(), 128);
    ///     Ok(())
    /// }
    /// ```
    pub fn get_ensured_size(&self) -> usize {
        self.ensured_size
    }

    /// Returns count of bytes in buffer.
    pub fn current_bytes(&self) -> usize {
        self.cap - self.pos
    }

    fn move_buf_to_head(&mut self) {
        self.buf.copy_within(self.pos..self.cap, 0);
        self.cap -= self.pos;
        self.pos = 0;
    }
}

impl<R: Read> Read for EnsuredBufReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.fill_buf()?.read(buf)?;
        self.consume(n);
        Ok(n)
    }
}

impl<R: Read> BufRead for EnsuredBufReader<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.fill_buf_with_ensured_size(self.ensured_size)
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
